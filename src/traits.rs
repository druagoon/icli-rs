pub type CliCommandResult = anyhow::Result<()>;

pub trait CliCommand {
    fn run(&self) -> CliCommandResult;
}
