use std::path::{Path, PathBuf};

use anyhow::Context;

use super::tag::ArgcTag;

const CONFIG_FILE_NAMES: [&str; 2] = [".shinc.toml", "shinc.toml"];

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    project: Project,
    cli: Option<Cli>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Project {
    name: String,
    version: String,
    description: Option<String>,
    meta: Option<ProjectMeta>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ProjectMeta {
    author: Option<String>,
    dotenv: Option<String>,
    require_tools: Option<Vec<String>>,
    man_section: Option<u8>,
    inherit_flag_options: Option<bool>,
    combine_shorts: Option<bool>,
    symbol: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Cli {
    name: Option<String>,
}

impl Config {
    #[allow(dead_code)]
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let cfg_file = Self::find_config_file(path)
            .with_context(|| "can't find shinc config file".to_string())?;
        log::info!("use config file: {}", cfg_file.display());
        let cfg = ::config::Config::builder()
            .add_source(::config::File::from(cfg_file))
            .build()?
            .try_deserialize::<Self>()?;
        Ok(cfg)
    }

    #[allow(dead_code)]
    pub fn find_config_file<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
        CONFIG_FILE_NAMES
            .iter()
            .map(|&x| path.as_ref().join(x))
            .find(|x| x.exists())
            .map(|x| std::path::absolute(x).unwrap())
    }

    pub fn get_project(&self) -> &Project {
        &self.project
    }

    pub fn get_cli_name(&self) -> &str {
        self.cli.as_ref().and_then(|x| x.name.as_deref()).unwrap_or(&self.project.name)
    }
}

impl Project {
    pub fn to_argc_tags(&self) -> Vec<String> {
        vec![
            ArgcTag::get_describe(self.description.as_deref().unwrap_or_default().trim()),
            ArgcTag::get_meta_version(&self.version),
        ]
    }

    pub fn get_meta(&self) -> Option<&ProjectMeta> {
        self.meta.as_ref()
    }
}

impl ProjectMeta {
    pub fn to_argc_tags(&self) -> Vec<String> {
        let mut buf = vec![];
        if let Some(author) = &self.author {
            buf.push(ArgcTag::get_meta_author(author));
        }
        if let Some(dotenv) = &self.dotenv {
            buf.push(ArgcTag::get_meta_dotenv(dotenv));
        }
        if let Some(require_tools) = &self.require_tools {
            buf.push(ArgcTag::get_meta_require_tools(require_tools));
        }
        if let Some(man_section) = self.man_section {
            buf.push(ArgcTag::get_meta_man_section(man_section));
        }
        if let Some(inherit_flag_options) = self.inherit_flag_options {
            if inherit_flag_options {
                buf.push(ArgcTag::get_meta_inherit_flag_options());
            }
        }
        if let Some(combine_shorts) = self.combine_shorts {
            if combine_shorts {
                buf.push(ArgcTag::get_meta_combine_shorts());
            }
        }
        if let Some(symbol) = &self.symbol {
            buf.push(ArgcTag::get_meta_symbol(symbol));
        }
        buf
    }
}
