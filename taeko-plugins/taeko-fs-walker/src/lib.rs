#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
//!
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Default)]
pub struct FSWalker;

pub type WalkContents = (String, Arc<String>);

impl FSWalker {
    pub fn walk<P>(&mut self, file_types: &[&str], root: P) -> Vec<WalkContents>
    where
        P: AsRef<Path>,
    {
        WalkDir::new(root.as_ref().canonicalize().unwrap())
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                if e.path().is_file() {
                    let extension = e.path().extension().unwrap().to_str().unwrap();
                    file_types.iter().any(|w| w == &extension)
                } else {
                    false
                }
            })
            .map(|entry| {
                (
                    entry.path().to_str().unwrap().to_string(),
                    Arc::new(
                        std::fs::read_to_string(entry.path().to_str().unwrap().to_string())
                            .unwrap(),
                    ),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut walker = FSWalker::default();
        let contents = walker.walk(&["md"], "./test_content/");
        assert_eq!(12, contents.len());
    }
}
