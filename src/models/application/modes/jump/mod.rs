mod tag_generator;
mod single_character_tag_generator;

use luthor::token::Category;
use crate::util::movement_lexer;
use std::collections::HashMap;
use scribe::buffer::{Distance, Position};
use crate::models::application::modes::select::SelectMode;
use crate::mode