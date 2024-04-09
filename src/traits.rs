use anyhow::Result;

pub type CliCommandResult = Result<()>;

pub trait CliCommand {
    fn run(&self) -> CliCommandResult;
}
