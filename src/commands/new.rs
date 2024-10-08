use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Context;

use crate::consts;
use crate::prelude::*;

const CLI_CMD_TEMPLATE_NAME: &str = "cmd.rs";
const CLI_MOD_TEMPLATE_NAME: &str = "mod.rs";
const CLI_TEMPLATES: [(&str, &str); 2] = [
    (CLI_CMD_TEMPLATE_NAME, include_template!("cli/cmd.rs")),
    (CLI_MOD_TEMPLATE_NAME, include_template!("cli/mod.rs")),
];

/// Crate a new command and generate the layout files.
#[derive(clap::Parser, Debug)]
pub struct NewCmd {
    /// Use slash to separate multi-level subcommands.
    path: String,
}

impl CliCommand for NewCmd {
    fn run(&self) -> CliResult {
        let engine = &init_engine()?;
        let mut pb = PathBuf::from_str(&self.path)?;

        let cmd_dir = consts::COMMANDS_DIR.deref();
        std::env::set_current_dir(cmd_dir).with_context(|| {
            format!("can't find the commands directory: {:?}", cmd_dir.as_os_str())
        })?;
        create_cmd(engine, &pb)?;
        create_mod(engine, &mut pb)?;

        Ok(())
    }
}

fn init_engine() -> tera::Result<tera::Tera> {
    let mut engine = tera::Tera::default();
    engine.add_raw_templates(CLI_TEMPLATES)?;
    Ok(engine)
}

fn make_path<T: AsRef<Path>>(p: &T) -> PathBuf {
    let it = p.as_ref().iter().map(|v| v.to_str().unwrap().replace('-', ""));
    PathBuf::from_iter(it)
}

fn make_name<T: AsRef<Path>>(p: &T) -> String {
    heck::AsPascalCase(p.as_ref().to_str().unwrap()).to_string()
}

fn create_cmd(engine: &tera::Tera, pb: &PathBuf) -> anyhow::Result<()> {
    let mut filepath = make_path(pb);
    filepath.set_extension("rs");
    if !filepath.is_file() {
        let file_dir = filepath.parent().unwrap();
        if !file_dir.is_dir() {
            fs::create_dir_all(file_dir)?;
        }
    }

    let name_c = make_name(pb);
    let mut ctx = tera::Context::new();
    ctx.insert("name_c", &name_c);

    let fp = fs::File::create(&filepath)?;
    engine.render_to(CLI_CMD_TEMPLATE_NAME, &ctx, fp)?;

    Ok(())
}

fn create_mod(engine: &tera::Tera, pb: &mut PathBuf) -> anyhow::Result<()> {
    let mut is_subcommand = false;

    while let Some(prev) = pb.parent() {
        let filepath = make_path(pb);
        let parent = filepath.parent().unwrap();
        if !parent.is_dir() {
            fs::create_dir_all(parent)?;
        }
        let mod_rs = parent.join(CLI_MOD_TEMPLATE_NAME);
        if !mod_rs.is_file() {
            let fp = fs::File::create(&mod_rs)?;

            let group = make_name(&prev);
            let name = filepath.file_stem().unwrap().to_str().unwrap();
            let name_p = pb.file_stem().unwrap().to_str().unwrap();
            let name_v = heck::AsPascalCase(name_p).to_string();
            let name_c = make_name(pb);

            let has_dash = name_p.contains('-');
            let mut attrs = vec![];
            if is_subcommand {
                attrs.push("subcommand");
            }
            let ns = format!(r#"name = "{}""#, name);
            if has_dash {
                attrs.push(&ns);
            }
            let has_c_attrs = !attrs.is_empty();
            let c_attrs = attrs.join(", ");

            let mut ctx = tera::Context::new();
            ctx.insert("group", &group);
            ctx.insert("name", name);
            ctx.insert("name_v", &name_v);
            ctx.insert("name_c", &name_c);
            ctx.insert("has_c_attrs", &has_c_attrs);
            ctx.insert("c_attrs", &c_attrs);
            engine.render_to(CLI_MOD_TEMPLATE_NAME, &ctx, fp)?;
        }
        pb.pop();
        is_subcommand = true;
    }

    Ok(())
}
