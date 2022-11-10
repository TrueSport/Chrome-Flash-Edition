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
        let mut pars = text.split("\n\n").peekable();

        let mut space_delims = ["".to_string(), " ".to_string(), "\n".to_string()];
        if prefix != "" {
        	space_delims[0] += prefix;
        	space_delims[0] += " ";
        	space_delims[2] += prefix;
        	space_delims[2] += " ";
        	limit -= prefix.len() + 1;
        }

        while let Some(par) = pars.next() {
        	let mut words = par.split_whitespace();
        	let mut len = 0;
        	let mut first = true;

        	while let Some(word) = words.next() {
        	    if word == prefix {
        		continue;
        	    }

        	    len += word.len();

        	    let over = len > limit;
        	    let u_over = over as usize;
        	    let idx = (!first as usize) * u_over + !first as usize;

        	    justified += &space_delims[idx];
        	    justified += word;

        	    // if we're over, set the length to 0, otherwise increment it
        	    // properly. This just does that mith multiplication by 0 instead of
        	    // branching.
        	    len = (len + 1) * (1 - u_over) + (word.len() + 1) * u_over;
        	    first = false;
        	}

        	if pars.peek().is_some() {
        	    justified += "\n\n"; // add back the paragraph break.
       