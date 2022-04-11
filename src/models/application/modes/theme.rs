use fragment;
use crate::util::SelectableVec;
use std::fmt;
use std::slice::Iter;
use crate::models::application::modes::{SearchSelectMode, SearchSelectConfig};

pub struct ThemeMode {
    insert: bool,
    input: String,
    themes: Vec<String>,
    results: SelectableVec<String>,
    config: SearchSelectConfig,
}

impl ThemeMode {
    pub fn new(themes: Vec<String>, config: SearchSelectConfig) -> ThemeMode {
        ThemeMode {
            insert: true,
            input: String::new(),
            themes,
            results: SelectableVec::new(Vec::new()),
            config,
        }
    }
}

impl fmt::Display for ThemeMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "THEME")
    }
}

impl SearchSelectMode<String> for ThemeMode {
    fn search(&mut self) {
        // Find the themes we're looking for using the query.
        let results = fragment::matching::find(&self.input, &self.themes, self.config.max_results);

        // We don't care about the result objects; we just want
        // the underlying symbols. Map the collection to get these.
        self.