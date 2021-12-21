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

        Ok(KeyMap(keymap))
    }

    /// Searches the keymap for the specified key.
    /// Character keys will fall back to wildcard character bindings
    /// if the specific character binding cannot be found.
    ///
    pub fn commands_for(&self, mode: &str, key: &Key) -> Option<SmallVec<[Command; 4]>> {
        self.0.get(mode).and_then(|mode_keymap| {
            if let Key::Char(_) = *key {
                // Look for a command for this specific character, falling
                // back to another search for a wildcard character binding.
                mode_keymap.get(key).or_else(|| mode_keymap.get(&Key::AnyChar))
            } else {
                mode_keymap.get(key)
            }
        }).map(|commands| (*commands).clone())
    }

    /// Loads the default keymap from a static
    /// YAML document injected during the build.
    pub fn default() -> Result<KeyMap> {
        let default_keymap_data = YamlLoader::load_from_str(KeyMap::default_data())
            .chain_err(|| "Couldn't parse default keymap")?
            .into_iter()
            .nth(0)
            .ok_or("Couldn't locate a document in the default keymap")?;

        KeyMap::from(&default_keymap_data.as_hash().unwrap())
    }

    /// Returns the defaul