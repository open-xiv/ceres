use crate::apis::notion::{fetch_base, update_base, update_fight};
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
    let base = fetch_base(&notion_config).await?;

    // update to notion
    for (idx, fight) in fights.iter().enumerate() {
        // send request to update notion
        update_fight(idx, base, fight, &notion_config).await?;

        // use event hook to update progress
        let _ = window.emit("export-progress", idx + 1);
    }

    // update base index
    let _ = update_base(fights.len() + base, &notion_config).await?;
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
    let rst = fetch_base(&config.notion).await;
    rst
}
