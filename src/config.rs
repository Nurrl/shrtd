use figment::{Figment, Metadata, Profile, Provider};
use serde::{Deserialize, Serialize};

/** App's global configuration structure */
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /** App's address on which the server will be exposed */
    pub address: String,
    /** App's port on which the server will be exposed */
    pub port: u16,

    /** App's permanent data storage directory */
    pub data_dir: String,
    /** App's temporary data storage directory */
    pub tmp_dir: String,
    /** URI's random slug length */
    pub slug_length: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: String::from("0.0.0.0"),
            port: 8000,
            data_dir: String::from("/tmp/shrt"),
            tmp_dir: String::from("/tmp/shrt"),
            slug_length: 7,
        }
    }
}

impl Config {
    /* Allow the configuration to be extracted from any [`Provider`] */
    pub fn from<T: Provider>(provider: T) -> Result<Self, crate::Error> {
        Ok(Figment::from(provider).extract()?)
    }

    /* Allow the configuration to be extracted from any [`Provider`] */
    pub fn figment() -> Figment {
        Figment::from(Config::default())
    }
}

use figment::value::{Dict, Map};

impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("shrt Config")
    }

    fn data(&self) -> figment::error::Result<Map<Profile, Dict>> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        None
    }
}
