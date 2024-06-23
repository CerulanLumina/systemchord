mod configured_key;
mod structs;

use crate::{APPLICATION, ORGANIZATION, QUALIFIER};
use anyhow::{anyhow, Context};
#[cfg(test)]
pub use configured_key::*;
use std::{fs, fs::File, io, path::PathBuf};
pub use structs::*;

fn default_config() -> PathBuf {
    log::debug!("Using default config");
    let dirs = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();
    dirs.config_dir().join(format!("{APPLICATION}.toml"))
}

pub fn load_config(config_path: Option<PathBuf>) -> anyhow::Result<Config> {
    let (config_path, defaulted) = match config_path {
        Some(conf) => (conf, false),
        None => (default_config(), true),
    };

    log::info!("Loading config from {}", config_path.to_string_lossy());

    match config_path.try_exists() {
        Ok(true) => {
            log::debug!("Config file exists, reading.");
            let conf = io::read_to_string(File::open(&config_path).context("Opening config file")?)
                .context("Reading config file")?;
            toml::from_str::<Config>(&conf).context("Reading config")
        }
        Ok(false) => {
            log::debug!("Config file does not exist.");
            if defaulted {
                log::debug!("Creating default directories");
                fs::create_dir_all(
                    config_path
                        .parent()
                        .ok_or(anyhow!("No parent in default config path"))?,
                )
                .context("Creating default config directory")?;
                log::warn!(
                    "Default config does not exist. Create your custom config at `{}`.",
                    config_path.as_os_str().to_string_lossy()
                );
                Ok(Config::default())
            } else {
                log::error!("Cannot read user-provided config.");
                Err(anyhow!("Config missing"))
            }
        }
        Err(err) => {
            log::error!("Could not determine if config path exists & is readable");
            Err(anyhow!(err).context("Path to config not accessible"))
        }
    }
}
