use crate::apis::notion::fetch_block_id;
use crate::apis::subook::fetch_token;
use crate::model::SuMentorConfig;
use crate::parser::config;
use serde_json::{from_str, to_string};
use std::fs;
use std::path::Path;

pub fn ensure_dir_and_file_exist(path: &Path) -> Result<(), String> {
    if !path.exists() {
        fs::create_dir_all(path).map_err(|_| "failed to create directory".to_string())?;
    }
    let config_path = path.join("config.json");
    if !config_path.exists() {
        fs::write(&config_path, "{}").map_err(|_| "failed to create config file".to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn write_config(cfg_dir_path: String, config: SuMentorConfig) -> Result<(), String> {
    let path = Path::new(&cfg_dir_path);
    ensure_dir_and_file_exist(path)?;
    let config_path = path.join("config.json");
    let config_contents = to_string(&config).map_err(|e| e.to_string())?;
    fs::write(config_path, config_contents)
        .map_err(|_| "failed to write config file".to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn read_config(cfg_dir_path: String) -> Result<SuMentorConfig, String> {
    let path = Path::new(&cfg_dir_path);
    ensure_dir_and_file_exist(path)?;
    let config_path = path.join("config.json");
    let config_contents =
        fs::read_to_string(&config_path).map_err(|_| "failed to read config file".to_string())?;
    let mut config: SuMentorConfig = from_str(&config_contents).map_err(|e| e.to_string())?;

    // notion: fetch sum block id when first load
    if config.notion.token != "invalid token" {
        let s_id = &config.notion.sum_block_id;
        if s_id.is_empty() || s_id == "no sum block id" {
            config.notion.sum_block_id = fetch_block_id(&config.notion).await?.to_string();
            config::write_config(cfg_dir_path.clone(), config.clone())?;
        }
    }

    // subook: fetch token when each load
    if config.su.key != "invalid key" {
        config.su.token = fetch_token(&config.su).await?.to_string();
        config::write_config(cfg_dir_path.clone(), config.clone())?;
    }

    Ok(config)
}
