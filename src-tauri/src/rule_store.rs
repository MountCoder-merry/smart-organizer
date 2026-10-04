use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::errors::AppError;
use crate::rule_engine::{validate_rule, StructuredRule};

const RULES_FILE: &str = "rules.json";
const MAX_RULES: usize = 100;

fn rules_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to resolve the app data directory: {error}"),
        })?;
    fs::create_dir_all(&directory).map_err(|error| AppError::PersistenceFailed {
        message: format!("Unable to create the app data directory: {error}"),
    })?;
    Ok(directory.join(RULES_FILE))
}

pub fn load(app: &AppHandle) -> Result<Vec<StructuredRule>, AppError> {
    let path = rules_path(app)?;
    crate::persistence::load_json(&path)
        .map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to read saved rules: {error}"),
        })
        .map(|rules| rules.unwrap_or_default())
}

pub fn save(app: &AppHandle, rule: StructuredRule) -> Result<Vec<StructuredRule>, AppError> {
    validate_rule(&rule)?;
    let path = rules_path(app)?;
    let mut rules = load(app)?;
    if let Some(existing) = rules.iter_mut().find(|item| item.id == rule.id) {
        *existing = rule;
    } else {
        rules.insert(0, rule);
    }
    rules.truncate(MAX_RULES);
    let encoded =
        serde_json::to_vec_pretty(&rules).map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to encode saved rules: {error}"),
        })?;
    crate::persistence::write_json_atomic(&path, &encoded).map_err(|error| {
        AppError::PersistenceFailed {
            message: format!("Unable to write saved rules safely: {error}"),
        }
    })?;
    Ok(rules)
}
