mod build;
mod completion;
mod config;
mod env;
mod man;
mod tag;

use crate::prelude::*;

/// Bash cli project manager using `argc`.
#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum ShincCmd {
    Build(build::ShincBuildCmd),
    Completion(completion::ShincCompletionCmd),
    Man(man::ShincManCmd),
}
