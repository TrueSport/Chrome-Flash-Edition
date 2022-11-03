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
    /// and the limit is the maximum length of a line, regardless of prefixes.
    pub fn new(
        buf: &'a mut Buffer, range: Range, limit: usize
    ) -> std::result::Result<Self, Error> {
        let text = buf.read(&range).ok_or("Selection is invalid.")?;
        Ok(Self { buf, range, text, limit })
    }

    pub fn apply(mut self) -> std::result::Result<(), Error> {
        let prefix = self.infer_pr