use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::SystemTime;

use chrono::{DateTime, Utc};

use crate::model::{Area, FightRecord, Meta, Oper, Player};

#[tauri::command]
pub fn load_act_log(path: String, meta: Meta) -> Result<Vec<FightRecord>, String> {
    let path = Path::new(&path);

    // dict -> load last file; file -> load
    if path.is_dir() {
        let entries = fs::read_dir(&path).map_err(|e| format!("{}", e))?;
        let mut latest_entry = None;
        let mut latest_time = SystemTime::UNIX_EPOCH;
        for entry in entries {
            // find the last created file
            let entry = entry.map_err(|e| format!("{}", e))?;
            let entry_time = entry
                .metadata()
                .map_err(|e| format!("{}", e))?
                .modified()
                .map_err(|e| format!("{}", e))?;
            if entry_time > latest_time {
                latest_entry = Some(entry);
                latest_time = entry_time;
            }
        }
        match latest_entry {
            Some(entry) => parse(&entry.path(), &meta).map_err(|e| format!("{}", e)),
            None => Err("no file in the directory".to_string()),
        }
    } else {
        parse(&path, &meta).map_err(|e| format!("{}", e))
    }
}

fn parse(path: &Path, meta: &Meta) -> Result<Vec<FightRecord>, String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut fights: Vec<FightRecord> = Vec::new();

    let mut in_instance = false;

    // read each line in logs file
    for (_, line_rst) in reader.lines().enumerate() {
        let line = line_rst.map_err(|e| format!("{}", e))?;

        // find start with '01' - change map
        if line.starts_with("01") {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 5 {
                return Err(format!("invalid line: {}", line));
            }

            let timestamp = parts[1];
            let timestamp = DateTime::parse_from_rfc3339(timestamp).map_err(|e| e.to_string())?;
            let timestamp = timestamp.with_timezone(&Utc).timestamp();

            let scene_name = parts[3].to_string();

            let op = Oper {
                op_code: parts[0].to_string(),
                timestamp,
            };

            let instance = meta.instances.iter().find(|&i| i.name == scene_name);
            if instance.is_some() {
                // start new fight
                in_instance = true;
                fights.push(FightRecord {
                    area: Area {
                        op,
                        instance: instance.unwrap().clone(),
                    },
                    players: Vec::new(),
                    pretty: false,
                    useful: false,
                })
            } else if in_instance {
                in_instance = false;
            }
        }
        // find start with '03' - fight
        else if line.starts_with("03") && in_instance {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 6 {
                return Err(format!("invalid line: {}", line));
            }

            // filter out non-player
            if !parts[2].starts_with("10") {
                continue;
            }

            let timestamp = parts[1];
            let timestamp = DateTime::parse_from_rfc3339(timestamp).map_err(|e| e.to_string())?;
            let timestamp = timestamp.with_timezone(&Utc).timestamp();

            let job: usize = usize::from_str_radix(parts[4], 16).map_err(|e| e.to_string())?;
            let job = meta.jobs.get(job).ok_or(format!("invalid job: {}", job))?;

            let level = u32::from_str_radix(parts[5], 16).map_err(|e| e.to_string())?;

            let op = Oper {
                op_code: parts[0].to_string(),
                timestamp,
            };

            let player = Player {
                op,
                id: parts[2].to_string(),
                name: parts[3].to_string(),
                job: job.clone(),
                level,
            };

            fights.last_mut().unwrap().players.push(player);
        }
    }

    // pretty each fight
    for fight in fights.iter_mut() {
        pretty_fight(fight);
    }

    Ok(fights)
}

fn pretty_fight(fight: &mut FightRecord) {
    if fight.pretty {
        return;
    }
    fight.pretty = true;

    // build hashmap for job gauges
    let gauges = vec!["tank", "healer", "melee", "physical", "magical", "unknown"];
    let gauges: HashMap<&str, usize> = gauges.iter().enumerate().map(|(i, g)| (*g, i)).collect();

    // sort players by job gauge
    fight.players[1..].sort_by(|a, b| {
        let a_gau = gauges.get(&a.job.gauge[..]);
        let b_gau = gauges.get(&b.job.gauge[..]);
        if a_gau.is_none() || b_gau.is_none() {
            return std::cmp::Ordering::Equal;
        }
        let a_gau = a_gau.unwrap();
        let b_gau = b_gau.unwrap();
        if a_gau == b_gau {
            return a.job.name.cmp(&b.job.name);
        }
        a_gau.cmp(b_gau)
    });

    // filter player with same name (repeat login, etc.)
    let mut names = HashSet::new();
    fight.players.retain(|p| {
        if names.contains(&p.name) {
            return false;
        }
        names.insert(p.name.clone());
        true
    });
}
