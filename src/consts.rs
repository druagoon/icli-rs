use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;

#[allow(unused)]
pub static ROOT_DIR: Lazy<&Path> =
    Lazy::new(|| Path::new(env!("CARGO_MANIFEST_DIR")));

#[allow(unused)]
pub static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| ROOT_DIR.join("src"));

#[allow(unused)]
pub static TEMPLATES_DIR: Lazy<PathBuf> =
    Lazy::new(|| ROOT_DIR.join("templates"));

#[allow(unused)]
pub static COMMANDS_DIR: Lazy<PathBuf> = Lazy::new(|| SRC_DIR.join("commands"));
