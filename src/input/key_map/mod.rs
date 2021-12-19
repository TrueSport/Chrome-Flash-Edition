use crate::commands::{self, Command};
use crate::errors::*;
use crate::input::Key;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::convert::Into;
use crate::yaml::yaml::{Hash, Yaml, YamlLoader};

/// Nested HashMap newtype that provides a more ergonomic interface.
pub struct KeyMap(HashMap<String, HashMap<Key, SmallVec<[Command; 4]>>>);

impl KeyMap {
    /// Parses a Yaml tree of modes and their keybindings into a complete keymap.
    ///
    /// e.g.
    ///
    ///  normal:
    ///     k: "cursor::move_up"
    ///
    /// becomes this HashMap entry:
    ///
    ///   "normal" => { Key::Char('k') => commands::cursor::move_up }
    ///
    pub fn from(keymap_data: &Hash) -> Result<KeyMap> {
        let mut keymap = HashMap::new();
        let commands = commands::hash_map();

        for (yaml_mode, yaml_key_bindings) in keymap_data {
            let mode = yaml_mode.as_str().ok_or_else(||
                "A mode key couldn't be parsed as a string".to_string()
            )?;
            let key_bindings = parse_mode_key_bindings(yaml_key_bindings, &commands).
                chain_err(|| format!("Failed to parse keymaps for \"{}\" mode", mode))?;

            keymap.insert(mode.to_string(), key_bindings);
        }

        Ok(KeyMap(k