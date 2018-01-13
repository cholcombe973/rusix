extern crate toml;

use std::fs::File;
use std::io::{Read, Result};

#[derive(Debug, Deserialize)]
struct Config {
    global: Option<Global>,
    peers: Option<Vec<PeerConfig>>,
}

#[derive(Debug, Deserialize)]
struct Global {
    fsid: Uuid,
}

#[derive(Debug, Deserialize)]
struct Peer {
    ip: String,
    port: Option<u16>,
}

pub fn load_config(p: &Path)-> Result<Config>{
    let f = File::open(p)?;
    let mut toml_str = String::new();
    f.read_to_string(&mut toml_str)?;

    let decoded: Config = toml::from_str(toml_str)?;
    Ok(decoded)
}
