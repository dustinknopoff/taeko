#![warn(missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::all)]
//!
use frontmatter::split_matter;

use pulldown_cmark::{html, Parser};
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, fs::File, io::BufReader, pin::Pin};

#[salsa::query_group(FSDatabaseStorage)]
pub trait FSDatabase: salsa::Database {
    #[salsa::input]
    fn text(&self, name: String) -> Arc<String>;

    #[salsa::input]
    fn blob(&self, name: String) -> Arc<&'static [u8]>;
}

pub trait Deserializeable {
    fn deserialize<T>(&self, path: String) -> Result<T, ()>
    where
        T: serde::de::DeserializeOwned;
}

mod md {
    use super::*;
    #[salsa::query_group(MarkdownDatabaseStorage)]
    pub trait MarkdownDatabase: FSDatabase {
        fn frontmatter(&self, path: String) -> Arc<Option<String>>;

        fn content(&self, path: String) -> Arc<String>;

        fn as_html(&self, path: String) -> Arc<String>;
    }

    pub fn frontmatter(db: &dyn MarkdownDatabase, path: String) -> Arc<Option<String>> {
        let contents = db.text(path);
        if let Ok((front, _)) = split_matter(&contents) {
            Arc::new(Some(front))
        } else {
            Arc::new(None)
        }
    }

    pub fn content(db: &dyn MarkdownDatabase, path: String) -> Arc<String> {
        let contents = db.text(path);
        if let Ok((_, content)) = split_matter(&contents) {
            Arc::new(content)
        } else {
            contents
        }
    }

    pub fn as_html(db: &dyn MarkdownDatabase, path: String) -> Arc<String> {
        let contents = db.text(path);
        let content = if let Ok((_, content)) = split_matter(&contents) {
            Arc::new(content)
        } else {
            contents
        };
        let parser = Parser::new(&content);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Arc::new(html_output)
    }

    #[salsa::database(FSDatabaseStorage, MarkdownDatabaseStorage)]
    #[derive(Default)]
    pub struct MarkdownDatabaseStruct {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for MarkdownDatabaseStruct {}

    impl super::Deserializeable for MarkdownDatabaseStruct {
        fn deserialize<T>(&self, path: String) -> Result<T, ()>
        where
            T: serde::de::DeserializeOwned,
        {
            let contents = self.text(path);
            if let Ok((front, _)) = split_matter(&contents) {
                let content: Result<T, serde_yaml::Error> = serde_yaml::from_str(&front);
                return match content {
                    Err(_) => Err(()),
                    Ok(val) => Ok(val),
                };
            }
            Err(())
        }
    }
}

// Flow of ownership:
// Register a plugin -> at the correct time in pipeline, run it, (not dropping) -> get database contents and render -> write to files?
// Every time after:
// at correct time in pipeline, run again (without redoing unneccessary work) -> get database contents and render -> write to files...rinse/repeat
// Context trait -> Salsa Databases Structures -> impl of differing Salsa Databases -> render methods

// ................
// MarkdownDatabaseStruct needs to be saved somewhere and then mutably borrowed and called in the `dyn Fn()`

// Something like `addSource(Sized)` which would hold on to `MarkdownDatabaseStruct`
// then `addPlugin(dyn Fn())`
// first run does addSource/addPlugin.
// n-runs afterwards `dyn Fn()` is called

#[salsa::database(FSDatabaseStorage, md::MarkdownDatabaseStorage)]
struct Context {
    storage: salsa::Storage<Self>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            storage: Default::default(),
        }
    }

    async fn get_json_txt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let resp = reqwest::get("https://httpbin.org/ip").await?.text().await?;
        self.set_text(String::from("IP"), Arc::new(resp));
        Ok(())
    }

    fn get_as_json(&mut self) -> HashMap<String, String> {
        serde_json::from_str(&self.text(String::from("IP"))).unwrap()
    }
}

impl salsa::Database for Context {}

// Here's the problem...
// High level:
// We essentially want to be able to fetch any data source an infinite number of times.
// But, any dependent actions (or systems) should only re-run if the fetched data has changed.
// Where this gets wonky is that there may be an infinite number of data sources (although infinite is near impossible)
// ECS allows us to not care about differences between data sources but makes determining uniqueness difficult.
// Salsa allows us to only re-execute functions when input data has changed.
// Both want to be the root of a program.

// ----------------------------------------------------------------
// 1. Fetch data -> db.set_file... define an entity_id with a component containing path and contents
// 2. Run pure functions based on input

// What about...Instead of ^^
// You have some Context struct which has all the impls for plugins,

// OK, Ok,OKKK
// Here's the deal.
// There's a base salsa DB. It can take in text or a blob.
// Markdown DB is on top of that only using text() from base
// A Web trait on that expects to be impl'd on something with a Storage...possible using async-trait
// Layer on top for deserializing text to JSON, layer => TOML, etc.

#[cfg(test)]
mod tests {
    use super::*;
    use md::*;
    #[test]
    fn test() {
        let mut context = Context::new();
        context.set_text(
            String::from("README.md"),
            Arc::new(String::from(
                r#"---
name: Dustin
---
# Hello World"#,
            )),
        );
        dbg!(context.frontmatter(String::from("README.md")));
        dbg!(context.as_html(String::from("README.md")));
        println!("Hello, world!");
        panic!();
    }

    #[tokio::test]
    async fn r_test() {
        let mut context = Context::new();
        context.set_text(
            String::from("README.md"),
            Arc::new(String::from(
                r#"---
name: Dustin
---
# Hello World"#,
            )),
        );
        context.get_json_txt().await.unwrap();
        dbg!(context.frontmatter(String::from("README.md")));
        dbg!(context.as_html(String::from("README.md")));
        dbg!(context.get_as_json());
        println!("Hello, world!");
        panic!();
    }
}
