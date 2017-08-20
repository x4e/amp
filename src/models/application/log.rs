use errors::*;
use models::application::Preferences;
use std::fs::File;
use std::ops::{Deref, DerefMut};

pub struct Log(File);

const LOG_FILE_PATH: &'static str = "log";

impl Log {
    pub fn new() -> Result<File> {
        let mut path = Preferences::directory()?;
        path.push(LOG_FILE_PATH);

        File::create(&path).chain_err(|| "Failed to create log file")
    }
}

impl Deref for Log {
    type Target = File;

    fn deref(&self) -> &File {
        &self.0
    }
}

impl DerefMut for Log {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.0
    }
}
