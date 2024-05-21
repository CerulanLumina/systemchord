use anyhow::{anyhow, Context};
use clap::Parser;
use log::LevelFilter;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};
use crate::backend::Backend;
use crate::config::Config;

mod backend;
mod chord;
mod config;
mod key_names;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "luminasapphira";
const APPLICATION: &str = env!("CARGO_PKG_NAME");

fn default_config() -> PathBuf {
    let dirs = directories::ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();
    dirs.config_dir().join(format!("{APPLICATION}.toml"))
}

fn real_main(cli: CLI) -> anyhow::Result<()> {
    let (config_path, defaulted) = match cli.config {
        Some(conf) => (conf, false),
        None => (default_config(), true),
    };

    let config = match config_path.try_exists() {
        Ok(true) => {
            let conf = io::read_to_string(File::open(&config_path).context("Opening config file")?)
                .context("Reading config file")?;
            toml::from_str::<Config<backend::BackendConfig>>(&conf).context("Reading config")?
        }
        Ok(false) => {
            if defaulted {
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
                Config::default()
            } else {
                log::error!("Cannot read user-provided config.");
                return Err(anyhow!("Config missing"));
            }
        }
        Err(err) => {
            return Err(anyhow!(err).context("Path to config not accessible"));
        }
    };
    let recv = backend::CurrentBackend::poll_events(config.os_config).context("Starting event polling")?;

    chord::start(recv, config.core);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .parse_env("SYSTEMCHORD_LOG")
        .init();

    let cli = CLI::parse();

    log::info!("Starting {APPLICATION}");

    if let Err(err) = real_main(cli) {
        log::error!("An error occurred during initialization: {err:?}");
    }

    Ok(())
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Parser)]
struct CLI {
    /// Set an alternate config file. Default is OS-dependent, check docs.
    #[arg(short, long)]
    config: Option<PathBuf>,
}
