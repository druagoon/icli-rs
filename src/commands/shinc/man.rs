use std::fs;
use std::path::PathBuf;

use anyhow::Context;

use super::config::Config;
use super::env::Env;
use crate::prelude::*;

/// Generate man-pages for the project.
#[derive(clap::Parser, Debug)]
pub struct ShincManCmd {
    /// Write man pages to DIR.
    #[arg(long, value_name = "DIR", default_value = "./target/man")]
    output_dir: PathBuf,
}

/// Generate the man page for the project.
impl CliCommand for ShincManCmd {
    fn run(&self) -> CliResult {
        let env = Env::init()?;
        let config = Config::load(env.get_base_dir())?;
        let cli_name = config.get_cli_name();
        let cli_file = env.get_target_file(cli_name);
        if !cli_file.is_file() {
            return Err(anyhow::format_err!("cli script file not found: {}", cli_file.display()));
        }
        if !self.output_dir.is_dir() {
            fs::create_dir_all(&self.output_dir)?;
        }
        let source = fs::read_to_string(cli_file)?;
        let pages = argc::mangen(&source, cli_name)?;
        for (filename, page) in pages {
            let dest = self.output_dir.join(filename);
            fs::write(&dest, page)
                .with_context(|| format!("failed to write '{}'", dest.display()))?;
            println!("saved {}", dest.display());
        }
        Ok(())
    }
}
