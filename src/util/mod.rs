pub use self::selectable_vec::SelectableVec;

pub mod movement_lexer;
mod selectable_vec;
pub mod reflow;
pub mod token;

use crate::errors::*;
use crate::models::Application;
use scribe::buffer::{Buffer, LineRange, Position, Range};

/// Translates a line range to a regular range, including its last line.
/// Handles ranges including and end li