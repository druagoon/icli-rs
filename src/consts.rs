use std::path::PathBuf;

use once_cell::sync::Lazy;

#[allow(dead_code)]
pub static ROOT_DIR: Lazy<PathBuf> = Lazy::new(|| std::env::current_dir().unwrap());

#[allow(dead_code)]
pub static SRC_DIR: Lazy<PathBuf> = Lazy::new(|| ROOT_DIR.join("src"));

#[allow(dead_code)]
pub static TEMPLATES_DIR: Lazy<PathBuf> = Lazy::new(|| ROOT_DIR.join("templates"));

#[allow(dead_code)]
pub static COMMANDS_DIR: Lazy<PathBuf> = Lazy::new(|| SRC_DIR.join("commands"));

#[allow(dead_code)]
pub static PROJECT_ID: &str = clap::crate_name!();
