use lazy_static::lazy_static;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use taeko::taeko_core::*;
use taeko::{taeko_fs_walker::*, taeko_markdown_transformer::*};
use tera::{Context, Tera};

#[salsa::query_group(TeraDatabaseStorage)]
trait TeraDatabase: TaekoCoreDatabase {
    fn rendered(&self, name: String) -> Arc<String>;
}

fn rendered(db: &dyn TeraDatabase, name: String) -> Arc<String> {
    db.text(name)
}

enum TaekoSections {
    Home,
    About,
    Contact,
}

#[salsa::database(TaekoCoreDatabaseStorage, MarkdownDatabaseStorage, TeraDatabaseStorage)]
struct SampleWebsiteContext {
    storage: salsa::Storage<Self>,
    sections: TaekoSections,
    walker: FSWalker,
    name: String,
    description: String,
    md_content: Vec<String>,
    templating: Tera,
}

impl salsa::Database for SampleWebsiteContext {}

impl SampleWebsiteContext {
    pub fn new(name: String, description: String) -> Self {
        let mut tera = match Tera::new("templates/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        tera.autoescape_on(vec!["html"]);
        Self {
            storage: Default::default(),
            sections: TaekoSections::Home,
            walker: FSWalker,
            name,
            description,
            md_content: Vec::new(),
            templating: tera,
        }
    }

    pub fn walk<P>(&mut self, file_types: &[&str], root: P)
    where
        P: AsRef<Path>,
    {
        let files = self.walker.walk(file_types, root);
        files.into_iter().for_each(|(path, contents)| {
            self.set_text(path.clone(), contents);
            self.md_content.push(path);
        });
    }
}

fn main() {
    // Make a pipeline...
    // Build a context with site-wide metadata.
    // Define page names
    // Gather markdown
    // Build pages using above data
    // Write to disk
    // Add a watcher for files added/

    // 1. Create a Context containing site-wide metadata
    let mut ctx = SampleWebsiteContext::new(
        String::from("Sample"),
        String::from("An example website built using taeko"),
    );
    ctx.walk(&["md"], "./content");
    let mut context = Context::new();
    context.insert("name", &ctx.name);
    context.insert("description", &ctx.description);
    ctx.set_rendered(
        String::from("Home"),
        Arc::new(ctx.templating.render("index.html", &context).unwrap()),
    );
}
