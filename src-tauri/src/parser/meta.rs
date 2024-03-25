use std::fs;
use std::path::Path;

use crate::model;

#[tauri::command]
pub fn load_meta(instances_path: String, jobs_path: String) -> Result<model::Meta, String> {
    let instances = load_instances(instances_path).map_err(|e| e.to_string())?;
    let jobs = load_jobs(jobs_path).map_err(|e| e.to_string())?;
    Ok(model::Meta { instances, jobs })
}

fn load_instances(path: String) -> Result<Vec<model::Instance>, String> {
    let path = Path::new(&path);

    // parse json
    let file_string = fs::read_to_string(path).or(Err("failed to read file".to_string()))?;
    let instances: Vec<model::Instance> =
        serde_json::from_str(&file_string).map_err(|e| e.to_string())?;

    // return
    Ok(instances)
}

fn load_jobs(path: String) -> Result<Vec<model::Job>, String> {
    let path = Path::new(&path);

    // parse json
    let file_string = fs::read_to_string(path).or(Err("failed to read file".to_string()))?;
    let jobs: Vec<model::Job> = serde_json::from_str(&file_string).map_err(|e| e.to_string())?;

    // return
    Ok(jobs)
}
