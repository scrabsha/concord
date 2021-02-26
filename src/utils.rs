use std::io::Error;
use std::{fs, path::PathBuf};

use serde_derive::{Deserialize, Serialize};

use home::home_dir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub available_profiles: Vec<String>,
    pub selected_profile: String,
}

impl Config {
    pub fn new_with_initial(initial_profile_name: String) -> Config {
        Config {
            available_profiles: Vec::new(),
            selected_profile: initial_profile_name,
        }
    }

    fn path_from_config_dir_path(mut dir: PathBuf) -> PathBuf {
        dir.push("Config.toml");
        dir
    }

    pub fn create(initial_profile_name: String) -> Result<Config, AppError> {
        let c = Config::new_with_initial(initial_profile_name);
        let config_dir = concord_config_dir();
        fs::create_dir_all(config_dir.as_path())?;

        c.write()?;

        Ok(c)
    }

    pub fn read() -> Result<Config, AppError> {
        let config_dir = concord_config_dir();
        let config_file_path = Config::path_from_config_dir_path(config_dir);

        let config_text = fs::read_to_string(config_file_path)?;
        let c = toml::from_str(config_text.as_str()).unwrap();

        Ok(c)
    }

    pub fn write(&self) -> Result<(), AppError> {
        let config_dir = concord_config_dir();
        let config_file_path = Config::path_from_config_dir_path(config_dir);

        let text = toml::to_string(self).unwrap();
        fs::write(config_file_path, text.as_str())?;

        Ok(())
    }

    pub fn profile_storage_path(profile_name: &str) -> PathBuf {
        let mut p = concord_config_dir();
        p.push(profile_name);
        p
    }
}

pub fn concord_config_dir() -> PathBuf {
    let mut p = home_dir().expect("Failed to get home director");
    p.push(".config/concord/");

    p
}

pub fn discord_config_dir() -> PathBuf {
    let mut p = home_dir().expect("Failed to get home director");
    p.push(".config/discord");

    p
}

#[derive(Debug)]
pub enum AppError {
    Io(Error),
}

impl From<Error> for AppError {
    fn from(e: Error) -> AppError {
        AppError::Io(e)
    }
}
