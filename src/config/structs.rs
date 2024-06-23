use crate::config::configured_key::ConfiguredKey;
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

const CHORD_OPTS_DEFAULT: ChordOpts = ChordOpts {
    passthrough: true,
    exclusive: false,
};

#[derive(Deserialize, Default, Debug)]
pub struct Config {
    pub executors: Vec<Executor>,
}

#[derive(Deserialize, Debug)]
pub struct Executor {
    #[serde(flatten)]
    pub backend: Backend,
    pub chords: Vec<Chord>,
    pub shell: Option<Vec<String>>,

    #[serde(default = "ChordOpts::default")]
    pub chord_options: ChordOpts,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct ChordOpts {
    pub passthrough: bool,
    pub exclusive: bool,
}

#[derive(Deserialize, Copy, Clone, Debug)]
pub struct ChordOptsChild {
    pub passthrough: Option<bool>,
    pub exclusive: Option<bool>,
}

impl Default for ChordOpts {
    fn default() -> Self {
        CHORD_OPTS_DEFAULT
    }
}

#[derive(Deserialize, Debug)]
pub struct Chord {
    pub sequence: Vec<ConfiguredKey>,
    pub action: ChordAction,
    pub options: Option<ChordOptsChild>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ChordAction {
    Shell(String),
    Command(Vec<String>),
}

#[derive(Deserialize, Debug)]
#[serde(tag = "backend", rename_all = "lowercase")]
pub enum Backend {
    #[cfg(feature = "backend-evdev")]
    Evdev {
        device: PathBuf,
        #[serde(default = "default_true")]
        retry: bool,
    },
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "backend-evdev")]
            Backend::Evdev { device, .. } => {
                write!(f, "evdev (on {})", device.to_string_lossy())
            }
        }
    }
}

const fn default_true() -> bool {
    true
}
