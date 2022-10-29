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
   