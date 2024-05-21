use std::collections::HashSet;
use anyhow::anyhow;
use serde::Deserialize;
use crate::backend::Key;

#[derive(Deserialize)]
pub struct Config<B> {
    #[serde(flatten)]
    pub core: CoreConfig,
    pub os_config: B,
}

#[derive(Deserialize)]
pub struct CoreConfig {
    pub hotchords: Vec<Chord>,
}

#[derive(Deserialize)]
pub struct Chord {
    keys: Vec<ConfiguredKey>,
    pub action: Vec<String>,
}

impl Chord {
    pub fn matches(&self, keys: &HashSet<Key>) -> bool {
        let mut yes = true;
        for our_key in &self.keys {
            if !keys.iter().any(|a| our_key.matches(a)) {
                yes = false;
            }
        }
        yes
    }
}

#[derive(Deserialize)]
#[serde(try_from = "String")]
pub struct ConfiguredKey {
    accepted: HashSet<Key>,
}

impl ConfiguredKey {
    pub fn matches(&self, key: &Key) -> bool {
        self.accepted.contains(key)
    }
}

impl TryFrom<String> for ConfiguredKey {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut accepted = HashSet::new();
        for key in value.split('|').map(|a| a.to_lowercase()) {
            if let Some(overrides) = crate::key_names::key_override(&key) {
                accepted.extend(overrides.into_iter());
            } else if let Some(k) = crate::key_names::get_key_for_name(&key) {
                if !accepted.insert(k) {
                    log::warn!("Duplicate key inserted: {}", &key);
                }
            } else {
                return Err(anyhow!("Did not understand key `{}`", &key));
            }
        }
        Ok(Self { accepted })
    }
}

impl<B: Default> Default for Config<B> {
    fn default() -> Self {
        Self {
            core: CoreConfig { hotchords: Vec::new() },
            os_config: B::default(),
        }
    }
}
