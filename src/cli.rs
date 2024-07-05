use std::path::PathBuf;

use clap::{CommandFactory, Parser};

use crate::commands::Command;
use crate::prelude::*;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(bin_name = clap::crate_name!())]
pub struct Cli {
    #[command(flatten)]
    global_opts: GlobalOpts,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(clap::Args, Debug)]
pub struct GlobalOpts {
    /// Set a custom config file
    #[arg(short = 'c', long, global = true, value_name = "FILE")]
    config: Option<PathBuf>,
}

impl Cli {
    pub fn exec() {
        if let Err(e) = Self::exec_inner() {
            println!("{:?}", e);
            ::std::process::exit(1);
        }
    }

    fn exec_inner() -> CliCommandResult {
        let cmd = Self::parse();
        cmd.run()
    }

    /// See also [`clap::Command::build`]
    ///
    /// can be used for completions.
    pub fn build() -> clap::Command {
        let mut cmd = Self::command();
        cmd.build();
        cmd
    }
}

impl CliCommand for Cli {
    fn run(&self) -> CliCommandResult {
        match &self.command {
            Some(cmd) => cmd.run(),
            None => {
                Self::command().print_long_help()?;
                Ok(())
            }
        }
    }
}
