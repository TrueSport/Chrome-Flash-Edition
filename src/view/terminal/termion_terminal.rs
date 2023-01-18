extern crate libc;
extern crate termion;

use crate::errors::*;
use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::unix::EventedFd;
use super::Terminal;
use std::io::Stdout;
use std::os::unix::io::AsRawFd;
use scribe::buffer::{Distance, Position};
use self::termion::color::{Bg, Fg};
use self::termion::{color, cursor};
use self::termion::input::{Keys, TermRead};
use self::termion::raw::{IntoRawMode, RawTerminal};
use self::termion::screen::{AlternateScreen, IntoAlternateScreen};
use self::termion::style;
use std::io::{BufWriter, Stdin, stdin, stdout, Write};
use std::fmt::Display;
use std::ops::Drop;
use std::sync::Mutex;
use std::time::Duration;
use crate::view::{Colors, Style};
use unicode_segmentation::UnicodeSegmentation;
use signal_hook::iterator::Signals;

use self::termion::event::Key as TermionKey;
use crate::input::Key;
use crate::models::application::Event;

const STDIN_INPUT: Token = Token(0);
const RESIZE: Token = Token(1);

pub struct TermionTerminal {
    event_listener: Poll,
    signals: Signals,
    input: Mutex<Option<Keys<Stdin>>>,
    output: Mutex<Option<BufWriter<RawTerminal<AlternateScreen<Stdout>>>>>,
    current_style: Mutex<Option<Style>>,
    current_colors: Mutex<Option<Colors>>,
    current_position: Mutex<Option<Position>>,
}

impl TermionTerminal {
    #[allow(dead_code)]
    pub fn new() -> Result<TermionTerminal> {
        let (event_listener, signals) = create_event_listener()?;

        Ok(TermionTerminal {
            event_listener,
            signals,
            input: Mutex::new(Some(stdin().keys())),
            output: Mutex::new(Some(create_output_instance())),
            current_style: Mutex::new(None),
            current_colors: Mutex::new(None),
            current_position: Mutex::new(None),
        })
    }

    // Clears any pre-existing styles.
    fn update_style(&self, style: Style) {
        if let Ok(mut guard) = self.output.lock() {
            if let Some(ref mut output) = *guard {
                // Check if style has changed.
                if let Ok(mut style_guard) = self.current_style.lock() {
                    if Some(style) != *style_guard {
                        if let Some(mapped_style) = map_style(style) {
                            let _ = write!(output, "{}", mapped_style);
                        } else {
                            let _ = write!(
                                output,
                                "{}",
                                style::Reset
                            );

                            // Resetting styles unfortunately clears active colors, too.
                            if let Ok(color_guard) = self.current_colors.lock() {
                                if let Some(ref current_colors) = *color_guard {
                                    match *current_colors {
                                        Colors::Default => { let _ = write!(output, "{}{}", Fg(color::Reset), Bg(color::Reset)); }
                                        Colors::Custom(fg, bg) =>