mod completion;
mod ip;
mod makefile;
mod new;

use crate::prelude::*;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Ip(ip::IpCmd),
    #[command(subcommand)]
    Makefile(makefile::MakefileCmd),
    Completion(completion::CompletionCmd),
    New(new::NewCmd),
}
