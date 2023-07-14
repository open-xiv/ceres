use serde_json::json;
use tauri::Window;

use crate::model::{CeresConfig, FightRecord, NotionConfig};

#[tauri::command]
pub async fn to_notion(
    window: Window,
    fights: Vec<FightRecord>,
    notion_config: NotionConfig,
) -> Result<(), String> {
    let fights: Vec<FightRecord> = fights.into_iter().filter(|f| f.useful).collect();

    // get base index from notion
    let base = get_notion_base(&notion_config).await?;

    // update to notion
    for (idx, fight) in fights.iter().enumerate() {
        let title = format!("{:0>4} {}", idx + 1 + base, fight.area.instance.name);
        let my_job = format!("使用职业 {}", fight.players[0].job.name);

        let mut other_job = String::from("");
        for player in &fight.players[1..] {
            other_job.push_str(&format!("{} ", player.job.name));
        }
        let other_job = format!("队友职业 {}", other_job);

        // build patch json block
        let block = json!({
            "children": [
                {
                    "object": "block",
                    "type": "heading_3",
                    "heading_3": {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": title
                                }
                            }
                        ]
                    }
                },
                {
                    "object": "block",
                    "type": "bulleted_list_item",
                    "bulleted_list_item": {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": my_job
                                }
                            }
                        ]
                    }
                },
                {
                    "object": "block",
                    "type": "bulleted_list_item",
                    "bulleted_list_item": {
                        "rich_text": [
                            {
                                "type": "text",
                                "text": {
                                    "content": other_job.trim()
                                }
                            }
                        ]
                    }
                }
            ]
        });

        // send patch request
        let client = reqwest::Client::new();
        let _ = client
            .patch(format!(
                "https://api.notion.com/v1/blocks/{}/children",
                notion_config.page_id
            ))
            .bearer_auth(&notion_config.token[..])
            .header("Notion-Version", "2022-02-22")
            .json(&block)
            .send()
            .await;

        // use event hook to update progress
        let _ = window.emit("export-progress", idx + 1);
    }

    // update base index
    let _ = update_notion_base(fights.len() + base, &notion_config).await?;

    Ok(())
}

async fn get_notion_base(notion_config: &NotionConfig) -> Result<usize, String> {
    let client = reqwest::Client::new();

    let rsp = client
        .get(format!(
            "https://api.notion.com/v1/blocks/{}",
            notion_config.sum_block_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", "2022-02-22")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let rsp = rsp.text().await.map_err(|e| e.to_string())?;
    let rsp = serde_json::from_str::<serde_json::Value>(&rsp).map_err(|e| e.to_string())?;
    let dsp = rsp["callout"]["rich_text"][0]["plain_text"]
        .as_str()
        .unwrap_or("已完成稻穗 0 次！");

    let base = dsp
        .split(" ")
        .nth(1)
        .unwrap_or("0")
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    Ok(base)
}

async fn update_notion_base(base: usize, notion_config: &NotionConfig) -> Result<(), String> {
    let client = reqwest::Client::new();

    let block = json!({
        "callout": {
            "rich_text": [
                {
                    "type": "text",
                    "text": {
                        "content": format!("已完成稻穗 {} 次！", base),
                    }
                }
            ]
        }
    });

    let _ = client
        .patch(format!(
            "https://api.notion.com/v1/blocks/{}",
            notion_config.sum_block_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", "2022-02-22")
        .json(&block)
        .send()
        .await;

    Ok(())
}

#[tauri::command]
pub fn to_json(window: Window, fights: Vec<FightRecord>) -> Result<String, String> {
    let fights: Vec<FightRecord> = fights.into_iter().filter(|f| f.useful).collect();
    let json = serde_json::to_string(&fights).map_err(|e| e.to_string())?;
    // use event hook to update progress
    let _ = window.emit("export-progress", fights.len());
    Ok(json)
}

#[tauri::command]
pub async fn count_times(config: CeresConfig) -> Result<usize, String> {
    let rst = get_notion_base(&config.notion).await;
    rst
}
