#![warn(missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::all)]
//!
use frontmatter::split_matter;
use pulldown_cmark::{html, Parser};
use std::{path::Path, sync::Arc};
use taeko_core::{salsa, TaekoCoreDatabase};

/// This is a 'database' for querying content that is markdown
/// To retrieve it's frontmatter, content, or convert in to HTML
// TODO: Convert all fns to Result using taeko-core Error type
#[salsa::query_group(MarkdownDatabaseStorage)]
pub trait MarkdownDatabase: TaekoCoreDatabase {
    fn frontmatter(&self, path: String) -> Arc<Option<String>>;

    fn content(&self, path: String) -> Arc<String>;

    fn as_html(&self, path: String) -> Arc<String>;
}

pub fn frontmatter(db: &dyn MarkdownDatabase, path: String) -> Arc<Option<String>> {
    assert_eq!("md", Path::new(&path).extension().unwrap());
    let contents = db.text(path);
    if let Ok((front, _)) = split_matter(&contents) {
        Arc::new(Some(front))
    } else {
        Arc::new(None)
    }
}

pub fn content(db: &dyn MarkdownDatabase, path: String) -> Arc<String> {
    assert_eq!("md", Path::new(&path).extension().unwrap());
    let contents = db.text(path);
    if let Ok((_, content)) = split_matter(&contents) {
        Arc::new(content)
    } else {
        contents
    }
}

pub fn as_html(db: &dyn MarkdownDatabase, path: String) -> Arc<String> {
    assert_eq!("md", Path::new(&path).extension().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Arc;
    use taeko_core::{salsa, TaekoCoreDatabase, TaekoCoreDatabaseStorage};
    use taeko_fs_walker::FSWalker;

    #[salsa::database(TaekoCoreDatabaseStorage, MarkdownDatabaseStorage)]
    #[derive(Default)]
    struct MarkdownFinder {
        storage: salsa::Storage<Self>,
        walker: FSWalker,
    }

    impl salsa::Database for MarkdownFinder {}

    impl MarkdownFinder {
        pub fn walk<P>(&mut self, file_types: &[&str], root: P)
        where
            P: AsRef<Path>,
        {
            let files = self.walker.walk(file_types, root);
            files.into_iter().for_each(|(path, contents)| {
                self.set_text(path, contents);
            });
        }
    }

    #[test]
    fn test() {
        let mut mf = MarkdownFinder::default();
        mf.walk(&["md"], "../taeko-fs-walker/test_content/");
        let path = PathBuf::from("../taeko-fs-walker/test_content/11.md")
            .canonicalize()
            .unwrap();
        let path = path.to_str().unwrap();
        assert_eq!(
            Arc::new("---\nname: Dustin\n---\n# Hello World\n".to_string()),
            mf.text(path.to_string())
        );
        assert_eq!(
            Arc::new(Some(String::from("name: Dustin\n"))),
            mf.frontmatter(path.to_string())
        );
        assert_eq!(
            Arc::new(String::from("# Hello World\n")),
            mf.content(path.to_string())
        );
        assert_eq!(
            Arc::new(String::from("<h1>Hello World</h1>\n")),
            mf.as_html(path.to_string())
        );
    }
}
