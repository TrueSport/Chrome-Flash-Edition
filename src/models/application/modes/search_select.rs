use std::fmt::Display;
use std::slice::Iter;

#[derive(Clone)]
pub struct SearchSelectConfig {
    pub max_results: usize,
}

impl Default for SearchSelectConfig {
    fn default() -> SearchSelectConfig {
        SearchSelectConfig {
            max_results: 5,
        }
    }
}

/// This trait will become vastly simpler if/when fields are added to traits.
/// See: https://github.com/rust-lang/rfcs/pull/154