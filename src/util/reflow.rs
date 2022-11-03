use super::*;

/// Encapsulate reflow logic for buffer manipulation.
pub struct Reflow<'a> {
    buf: &'a mut Buffer,
    range: Range,
    text: String,
    limit: usize,
}

impl<'a> Reflow<'a> {
    /// Create a reflow instance, where buffer and range determine the target,
    /// 