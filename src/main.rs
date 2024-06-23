use anyhow::Context;
use clap::Parser;
use log::LevelFilter;

use crate::cli::Cli;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "luminasapphira";
const APPLICATION: &str = env!("CARGO_PKG_NAME");

mod backend;
mod chord;
mod cli;
mod config;
mod exec;
mod key;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(LevelFilter::Info)
        .parse_env("SYSTEMCHORD_LOG")
        .init();
    let cli = Cli::parse();
    log::info!("Starting {APPLICATION}");

    let config = config::load_config(cli.config).context("Loading config")?;

    let mut handles = Vec::with_capacity(config.executors.len() * 2);

    for executor in config.executors {
        log::info!("Starting chord service: {}", &executor.backend);
        let (recv, handle) = backend::start_backend(executor.backend);
        handles.push(handle);
        handles.push(chord::chord_handler(
            recv,
            executor.chords,
            executor.chord_options,
            executor.shell,
        ));
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
