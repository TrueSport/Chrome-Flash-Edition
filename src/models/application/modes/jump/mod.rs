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
    mapped_lexeme_values: Vec<MappedLexemeValue>,
}

impl JumpMode {
    pub fn new(cursor_line: usize) -> JumpMode {
        JumpMode {
            input: String::new(),
            first_phase: true,
            cursor_line,
            select_mode: SelectModeOptions::None,
            tag_positions: HashMap::new(),
            tag_generator: TagGenerator::new(),
            single_characters: SingleCharacterTagGenerator::new(),
            current_position: Position{ line: 0, offset: 0 },
            mapped_lexeme_values: Vec::new(),
        }
    }

    pub fn map_tag(&self, tag: &str) -> Option<&Position> {
        self.tag_positions.get(tag)
    }

    pub fn reset_display(&mut self) {
        self.tag_positions.clear();
        self.tag_generator.reset();
        self.single_characters.reset();
    }
}

impl LexemeMapper for JumpMode {
    // Translates a regular set of tokens into one appropriate
    // appropriate for jump mode. Lexemes of a size greater than 2
    // have their leading characters replaced with a jump tag, and
    // the set of categories is reduced to two: keywords (tags) and
    // regular text.
    //
    // We also track jump tag locations so that tags can be
    // resolved to po