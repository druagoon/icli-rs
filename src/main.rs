mod cli;
mod commands;
mod consts;
mod prelude;
mod traits;

fn main() {
    self::cli::Cli::exec();
}
