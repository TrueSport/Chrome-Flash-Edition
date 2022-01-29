
pub use self::key_map::KeyMap;

mod key_map;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Delete,
    Insert,
    Esc,
    Tab,
    Enter,
    AnyChar,
    Char(char),
    Ctrl(char),
}