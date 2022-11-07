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
        let prefix = self.infer_prefix()?;
        let jtxt = self.justify_str(&prefix);
        self.buf.delete_range(self.range.clone());
        self.buf.cursor.move_to(self.range.start());
        self.buf.insert(jtxt);

        Ok(())
    }

    fn infer_prefix(&self) -> std::result::Result<String, Error> {
        match self.text.split_whitespace().next() {
        	Some(n) => if n.chars().next().unwrap().is_alphanumeric() {
        	    Ok("".to_string())
        	} else {
        	    Ok(n.to_string())
        	},
        	None => bail!("Selection is empty."),
        }
    }


    fn justify_str(&mut self, prefix: &str) -> String {
        let text = self.buf.read(&self.range).unwrap();
        let mut limit = self.limit;
        let mut justified = String::with_capacity(text.len());
        let mut pars = text.split("\n\n").p