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
/// See: https://github.com/rust-lang/rfcs/pull/1546
pub trait SearchSelectMode<T: Display>: Display {
    fn query(&mut self) -> &mut String;
    fn search(&mut self);
    fn insert_mode(&self) -> bool;
    fn set_insert_mode(&mut self, insert_mode: bool);
    fn results(&self) -> Iter<T>;
    fn selection(&self) -> Option<&T>;
    fn selected_index(&self) -> usize;
    fn select_previous(&mut self);
    fn select_next(&mut self);
    fn config(&self) -> &SearchSelectConfig;
    fn message(&mut self) -> Option<String> {
        if self.query().is_empty() {
            Some(String::from("Enter a search query to start."))
        } else if self.results().count() == 0 {
            Some(String::from("No matching entries found."))
        } else {
            None
        }
    }

    fn push_search_char(&mut self, c: char) {
        self.query().push(c);
    