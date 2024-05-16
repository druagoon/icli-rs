mod cli;
mod commands;
mod config;
mod consts;
mod de;
mod macros;
mod prelude;
mod traits;

fn main() {
    self::cli::Cli::exec();
}
