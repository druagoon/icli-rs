mod cli;
mod commands;
mod prelude;
mod traits;

fn main() {
    self::cli::Cli::exec();
}
