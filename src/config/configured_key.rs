use crate::key::{self, Key};
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct ConfiguredKey {
    accepted: Vec<Key>,
}

impl ConfiguredKey {
    pub fn matching(&self) -> impl Iterator<Item = &'_ Key> {
        self.accepted.iter()
    }

    pub fn matches(&self, key: &Key) -> bool {
        self.accepted.contains(key)
    }
}

impl FromStr for ConfiguredKey {
    type Err = UnknownKey;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut accepted = Vec::new();
        for key in s.split('|').map(|a| a.to_lowercase()) {
            if let Some(overrides) = key::key_override(&key) {
                accepted.extend(overrides.into_iter());
            } else if let Some(k) = key::get_key_for_name(&key) {
                if accepted.contains(&k) {
                    log::warn!("Duplicate key: {k} in {s}");
                }
                accepted.push(k);
            } else {
                return Err(UnknownKey { key });
            }
        }
        Ok(Self { accepted })
    }
}

impl TryFrom<String> for ConfiguredKey {
    type Error = UnknownKey;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ConfiguredKey::from_str(&value)
    }
}

#[derive(Debug)]
pub struct UnknownKey {
    pub key: String,
}

impl Display for UnknownKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown key: {}", self.key)
    }
}

impl std::error::Error for UnknownKey {}
