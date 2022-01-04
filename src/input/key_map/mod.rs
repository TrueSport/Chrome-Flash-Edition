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

    /// Returns the default YAML keymap data as a string.
    pub fn default_data() -> &'static str {
        include_str!("default.yml")
    }

    /// Merges each of the passed key map's modes, consuming them in the process.
    /// Note: the mode must exist to be merged; unmatched modes are discarded.
    ///
    /// e.g.
    ///
    /// normal:
    ///     k: "cursor::move_up"
    ///
    /// merged with:
    ///
    /// normal:
    ///     j: "cursor::move_down"
    /// unknown:
    ///     l: "cursor::move_right"
    ///
    /// becomes this:
    ///
    ///   "normal" => {
    ///       Key::Char('k') => commands::cursor::move_up
    ///       Key::Char('j') => commands::cursor::move_down
    ///   }
    ///
    pub fn merge(&mut self, mut key_map: KeyMap) {
        // Step through the specified key map's modes.
        for (mode, other_key_bindings) in key_map.iter_mut() {
            // Fetch the current key bindings for the specified mode.
            if let Some(key_bindings) = self.get_mut(mode) {
                for (key, command) in other_key_bindings.drain() {
                    key_bindings.insert(key, command);
                }
            }
        }
    }
}

/// Parses the key bindings for a particular mode.
///
/// e.g.
///
///   k: "cursor::move_up"
///
/// becomes this HashMap entry:
///
///   Key::Char('k') => [commands::cursor::move_up]
///
fn parse_mode_key_bindings(mode: &Yaml, commands: &HashMap<&str, Command>) -> Result<HashMap<Key, SmallVec<[Command; 4]>>> {
    let mode_key_bindings = mode.as_hash().ok_or(
        "Keymap mode config didn't return a hash of key bindings",
    )?;

    let mut key_bindings = HashMap::new();
    for (yaml_key, yaml_command) in mode_key_bindings {
        // Parse modifier/character from key component.
        let key = parse_key(yaml_key.as_str().ok_or_else(||
            "A keymap key couldn't be parsed as a string".to_string()
        )?)?;

        let mut key_commands = SmallVec::new();

        // Parse and find command reference from command component.
        match *yaml_command {
            Yaml::String(ref command) => {
                let command_string = command.as_str();

                key_commands.push(
                    *commands.get(&command_string).ok_or_else(|| format!(
                        "Keymap command \"{}\" doesn't exist",
                        command_string
                    ))?
                );
            },
            Yaml::Array(ref command_array) => {
                for command in command_array {
                    let command_string = command.as_str().ok_or_else(||
                        format!("Keymap command \"{:?}\" couldn't be parsed as a string", command)
                    )?;

                    key_commands.push(
                        *commands.get(command_string).ok_or_else(||
                            format!("Keymap command \"{}\" doesn't exist", command_string)
                        )?
                    );
                }
            },
            _ => bail!(format!("Keymap command \"{:?}\" couldn't be parsed", yaml_command))
        }

        // Add a key/command entry to the mapping.
        key_bindings.insert(key, key_commands);
    }

    Ok(key_bindings)
}

/// Parses a str-based key into its Key equivalent.
///
/// e.g.
///
///   ctrl-r becomes Key::Ctrl('r')
///
fn parse_key(data: &str) -> Result<Key> {
    let mut key_components = data.split('-');
    let component = key_components.next().ok_or(
        "A keymap key is an empty string",
    )?;

    if let Some(key) = key_components.next() {
        // We have a modifier-qualified key; get the key.
        let key_char = key.chars().nth(0).ok_or_else(|| format!(
            "Keymap key