use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::prelude::*;

/// Generate the help target for Makefile.
#[derive(clap::Parser, Debug)]
pub struct MakefileAddHelpCmd {
    /// Where the help target shell is placed, relative to the current
    /// directory.
    #[arg(long, default_value = ".make")]
    shell_dir: String,
    /// Help target description
    #[arg(long, default_value = "Display help messages")]
    description: String,
}

const MAKEFILE_HELP_TEMPLATE_NAME: &str = "help";
const MAKEFILE_TEMPLATES: [(&str, &str); 1] =
    [(MAKEFILE_HELP_TEMPLATE_NAME, include_template!("makefile/help"))];

impl CliCommand for MakefileAddHelpCmd {
    fn run(&self) -> CliResult {
        let engine = &init_engine()?;
        generate(engine, &self.shell_dir, &self.description)?;
        Ok(())
    }
}

fn init_engine() -> tera::Result<tera::Tera> {
    let mut engine = tera::Tera::default();
    engine.add_raw_templates(MAKEFILE_TEMPLATES)?;
    Ok(engine)
}

fn generate(engine: &tera::Tera, shell_dir: &str, description: &str) -> anyhow::Result<()> {
    // Write shell file that used by help target
    let help_tail = PathBuf::from_iter(shell_dir.split('/')).join("help");
    let help = std::env::current_dir()?.join(&help_tail);
    if !help.is_file() {
        let help_dir = help.parent().unwrap();
        if !help_dir.is_dir() {
            fs::create_dir_all(help_dir)?;
        }
    }
    let fp = fs::File::create(&help)?;
    let ctx = tera::Context::new();
    engine.render_to(MAKEFILE_HELP_TEMPLATE_NAME, &ctx, &fp)?;
    #[cfg(unix)]
    fs::set_permissions(help, fs::Permissions::from_mode(0o755))?;

    // Write Makefile help target
    let makefile = std::env::current_dir()?.join("Makefile");
    let mut f = fs::OpenOptions::new().append(true).open(makefile)?;
    write!(
        f,
        r#"
.PHONY: help
help: ## {}
	@./{} "$(MAKEFILE_LIST)"
"#,
        description,
        help_tail.to_str().unwrap()
    )?;
    Ok(())
}
