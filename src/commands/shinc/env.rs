use std::fs;
use std::path::{Path, PathBuf};

const MAIN_FILENAME: &str = "main.sh";

pub struct Env {
    base_dir: PathBuf,
    src_dir: PathBuf,
    target_dir: PathBuf,
}

impl Env {
    pub fn init() -> anyhow::Result<Self> {
        let base_dir = std::env::current_dir()?;
        let src_dir = base_dir.join("src");
        let target_dir = base_dir.join("target");
        if !target_dir.is_dir() {
            fs::create_dir_all(&target_dir)?;
        }
        Ok(Self { base_dir, src_dir, target_dir })
    }

    pub fn get_base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    pub fn get_target_file<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.target_dir.join(path)
    }

    pub fn get_target_main(&self) -> PathBuf {
        self.get_target_file(MAIN_FILENAME)
    }

    pub fn get_src_file<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.src_dir.join(path)
    }

    pub fn get_src_main(&self) -> PathBuf {
        self.get_src_file(MAIN_FILENAME)
    }
}
