use serde_json::json;
use serde_json::{from_str, Value};

use crate::model::{FightRecord, SuConfig};
use crate::GLOBAL_CLIENT;

// subook related constants
const SUBOOK_URL: &str = "http://localhost:8123";

fn check_token(su_config: &SuConfig) -> bool {
    return false;
}

pub async fn fetch_token(su_config: &SuConfig) -> Result<String, String> {
    // if token valid, return it
    if check_token(su_config) {
        return Ok(su_config.token.clone());
    }

    let rsp = GLOBAL_CLIENT
        .get(format!("{}/protect/auth", SUBOOK_URL))
        .json(&json!({"key": su_config.key})) // Pass a reference to the JSON value
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let rsp = rsp.text().await.map_err(|e| e.to_string())?;
    let rsp = from_str::<Value>(&rsp).map_err(|e| e.to_string())?;

    Ok(rsp["token"].as_str().map_or_else(
        || Err("failed to get token".to_string()),
        |s| Ok(s.to_string()),
    )?)
}

pub async fn fetch_base(su_config: &SuConfig) -> Result<usize, String> {
    let rsp = GLOBAL_CLIENT
        .get(format!("{}/private/user", SUBOOK_URL))
        .bearer_auth(&su_config.token[..])
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    let rsp = rsp
        .text()
        .await
        .map_err(|e| format!("Failed to get response text: {}", e))?;

    let rsp =
        from_str::<Value>(&rsp).map_err(|e| format!("Failed to parse response as JSON: {}", e))?;

    let base = rsp["meta"]["base"].as_u64().ok_or("failed to get base")?;

    Ok(base as usize)
}

pub async fn update_fight(fight: &FightRecord, su_config: &SuConfig) -> Result<(), String> {
    // serialize to json
    let fight = json!(fight);

    // send request to update subook
    let _ = GLOBAL_CLIENT
        .post(format!("{}/private/fight", SUBOOK_URL))
        .bearer_auth(&su_config.token[..])
        .json(&fight)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
