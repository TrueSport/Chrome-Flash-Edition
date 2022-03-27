use std::fmt;

pub struct PathMode {
    pub input: String,
    pub save_on_accept: bool,
}

impl PathMode {
    pub fn new(initial_path: String) -> PathMode {
        PathMode {
            input: initi