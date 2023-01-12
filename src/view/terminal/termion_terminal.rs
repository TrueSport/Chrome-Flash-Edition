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
    s