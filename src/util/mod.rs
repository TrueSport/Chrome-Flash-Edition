pub use self::selectable_vec::SelectableVec;

pub mod movement_lexer;
mod selectable_vec;
pub mod reflow;
pub mod token;

use crate::errors::*;
use crate::models::Application;
use scribe::buffer::{Buffer, LineRange, Position, Range};

/// Translates a line range to a regular range, including its last line.
/// Handles ranges including and end line without trailing newline character.
pub fn inclusive_range(line_range: &LineRange, buffer: &mut Buffer) -> Range {
    let data = buffer.data();
    let next_line = line_range.end() + 1;
    let line_count = buffer.line_count();
    let end_position = if line_count > next_line {
        // There's a line below the end of the range, so just use that
        // to capture the last line and its trailing newline character.
        Position {
            line: next_line,
            offset: 0,
        }
    } else {
        // There isn't a line below the end of the range, so try to get
        // the last line's length and use that as the ending offset.
        match data.lines().nth(line_range.end()) {
            Some(line_content) => {
                // Found the last line's content; use it.
                Position {
                    line: line_range.end(),
                    offset: line_content.len(),
                }
            }
            // Couldn't find any content for the last line; use a zero offset.
            None => {
                Position {
                    line: line_range.end(),
                    offset: 0,
                }
            }
        }
    };

    Range::new(Position {
               