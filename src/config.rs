use std::{io, time::Duration};

use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DurationSeconds};

use directories::BaseDirs;

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde_as(as = "DurationSeconds<u64>")]
    pub work_duration: Duration,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub break_duration: Duration,
}

impl Default for Config {
    fn default() -> Self {
        match Config::read() {
            Ok(config) => config,
            Err(_) => {
                let new_config = Config {
                    work_duration: Duration::from_secs(1800),
                    break_duration: Duration::from_secs(30)
                };
                new_config.write().expect("Error writing new config file.");
                new_config
            }
        }
    }
}

impl Config {
    
    pub fn config_path() -> String {
        let base_dir = BaseDirs::new().expect("Couldn't find base directory!");
        let config_dir = base_dir.config_dir().join("pausemause");
        std::fs::create_dir_all(&config_dir).expect("Couldn't create directory for config.toml file.");
        let config_file_binding = config_dir.join("config.toml");
        let config_file = config_file_binding.to_str().unwrap();
        String::from(config_file)
    }
    
    pub fn write(&self) -> std::io::Result<()> {
        let s = toml::to_string(self).expect("Error converting config to string.");
        std::fs::write(Config::config_path(), s)
    }

    pub fn read() -> std::io::Result<Config> {
        let s = std::fs::read_to_string(Config::config_path())?;
        toml::from_str(&s).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse config file: {}", e))
        })
    } 
}
