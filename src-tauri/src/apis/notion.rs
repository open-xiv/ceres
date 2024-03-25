use crate::model::{FightRecord, NotionConfig};
use crate::{tools::get_time, GLOBAL_CLIENT};
use serde_json::json;
use serde_json::{from_str, Value};

// notion related constants
const NOTION_API_VERSION: &str = "2022-02-22";

pub async fn fetch_block_id(notion_config: &NotionConfig) -> Result<String, String> {
    let rsp = GLOBAL_CLIENT
        .get(format!(
            "https://api.notion.com/v1/blocks/{}/children",
            notion_config.page_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", NOTION_API_VERSION)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let rsp = rsp.text().await.map_err(|e| e.to_string())?;
    let rsp = from_str::<Value>(&rsp).map_err(|e| e.to_string())?;

    for block in rsp["results"]
        .as_array()
        .ok_or("failed to get sum block id")?
    {
        if block["type"].as_str() == Some("callout") {
            let b_text = block["callout"]["rich_text"][0]["plain_text"].as_str();
            if b_text.map_or(false, |b_text| b_text.starts_with("已完成稻穗")) {
                return block["id"].as_str().map_or_else(
                    || Err("failed to get sum block id".to_string()),
                    |s| Ok(s.to_string()),
                );
            }
        }
    }
    Err("failed to get sum block id".to_string())
}

pub async fn fetch_base(notion_config: &NotionConfig) -> Result<usize, String> {
    let response = GLOBAL_CLIENT
        .get(format!(
            "https://api.notion.com/v1/blocks/{}",
            notion_config.sum_block_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", NOTION_API_VERSION)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;

    let response_json = serde_json::from_str::<serde_json::Value>(&response_text)
        .map_err(|e| format!("Failed to parse response as JSON: {}", e))?;

    let display_text = response_json["callout"]["rich_text"][0]["plain_text"]
        .as_str()
        .ok_or("Failed to extract base index text from response")?;

    let base = display_text
        .split_whitespace()
        .nth(1)
        .ok_or("Failed to split base index text into expected parts")?
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse base index as usize: {}", e))?;

    Ok(base)
}

pub async fn update_base(base: usize, notion_config: &NotionConfig) -> Result<(), String> {
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

    let response = GLOBAL_CLIENT
        .patch(format!(
            "https://api.notion.com/v1/blocks/{}",
            notion_config.sum_block_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", NOTION_API_VERSION)
        .json(&block)
        .send()
        .await
        .map_err(|e| format!("Failed to send update request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to update Notion base: received HTTP {}",
            response.status()
        ));
    }

    Ok(())
}

pub async fn update_fight(
    idx: usize,
    base: usize,
    fight: &FightRecord,
    notion_config: &NotionConfig,
) -> Result<(), String> {
    let title = format!("{:0>4} {}", idx + 1 + base, fight.area.instance.name);

    // timestamp: 1689243652 -> str {utc+8} {utc-4}
    let timestamp = fight.area.op.timestamp;
    let cur_time = format!(
        "时间 cn:[{}] us:[{}]",
        get_time(timestamp, 8),
        get_time(timestamp, -4)
    );

    // my job
    let my_job = format!("使用职业 {}", fight.players[0].job.name);

    let mut other_job = String::from("");
    for player in &fight.players[1..] {
        other_job.push_str(&format!("{}[{}] ", player.job.name, player.name));
    }
    let other_job = format!("队友信息 {}", other_job);

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
                                "content": cur_time
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
    let _ = GLOBAL_CLIENT
        .patch(format!(
            "https://api.notion.com/v1/blocks/{}/children",
            notion_config.page_id
        ))
        .bearer_auth(&notion_config.token[..])
        .header("Notion-Version", NOTION_API_VERSION)
        .json(&block)
        .send()
        .await;

    Ok(())
}
