use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use once_cell::sync::Lazy;

use super::config::{Config, Project};
use super::env::Env;
use crate::prelude::*;
use crate::utils::fs::{read_lines, set_executable};

static REGEX_INCLUDE: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r#"^include\s+"(?<filename>.+)"$"#).unwrap());
const SHEBANG_PREFIX: &str = "#!";

/// Generate and build project shell script.
#[derive(clap::Parser, Debug)]
pub struct ShincBuildCmd {}

impl CliCommand for ShincBuildCmd {
    fn run(&self) -> CliResult {
        build()
    }
}

struct Target {
    inner: fs::File,
}

impl Target {
    fn new(fp: fs::File) -> Self {
        Self { inner: fp }
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        self.inner.flush()?;
        Ok(())
    }

    fn writeln(&mut self, buf: &str) -> anyhow::Result<()> {
        Ok(writeln!(self.inner, "{}", buf)?)
    }

    fn write_newline(&mut self) -> anyhow::Result<()> {
        Ok(writeln!(self.inner)?)
    }

    fn write_project_info(&mut self, project: &Project) -> anyhow::Result<()> {
        let mut lines = project.to_argc_tags();
        if let Some(meta) = project.get_meta() {
            lines.extend(meta.to_argc_tags());
        }
        let text = lines.join("\n");
        self.writeln(&text)
    }

    fn write_include_file<P: AsRef<Path>>(&mut self, buf: &str, path: P) -> anyhow::Result<()> {
        self.writeln(&format!("#{buf}"))?;
        for line in read_lines(path)? {
            self.writeln(&line?)?;
        }
        Ok(())
    }

    fn write_argc_hook(&mut self) -> anyhow::Result<()> {
        self.write_newline()?;
        self.writeln(r#"eval "$(argc --argc-eval "$0" "$@")""#)
    }
}

fn get_term_width() -> Option<usize> {
    std::env::var("TERM_WIDTH").ok().and_then(|v| v.parse().ok())
}

fn ensure_path<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    let p = path.as_ref();
    p.canonicalize().with_context(|| format!("file not found: {}", p.display()))
}

fn build() -> CliResult {
    let env = Env::init()?;
    let config = Config::load(env.get_base_dir())?;
    let project = config.get_project();
    let src_main = ensure_path(env.get_src_main())?;
    let target_main = env.get_target_main();
    let mut target = Target::new(fs::File::create(&target_main)?);
    // Parse and generate shell script with argc hook
    log::info!("parse src file: {}", src_main.display());
    for v in read_lines(&src_main)? {
        let line = &v?;
        if line.starts_with(SHEBANG_PREFIX) {
            target.writeln(line)?;
            target.write_newline()?;
            target.write_project_info(project)?;
        } else if let Some(caps) = REGEX_INCLUDE.captures(line) {
            let filename = &caps["filename"];
            let filepath = ensure_path(env.get_src_file(filename))?;
            log::info!("write include file: {}", filepath.display());
            target.write_include_file(line, filepath)?;
        } else {
            target.writeln(line)?;
        }
    }
    target.write_argc_hook()?;
    target.flush()?;
    log::info!("generate cli script: {}", target_main.display());

    // Build cli script
    let source = fs::read_to_string(target_main)?;
    let cli_name = config.get_cli_name();
    let content = argc::build(&source, cli_name, get_term_width())?;
    let cli_file = env.get_target_file(cli_name);
    fs::write(&cli_file, content)
        .with_context(|| format!("failed to write script to '{}'", cli_file.display()))?;
    log::info!("build cli script: {}", cli_file.display());
    set_executable(&cli_file)
        .with_context(|| format!("failed to set execute permission to '{}'", cli_file.display()))?;
    Ok(())
}
