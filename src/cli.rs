use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    /// Set an alternate config file. Default is OS-dependent, check docs.
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
