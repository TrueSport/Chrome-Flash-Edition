use crate::view::terminal::{Cell, TerminalBufferIterator};
use scribe::buffer::Position;

pub struct TerminalBuffer<'c> {
    width: usize,
    height: usize,
    cells: Vec<Cell<'c>>,
}

impl<'c> TerminalBuffer<'c> {
    pub fn new(width: usize, height: usize) -> Terminal