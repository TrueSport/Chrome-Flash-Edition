
use crate::models::application::Preferences;
use scribe::buffer::{Buffer, Position, Range};
use scribe::util::LineIterator;
use crate::view::buffer::{LexemeMapper, MappedLexeme, RenderState};
use crate::view::buffer::line_numbers::*;
use crate::view::{Colors, RENDER_CACHE_FREQUENCY, RGBColor, Style};
use crate::view::color::to_rgb_color;
use crate::view::terminal::{Cell, Terminal, TerminalBuffer};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use syntect::highlighting::{Highlighter, HighlightIterator, Theme};
use syntect::highlighting::Style as ThemeStyle;
use syntect::parsing::ScopeStack;
use unicode_segmentation::UnicodeSegmentation;
use crate::errors::*;

/// A one-time-use type that encapsulates all of the
/// details involved in rendering a buffer to the screen.
pub struct BufferRenderer<'a, 'p> {
    buffer: &'a Buffer,
    buffer_position: Position,
    cursor_position: Option<Position>,
    gutter_width: usize,
    highlights: Option<&'a [Range]>,
    stylist: Highlighter<'a>,
    current_style: ThemeStyle,
    line_numbers: LineNumbers,
    preferences: &'a Preferences,
    render_cache: &'a Rc<RefCell<HashMap<usize, RenderState>>>,
    screen_position: Position,
    scroll_offset: usize,
    terminal: &'a dyn Terminal,
    terminal_buffer: &'a mut TerminalBuffer<'p>,
    theme: &'a Theme,
}

impl<'a, 'p> BufferRenderer<'a, 'p> {
    pub fn new(buffer: &'a Buffer, highlights: Option<&'a [Range]>,
    scroll_offset: usize, terminal: &'a dyn Terminal, theme: &'a Theme,
    preferences: &'a Preferences,
    render_cache: &'a Rc<RefCell<HashMap<usize, RenderState>>>,
    terminal_buffer: &'a mut TerminalBuffer<'p>) -> BufferRenderer<'a, 'p> {
        let line_numbers = LineNumbers::new(&buffer, Some(scroll_offset));
        let gutter_width = line_numbers.width() + 1;

        // Build an initial style to start with,
        // which we'll modify as we highlight tokens.
        let stylist = Highlighter::new(theme);
        let current_style = stylist.get_default();

        BufferRenderer{
            buffer,
            cursor_position: None,
            gutter_width,
            highlights,
            stylist,
            current_style,
            line_numbers,
            buffer_position: Position{ line: 0, offset: 0 },
            preferences,
            render_cache,
            screen_position: Position{ line: 0, offset: 0 },
            scroll_offset,
            terminal,
            terminal_buffer,
            theme,
        }
    }

    fn on_cursor_line(&self) -> bool {
        self.buffer_position.line == self.buffer.cursor.line
    }

    fn print_rest_of_line(&mut self) {
        let on_cursor_line = self.on_cursor_line();
        let guide_offset = self.length_guide_offset();

        for offset in self.screen_position.offset..self.terminal.width() {
            let colors = if on_cursor_line || guide_offset.map(|go| go == offset).unwrap_or(false) {
                Colors::Focused
            } else {
                Colors::Default
            };

            self.print(Position{ line: self.screen_position.line, offset },
                       Style::Default,
                       colors,
                       " ");
        }
    }

    fn length_guide_offset(&self) -> Option<usize> {
        self.preferences.line_length_guide().map(|offset| self.gutter_width + offset)
    }

    fn advance_to_next_line(&mut self) {
        if self.inside_visible_content() {
            self.set_cursor();
            self.print_rest_of_line();

            // It's important to only increase this once we've entered the
            // visible area. Otherwise, we're moving the screen location even
            // though we're not yet rendering to it.
            self.screen_position.line += 1;
        }

        // Move the buffer position to the next line.
        self.buffer_position.line += 1;
        self.buffer_position.offset = 0;

        // Print this on the brand new line.
        self.print_line_number();
    }

    // Check if we've arrived at the buffer's cursor position,
    // at which point we can set it relative to the screen,
    // which will compensate for scrolling, tab expansion, etc.
    fn set_cursor(&mut self) {
        if self.inside_visible_content() && *self.buffer.cursor == self.buffer_position {
            self.cursor_position = Some(self.screen_position);
        }
    }

    fn current_char_style(&self, token_color: RGBColor) -> (Style, Colors) {
        let (style, colors) = match self.highlights {
            Some(highlight_ranges) => {
                for range in highlight_ranges {
                    if range.includes(&self.buffer_position) {
                        // We're inside of one of the highlighted areas.
                        // Return early with highlight colors.
                        if range.includes(&self.buffer.cursor) {
                            return (Style::Bold, Colors::SelectMode)
                        } else {
                            return (Style::Inverted, Colors::Default)
                        }
                    }
                }

                // We aren't inside one of the highlighted areas.
                // Fall back to other styling considerations.
                if self.on_cursor_line() {
                    (Style::Default, Colors::CustomFocusedForeground(token_color))
                } else {
                    (Style::Default, Colors::CustomForeground(token_color))
                }
            }
            None => {
                if self.on_cursor_line() {
                    (Style::Default, Colors::CustomFocusedForeground(token_color))
                } else {
                    (Style::Default, Colors::CustomForeground(token_color))
                }
            },
        };

        (style, colors)
    }

    pub fn print_lexeme<L: Into<Cow<'p, str>>>(&mut self, lexeme: L) {
        for character in lexeme.into().graphemes(true) {
            // Ignore newline characters.
            if character == "\n" { continue; }

            self.set_cursor();

            // Determine the style we'll use to print.
            let token_color = to_rgb_color(self.current_style.foreground);
            let (style, color) = self.current_char_style(token_color);

            if self.preferences.line_wrapping() && self.screen_position.offset == self.terminal.width() {
                self.screen_position.line += 1;
                self.screen_position.offset = self.gutter_width;
                self.print(self.screen_position, style, color, character.to_string());
                self.screen_position.offset += 1;
                self.buffer_position.offset += 1;
            } else if character == "\t" {
                // Calculate the next tab stop using the tab-aware offset,
                // *without considering the line number gutter*, and then
                // re-add the gutter width to get the actual/screen offset.
                let buffer_tab_stop = self.next_tab_stop(self.screen_position.offset - self.gutter_width);
                let mut screen_tab_stop = buffer_tab_stop + self.gutter_width;

                // Now that we know where we'd like to go, prevent it from being off-screen.
                if screen_tab_stop > self.terminal.width() {
                    screen_tab_stop = self.terminal.width();
                }

                // Print the sequence of spaces and move the offset accordingly.
                for _ in self.screen_position.offset..screen_tab_stop {
                    self.print(self.screen_position, style, color, " ");
                    self.screen_position.offset += 1;
                }
                self.buffer_position.offset += 1;