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
        let terminal = build_terminal().chain_err(|| "Failed to initialize terminal")?;
        let theme_path = preferences.borrow().theme_path()?;
        let theme_set = ThemeLoader::new(theme_path).load()?;

        let (killswitch_tx, killswitch_rx) = mpsc::sync_channel(0);
        EventListener::start(terminal.clone(), event_channel.clone(), killswitch_rx);

        Ok(View {
            terminal,
            last_key: None,
            preferences,
            scrollable_regions: HashMap::new(),
            render_caches: HashMap::new(),
            theme_set,
            event_channel,
            event_listener_killswitch: killswitch_tx
        })
    }

    pub fn build_presenter<'a>(&'a mut self) -> Result<Presenter<'a>> {
        Presenter::new(self)
    }

    ///
    /// Scrollable region delegation methods.
    ///

    pub fn scroll_to_cursor(&mut self, buffer: &Buffer) -> Result<()> {
        self.get_region(buffer)?.scroll_into_view(&buffer);

        Ok(())
    }

    pub fn scroll_to_center(&mut self, buffer: &Buffer) -> Result<()> {
        self.get_region(buffer)?.scroll_to_center(&buffer);

        Ok(())
    }

    pub fn scroll_up(&mut self, buffer: &Buffer, amount: usize) -> Result<()> {
        self.get_region(buffer)?.scroll_up(amount);

        Ok(())
    }

    pub fn scroll_down(&mut self, buffer: &Buffer, amount: usize) -> Result<()> {
        let current_offset = self.get_region(buffer)?.line_offset();
        let line_count = buffer.line_count();
        let half_screen_height = self.terminal.height() / 2;

        // Limit scrolling to 50% of the screen beyond the end of the buffer.
        let max = if line_count > half_screen_height {
            let visible_line_count =
                line_count.saturating_sub(current_offset);

            // Of the visible lines, allow scrolling down by however
            // many lines are beyond the halfway point of the screen.
            visible_line_count.saturating_sub(half_screen_height)
        } else {
            0
        };

        self.get_region(buffer)?.scroll_down(
            cmp::min(amount, max)
        );

        Ok(())
    }

    /// Cleans up buffer-related view data. This method
    /// should be called whenever a buffer is closed.
    pub fn forget_buffer(&mut self, buffer: &Buffer) -> Result<()> {
        self.scrollable_regions.remove(&buffer_key(buffer)?);
        self.render_caches.remove(&buffer_key(buffer)?);

        Ok(())
    }

    // Tries to fetch a scrollable region for the specified buffer,
    // inserting (and returning a reference to) a new one if not.
    fn get_region(&mut self, buffer: &Buffer) -> Result<&mut ScrollableRegion> {
        Ok(self.scrollable_regions
            .entry(buffer_key(buffer)?)
            .or_insert(
                ScrollableRegion::new(self.terminal.clone())
            )
        )
    }

    fn get_render_cache(&self, buffer: &Buffer) -> Result<&Rc<RefCell<HashMap<usize, RenderState>>>> {
        let cache = self.render_caches
            .get(&buffer_key(buffer)?)
            .ok_or("Buffer not properly initialized (render cache not present).")?;

        Ok(cache)
    }

    pub fn suspend(&mut self) {
        let _ = self.event_listener_killswitch.send(());
        self.termin