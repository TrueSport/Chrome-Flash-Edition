use crate::view::terminal::{Cell, TerminalBufferIterator};
use scribe::buffer::Position;

pub struct TerminalBuffer<'c> {
    width: usize,
    height: usize,
    cells: Vec<Cell<'c>>,
}

impl<'c> TerminalBuffer<'c> {
    pub fn new(width: usize, height: usize) -> TerminalBuffer<'c> {
        TerminalBuffer{
            width,
            height,
            cells: vec![Cell::default(); width*height],
        }
    }

    pub fn set_cell(&mut self, position: Position, cell: Cell<'c>) {
        let index = position.line * self.width + position.offset;

        if index < self.cells.len() {
            self.cells[position.line * self.width + position.offset] = cell;
        }
    }

    pub fn clear(&mut self) {
        self.cells = vec![Cell::default(); self.width*self.height];
    }

    pub fn iter(&self) -> TerminalBufferIterator {
        TerminalBufferIterator::new(self.width, &self.cells)
    }

    #[cfg(test)]
    // For testing purposes, produces a String representation of the
    // terminal buffer that can be used to assert a particular state.
    pub fn content(&self) -> 