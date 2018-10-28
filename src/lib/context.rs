
extern crate simplelog;

use simplelog::LevelFilter;
use std::path::PathBuf;

/// The context is handed out to clients and is used to keep track of 
/// their current directory, logging level, and various other settings.
/// That current directory is handed off to the cluster to solve the
/// chicken and egg problem of mkdir not knowing how to find the parent
/// directory to link to
pub struct Context {
    current_location: PathBuf,
    log_level: LevelFilter,
}

impl Context {
    pub fn new() -> Self {
        Context {
            current_location: PathBuf::from("/"),
            log_level: LevelFilter::Error,
        }
    }
}
