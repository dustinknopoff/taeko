#![warn(missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::all)]
//!
use std::sync::Arc;

pub use salsa;
/// This is the 'root' Database of a Taeko Pipeline
/// All other plugins/elements build on top of this single database
/// This allows us to only re-do work when an input text or blob changes
#[salsa::query_group(TaekoCoreDatabaseStorage)]
pub trait TaekoCoreDatabase: salsa::Database {
    #[salsa::input]
    fn text(&self, name: String) -> Arc<String>;

    #[salsa::input]
    fn blob(&self, name: String) -> Arc<&'static [u8]>;
}

// TODO: Create an error type
