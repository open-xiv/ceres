use crate::{
    apis::{notion, subook},
    model::SuConfig,
};
use tauri::Window;

use crate::model::{FightRecord, NotionConfig, SuMentorConfig};

#[tauri::command]
pub async fn to_notion(
    window: Window,
    fights: Vec<FightRecord>,
    notion_config: NotionConfig,
) -> Result<(), String> {
    // filter out useless fights
    let fights: Vec<FightRecord> = fights.into_iter().filter(|f| f.useful).collect();

    // get base index from notion
    let base = notion::fetch_base(&notion_config).await?;

    // update to notion
    for (idx, fight) in fights.iter().enumerate() {
        // send request to update notion
        notion::update_fight(idx, base, fight, &notion_config).await?;

        // use event hook to update progress
        let _ = window.emit("export-progress", idx + 1);
    }

    // update base index
    let _ = notion::update_base(fights.len() + base, &notion_config).await?;
    Ok(())
}

#[tauri::command]
pub async fn to_subook(
    window: Window,
    fights: Vec<FightRecord>,
    su_config: SuConfig,
) -> Result<(), String> {
    // filter out useless fights
    let fights: Vec<FightRecord> = fights.into_iter().filter(|f| f.useful).collect();

    // update to subook
    for (idx, fight) in fights.iter().enumerate() {
        // send request to update subook
        subook::update_fight(fight, &su_config).await?;

        // use event hook to update progress
        let _ = window.emit("export-progress", idx + 1);
    }
    Ok(())
}

#[tauri::command]
pub fn to_json(window: Window, fights: Vec<FightRecord>) -> Result<String, String> {
    // filter out useless fights
    let fights: Vec<FightRecord> = fights.into_iter().filter(|f| f.useful).collect();

    // serialize to json
    let json = serde_json::to_string(&fights).map_err(|e| e.to_string())?;

    // use event hook to update progress
    let _ = window.emit("export-progress", fights.len());
    Ok(json)
}

#[tauri::command]
pub async fn count_times(config: SuMentorConfig) -> Result<usize, String> {
    // use subook > notion
    if config.su.token != "invalid token" {
        let rst = subook::fetch_base(&config.su).await;
        return rst;
    }
    // use notion as fallback
    let rst = notion::fetch_base(&config.notion).await;
    return rst;
}
