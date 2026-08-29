use std::{io::Result, time::Duration};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub work_duration: Duration,
    pub break_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_duration: Duration::from_secs(10),
            break_duration: Duration::from_secs(60)
        }
    }
}

impl Config {
    pub fn write_to_file(&self, path: &str) -> Result<()> {
        return Ok(())
    }
}
