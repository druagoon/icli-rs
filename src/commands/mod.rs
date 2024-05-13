use clap;
use icli_derive;

use crate::prelude::*;

mod completion;
mod ip;
mod new;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    #[command(subcommand)]
    Ip(ip::IpCmd),
    Completion(completion::CompletionCmd),
    New(new::NewCmd),
}
