use crate::prelude::*;

#[derive(clap::Parser, Debug)]
pub struct {{ name_c }}Cmd {}

impl CliCommand for {{ name_c }}Cmd {
    fn run(&self) -> CliResult {
        Ok(())
    }
}
