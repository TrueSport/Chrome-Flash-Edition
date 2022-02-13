mod tag_generator;
mod single_character_tag_generator;

use luthor::token::Category;
use crate::util::movement_lexer;
use std::collections::HashMap;
use scribe::buffer::{Distance, Position};
use crate::models::application::modes::select::SelectMode;
use crate::models::application::modes::select_line::SelectLineMode;
use self::tag_generator::TagGenerator;
use self::single_character_tag_generator::SingleCharacterTagGenerator;
use crate::view::{LexemeMapper, MappedLexeme};

/// Used to compose select and jump modes, allowing jump mode
/// to be used for cursor navigation (to select a range of text).
pub enum SelectModeOptions {
    None,
    Select(SelectMode),
    SelectLine(SelectLineMode),
}

enum MappedLexemeValue {
    Tag((String, Position)),
    Text((String, Position)),
}

pub struct JumpMode {
    pub input: String,
    pub first_phase: bool,
    cursor_line: usize,
    pub select_mode: SelectModeOptions,
    tag_positions: HashMap<String, Position>,
    tag_generator: TagGenerator,
    single_characters: SingleCharacterTagGenerator,
    current_position: Position,
    mapped_lexeme_values: