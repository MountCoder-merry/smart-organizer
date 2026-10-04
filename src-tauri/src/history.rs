use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::errors::AppError;
use crate::filesystem::OperationTransaction;

const HISTORY_FILE: &str = "transactions.json";
const MAX_TRANSACTIONS: usize = 50;

fn history_path(app: &AppHandle) -> Result<PathBuf, AppError> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to resolve the app data directory: {error}"),
        })?;
    fs::create_dir_all(&directory).map_err(|error| AppError::PersistenceFailed {
        message: format!("Unable to create the app data directory: {error}"),
    })?;
    Ok(directory.join(HISTORY_FILE))
}

pub fn load(app: &AppHandle) -> Result<Vec<OperationTransaction>, AppError> {
    let path = history_path(app)?;
    crate::persistence::load_json(&path)
        .map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to read transaction history: {error}"),
        })
        .map(|history| history.unwrap_or_default())
}

pub fn record(
    app: &AppHandle,
    transaction: OperationTransaction,
) -> Result<Vec<OperationTransaction>, AppError> {
    let path = history_path(app)?;
    let mut transactions = load(app)?;
    if let Some(existing) = transactions
        .iter_mut()
        .find(|item| item.id == transaction.id)
    {
        *existing = transaction;
    } else {
        transactions.insert(0, transaction);
    }
    transactions.truncate(MAX_TRANSACTIONS);
    let encoded =
        serde_json::to_vec_pretty(&transactions).map_err(|error| AppError::PersistenceFailed {
            message: format!("Unable to encode transaction history: {error}"),
        })?;
    crate::persistence::write_json_atomic(&path, &encoded).map_err(|error| {
        AppError::PersistenceFailed {
            message: format!("Unable to write transaction history safely: {error}"),
        }
    })?;
    Ok(transactions)
}
