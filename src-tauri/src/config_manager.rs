use std::fs;
use std::path::Path;

use crate::model::{CeresConfig, NotionConfig};

#[tauri::command]
pub async fn read_config(cfg_dir_path: String) -> Result<CeresConfig, String> {
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
        let mut config: CeresConfig = serde_json::from_str(&config).map_err(|e| e.to_string())?;

        // notion support
        if config.notion.token != "invalid token" {
            let s_id = config.notion.sum_block_id.clone();
            if s_id.is_empty() || s_id == "no sum block id" {
                config.notion.sum_block_id = fetch_notion_sum_block_id(&config.notion)
                    .await
                    .map_err(|e| e.to_string())?
                    .to_string();
                let _ = write_config(cfg_dir_path.clone(), config.clone());
            }
        }

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

async fn fetch_notion_sum_block_id(notion_config: &NotionConfig) -> Result<String, String> {
    let client = reqwest::Client::new();

    let rsp = client
        .get(format!(
            "https://api.notion.com/v1/blocks/{}/children",
            notion_config.page_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", "2022-02-22")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let rsp = rsp.text().await.map_err(|e| e.to_string())?;
    let rsp = serde_json::from_str::<serde_json::Value>(&rsp).map_err(|e| e.to_string())?;

    // find ["results"][x]["type == callout"]
    for block in rsp["results"]
        .as_array()
        .ok_or("failed to get sum block id")?
    {
        let b_type = block["type"].as_str().ok_or("failed to get sum block id")?;
        if b_type == "callout" {
            let b_text = block["callout"]["rich_text"][0]["plain_text"]
                .as_str()
                .ok_or("failed to get sum block id")?;
            if b_text.starts_with("已完成稻穗") {
                return Ok(block["id"]
                    .as_str()
                    .ok_or("failed to get sum block id")?
                    .to_string());
            }
        }
    }

    Err("failed to get sum block id".to_string())
}
