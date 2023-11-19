//! The MinraConfig contains any required and optional configuration options that has to be read during startup

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::util::{InitError, Result};
use tauri::{AppHandle, Manager};

const DB_FILE_NAME: &'static str = "db.sqlite";
const CONF_FILE_NAME: &'static str = "minra.toml";
const CONF_DIR: &'static str = ".minra";

#[derive(Serialize, Deserialize)]
pub struct MinraConfig {
    pub library_path: PathBuf,
    #[serde(skip)]
    pub conf_dir: PathBuf,
    #[serde(skip)]
    pub db_path: PathBuf,
}

pub fn load_config(app: &AppHandle) -> Result<MinraConfig, InitError> {
    let path_handler = app.path();

    // read in the config file
    let config_dir: PathBuf = path_handler.app_config_dir()?;
    let config_file_path = config_dir.join(CONF_FILE_NAME);
    let conf_str = std::fs::read_to_string(config_file_path)?;

    let mut config: MinraConfig = toml::from_str(&conf_str)?;

    // add the library to the allowed scope
    app.asset_protocol_scope()
        .allow_directory(config.library_path, true);

    // the config dir and db path is computed from the library path
    config.conf_dir = config.library_path.join(CONF_DIR);
    config.db_path = config.conf_dir.join(DB_FILE_NAME);

    // now we make sure that the directories exists

    if !config.conf_dir.exists() {
        std::fs::create_dir_all(&config.conf_dir)?;
    }

    Ok(config)
}
