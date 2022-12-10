pub mod color;
pub mod terminal;
mod buffer;
mod data;
mod event_listener;
mod presenter;
mod style;
mod theme_loader;

// Published API
pub use self::data::StatusLineData;
pub use self::buffer::{LexemeMapper, MappedLexeme};
pub use self::style::Style;
pub use self::color::{Colors, RGBColor};
pub use self::presenter::Presenter;
pub use self::terminal::*;

use crate::errors::*;
use crate::input::Key;
use crate::models::application::{Event, Preferences};
use self::buffer::{RenderCache, RenderState};
use self::buffer::ScrollableRegion;
use self::event_listener::EventListener;
use scribe::buffer::Buffer;
use std::cmp;
use std::collections::HashMap;
use std::rc::