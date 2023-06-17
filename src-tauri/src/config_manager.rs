use std::fs;
use std::path::Path;

use crate::model::CeresConfig;

#[tauri::command]
pub fn read_config(cfg_dir_path: String) -> Result<CeresConfig, String> {
    // check config directory: if not exists -> create
    let path = Path::new(&cfg_dir_path);
    if !path.exists() {
        fs::create_dir_all(path).or(Err("failed to create directory".to_string()))?;
    }

    // find config file: ${config_path}/config.json
    let config = path.join("config.json");
    if !config.exists() {
        fs::write(&config, "{}").or(Err("failed to create config file".to_string()))?;
        let config = CeresConfig::default();
        let _ = write_config(cfg_dir_path, config.clone());
        Ok(config)
    } else {
        let config =
            fs::read_to_string(&config).or(Err("failed to read config file".to_string()))?;
        let config: CeresConfig = serde_json::from_str(&config).map_err(|e| e.to_string())?;
        Ok(config)
    }
}

#[tauri::command]
pub fn write_config(cfg_dir_path: String, config: CeresConfig) -> Result<(), String> {
    // check config directory: if not exists -> create
    let path = Path::new(&cfg_dir_path);
    if !path.exists() {
        fs::create_dir_all(path).or(Err("failed to create directory".to_string()))?;
    }

    // find config file: ${config_path}/config.json
    let config_path = path.join("config.json");
    if !config_path.exists() {
        fs::write(&config_path, "{}").or(Err("failed to create config file".to_string()))?;
    }

    // save config
    let config = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    fs::write(&config_path, config).or(Err("failed to write config file".to_string()))?;

    // return
    Ok(())
}
