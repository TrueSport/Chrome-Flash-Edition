// The upper limit on two-letter values ("zz").
const TAG_INDEX_LIMIT: u16 = 675;

pub struct TagGenerator {
    index: u16,
}

impl Tag