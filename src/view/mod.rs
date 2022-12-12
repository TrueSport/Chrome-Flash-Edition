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
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Drop;
use std::sync::mpsc::{self, Sender, SyncSender};
use std::sync::Arc;
use self::theme_loader::ThemeLoader;
use syntect::highlighting::ThemeSet;

const RENDER_CACHE_FREQUENCY: usize = 100;

pub struct View {
    terminal: Arc<Box<dyn Terminal + Sync + Send + 'static>>,
    scrollable_regions: HashMap<usize, ScrollableRegion>,
    render_caches: HashMap<usize, Rc<RefCell<HashMap<usize, RenderState>>>>,
    pub theme_set: ThemeSet,
    preferences: Rc<RefCell<Preferences>>,
    pub last_key: Option<Key>,
    event_channel: Sender<Event>,
    event_listener_killswitch: SyncSender<()>
}

impl View {
    pub fn new(preferences: Rc<RefCell<Preferences>>, event_channel: Sender<Event>) -> Result<View> {
        let terminal = build_terminal().chain_err(|| "Fai