use fragment;
use crate::util::SelectableVec;
use std::fmt;
use std::slice::Iter;
use crate::models::application::modes::{SearchSelectMode, SearchSelectConfig};

pub struct ThemeMode {
    insert: bool,
    input: String,
    themes: Vec<String>,
    results: Selecta