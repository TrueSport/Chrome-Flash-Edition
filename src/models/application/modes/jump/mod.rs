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
    // resolved to positions for performing the actual jump later on.
    fn map<'a, 'b>(&'a mut self, lexeme: &'b str, position: Position) -> Vec<MappedLexeme<'a>> {
        self.mapped_lexeme_values = Vec::new();
        self.current_position = position;

        for subtoken in movement_lexer::lex(lexeme) {
            if subtoken.category == Category::Whitespace {
                let distance = Distance::of_str(&subtoken.lexeme);

                // We don't do anything to whitespace tokens.
                self.mapped_lexeme_values.push(
                    MappedLexemeValue::Text((
                        subtoken.lexeme,
                        self.current_position
                    ))
                );

                // Advance beyond this subtoken.
                self.current_position += distance;
            } else {
                let tag = if self.first_phase {
                    if self.current_position.line >= self.cursor_line {
                        self.single_characters.next()
                    } else {
                        None // We haven't reached the cursor yet.
                    }
                } else if subtoken.lexeme.len() > 1 {
                    self.tag_generator.next()
                } else {
                    None
                };

                match tag {
                    Some(tag) => {
                        let tag_len = tag.len();

                        // Keep a copy of the current tag
                        // that we'll use to loan out a lexeme.
                        self.mapped_lexeme_values.push(
                            MappedLexemeValue::Tag((
                                tag.clone(),
                                self.current_position
                            ))
                        );

                        // Track the location of this tag.
                        self.tag_positions.insert(tag, self.current_position);

                        // Advance beyond this tag.
                        self.current_position += Distance{
                   