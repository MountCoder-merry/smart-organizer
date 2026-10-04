use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

fn temporary_path(path: &Path) -> PathBuf {
    path.with_extension("json.tmp")
}

fn backup_path(path: &Path) -> PathBuf {
    path.with_extension("json.bak")
}

/// Load JSON from the primary file, falling back to the last known-good backup.
/// `Ok(None)` means neither file exists yet.
pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, String> {
    let candidates = [path.to_path_buf(), backup_path(path)];
    let mut errors = Vec::new();
    let mut found_file = false;

    for candidate in candidates {
        match fs::read(&candidate) {
            Ok(bytes) => {
                found_file = true;
                match serde_json::from_slice(&bytes) {
                    Ok(value) => return Ok(Some(value)),
                    Err(error) => errors.push(format!("{}: {error}", candidate.display())),
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => errors.push(format!("{}: {error}", candidate.display())),
        }
    }

    if !found_file && errors.is_empty() {
        Ok(None)
    } else {
        Err(errors.join("; "))
    }
}

/// Persist bytes with a synced temporary file and a recoverable backup.
/// The old primary is retained as `.bak` until the new file is in place.
pub fn write_json_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let temporary = temporary_path(path);
    let backup = backup_path(path);
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    drop(file);

    let had_primary = path.exists();
    if had_primary {
        if backup.exists() {
            fs::remove_file(&backup)?;
        }
        fs::rename(path, &backup)?;
    }

    match fs::rename(&temporary, path) {
        Ok(()) => {
            if backup.exists() {
                fs::remove_file(backup)?;
            }
            Ok(())
        }
        Err(error) => {
            let _ = fs::remove_file(&temporary);
            if had_primary && !path.exists() {
                let _ = fs::rename(&backup, path);
            }
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

    struct TestDir(PathBuf);

    impl TestDir {
        fn new() -> Self {
            let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "smart-organizer-persistence-{}-{id}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("tempdir");
            Self(path)
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn writes_json_and_recovers_from_a_corrupt_primary() {
        let temp = TestDir::new();
        let path = temp.0.join("rules.json");
        write_json_atomic(&path, br#"[1, 2]"#).expect("write");
        assert_eq!(load_json::<Vec<u8>>(&path).expect("load"), Some(vec![1, 2]));

        fs::write(backup_path(&path), br#"[3, 4]"#).expect("backup");
        fs::write(&path, b"not json").expect("corrupt primary");
        assert_eq!(
            load_json::<Vec<u8>>(&path).expect("recover"),
            Some(vec![3, 4])
        );
    }

    #[test]
    fn returns_none_when_no_saved_file_exists() {
        let temp = TestDir::new();
        assert_eq!(
            load_json::<Vec<u8>>(&temp.0.join("missing.json")).expect("load"),
            None
        );
    }
}
