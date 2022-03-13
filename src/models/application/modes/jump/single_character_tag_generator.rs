// The upper limit on one-letter values ("z").
const TAG_INDEX_LIMIT: u8 = 122;

pub struct SingleCharacterTagGenerator {
    index: u8,
}

impl SingleCharacterTagGenerator {
    pub fn new() -> SingleCharacterTagGenerator {
        SingleCharacterTagGenerator{ index: 96 }
    }

    /// Restarts the tag generator sequence.
    pub fn reset(&mut self) {
        self.index = 96;
    }
}

impl Iterator for SingleCharacterTagGenerator {
    type Ite