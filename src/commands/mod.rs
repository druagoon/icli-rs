use clap;
use icli_derive;

use crate::prelude::*;

mod completion;
mod ip;

#[derive(clap::Subcommand, icli_derive::CliCommand, Debug)]
pub enum Command {
    Completion(completion::CompletionCmd),
    #[command(subcommand)]
    IP(ip::IpCmd),
}
