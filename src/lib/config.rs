extern crate toml;
extern crate uuid;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::IpAddr;
use std::path::Path;

use self::uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub global: Option<Global>,
    pub peers: Option<Vec<Peer>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Global {
    pub fsid: String, // Uuid Serialize and Deserialize isn't implemented for Uuid
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
}

impl Config {
    pub fn new(&self) -> Self {
        Config {
            global: None,
            peers: None,
        }
    }
    pub fn load_config(&self, p: &Path) -> Result<Self> {
        let mut f = File::open(p)?;
        let mut toml_str = String::new();
        f.read_to_string(&mut toml_str)?;

        let decoded: Config =
            toml::de::from_str(&toml_str).map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(decoded)
    }

    pub fn write_config(&self, p: &Path) -> Result<()> {
        let mut f = File::create(p)?;
        let toml_str = toml::ser::to_string(self).map_err(|e| Error::new(ErrorKind::Other, e))?;
        f.write_all(&toml_str.as_bytes())?;

        Ok(())
    }
}
