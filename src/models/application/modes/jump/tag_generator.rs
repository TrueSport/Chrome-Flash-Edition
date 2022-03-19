// The upper limit on two-letter values ("zz").
const TAG_INDEX_LIMIT: u16 = 675;

pub struct TagGenerator {
    index: u16,
}

impl TagGenerator {
    /// Builds a new zero-indexed tag generator.
    pub fn new() -> TagGenerator {
        TagGenerator { index: 0 }
    }

    /// Restarts the tag generator sequence.
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl Iterator for TagGenerator {
    type Item = String;

    // Returns the next two-letter tag, or none
    // if we've passed the limit ("zz").
    fn next(&mut self) -> Option<String> {