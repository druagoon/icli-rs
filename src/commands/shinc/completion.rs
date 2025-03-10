use std::str::FromStr;

use clap_complete::aot::Shell;

use super::config::Config;
use super::env::Env;
use crate::prelude::*;

/// Generate an auto-completion script for the project.
#[derive(clap::Parser, Debug)]
pub struct ShincCompletionCmd {
    #[arg(value_enum)]
    shell: Shell,
}

impl CliCommand for ShincCompletionCmd {
    fn run(&self) -> CliResult {
        let env = Env::init()?;
        let config = Config::load(env.get_base_dir())?;
        let shell = argc::Shell::from_str(&self.shell.to_string())?;
        let commands = vec![string!("argc"), config.get_cli_name().to_string()];
        let content = argc::generate_completions(shell, &commands);
        print!("{content}");
        Ok(())
    }
}
