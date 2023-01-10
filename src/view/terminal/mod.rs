mod buffer;
mod buffer_iterator;
mod cell;
mod termion_terminal;

#[cfg(any(test, feature = "bench"))]
mod test_terminal;

use crate::errors::*;
use crate::models::application::Event;
use scribe::buffer::Position;
use crate::view::{Colo