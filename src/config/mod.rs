use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProfileConfig {
    pub name: String,
    #[serde(default)]
    pub runtime: RuntimeConfig,
    #[serde(default)]
    pub module: BTreeMap<String, ModuleConfig>,
    #[serde(default)]
    pub commands: BTreeMap<String, CommandConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct RuntimeConfig {
    pub head_module: Option<String>,
    pub default_handler: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModuleConfig {
    #[serde(rename = "type")]
    pub module_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CommandConfig {
    pub handler: String,
}

pub fn load_profile(invocation_name: &str) -> Result<(PathBuf, ProfileConfig)> {
    let requested = PathBuf::from("configs").join(format!("{invocation_name}.toml"));
    let path = if requested.exists() {
        requested
    } else {
        PathBuf::from("configs/default.toml")
    };

    let profile = load_from_path(&path)?;
    Ok((path, profile))
}

fn load_from_path(path: &Path) -> Result<ProfileConfig> {
    let contents = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file at {}", path.display()))?;
    toml::from_str(&contents)
        .with_context(|| format!("failed to parse config file at {}", path.display()))
}
