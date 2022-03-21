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
        if self.index > TAG_INDEX_LIMIT {
            return None;
        }

        // Calculate the tag characters based on the index value.
        let first_letter = ((self.index / 26) + 97) as u8;
        let second_letter = ((self.index % 26) + 97) as u8;

        // Increment the index.
        self.index += 1;

        // Stitch the two calculated letters together.
        match String::from_utf8(vec![first_letter, second_letter]) {
            Ok(tag) => Some(tag),
            Err(_) => panic!("Couldn't generate a valid UTF-8 jump mode tag."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagGenerator;

    #[test]
    fn next_returns_sequential_letters_of_the_alphabet() {
        let mut generator = TagGenerator::new();
        assert_eq!(generator.next().unwrap(), "aa");
        as