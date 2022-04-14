
use app_dirs::{app_dir, app_root, get_app_root, AppDataType, AppInfo};
use bloodhound::ExclusionPattern;
use crate::errors::*;
use crate::input::KeyMap;
use crate::models::application::modes::open;
use scribe::Buffer;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use crate::yaml::yaml::{Hash, Yaml, YamlLoader};
use crate::models::application::modes::SearchSelectConfig;

const APP_INFO: AppInfo = AppInfo {
    name: "amp",
    author: "Jordan MacDonald",
};
const FILE_NAME: &str = "config.yml";
const LINE_COMMENT_PREFIX_KEY: &str = "line_comment_prefix";
const LINE_LENGTH_GUIDE_KEY: &str = "line_length_guide";
const LINE_WRAPPING_KEY: &str = "line_wrapping";
const OPEN_MODE_KEY: &str = "open_mode";
const OPEN_MODE_EXCLUSIONS_KEY: &str = "exclusions";
const SEARCH_SELECT_KEY: &str = "search_select";
const SOFT_TABS_KEY: &str = "soft_tabs";
const SYNTAX_PATH: &str = "syntaxes";
const TAB_WIDTH_KEY: &str = "tab_width";