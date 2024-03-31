use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    pub id: u32,
    pub name: String,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub name: String,
    pub gauge: String, // tank, healer, melee, physical, magical, unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub instances: Vec<Instance>,
    pub jobs: Vec<Job>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Oper {
    pub op_code: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub op: Oper,
    pub instance: Instance,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub op: Oper,
    pub id: String,
    pub name: String,
    pub server: String,
    pub job: Job,
    pub level: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FightRecord {
    pub area: Area,
    pub players: Vec<Player>,
    pub pretty: bool,
    pub useful: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NotionConfig {
    pub page_id: String,
    pub token: String,
    pub sum_block_id: String,
}

impl Default for NotionConfig {
    fn default() -> Self {
        Self {
            page_id: String::from("no page id"),
            token: String::from("invalid token"),
            sum_block_id: String::from("no sum block id"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuConfig {
    pub token: String,
}

impl Default for SuConfig {
    fn default() -> Self {
        Self {
            token: String::from("invalid token"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct SuMentorConfig {
    pub notion: NotionConfig,
    pub su: SuConfig,
    pub theme: String,
    pub log_folder: String,
}

impl Default for SuMentorConfig {
    fn default() -> Self {
        Self {
            notion: NotionConfig::default(),
            su: SuConfig::default(),
            theme: String::from("cupcake"),
            log_folder: String::from(""),
        }
    }
}
