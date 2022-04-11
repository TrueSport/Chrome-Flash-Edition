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
 