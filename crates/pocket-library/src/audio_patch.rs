//! Missing EverQuest sounds can be supplied from the user's other episodes.
//! An audio-only patch must not reinstall the CAB or replace saved characters.
use std::collections::HashSet;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const MANIFEST: &str = "pockethle-audio.json";
const MAX_FILES: usize = 128;
const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const MAX_TOTAL_BYTES: usize = 32 * 1024 * 1024;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub game: String,
    pub files: Vec<String>,
}

pub struct Patch {
    pub game: String,
    files: Vec<(String, Vec<u8>)>,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn bounded<R: Read>(input: R, limit: u64) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    input.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(invalid("audio patch entry exceeds its size limit"));
    }
    Ok(bytes)
}

/// Return None for an ordinary game ZIP, validating the whole patch before writes.
pub fn read(path: &Path) -> io::Result<Option<Patch>> {
    let mut zip =
        zip::ZipArchive::new(fs::File::open(path)?).map_err(|e| invalid(e.to_string()))?;
    if !zip.file_names().any(|name| name == MANIFEST) {
        return Ok(None);
    }
    let body = bounded(
        zip.by_name(MANIFEST).map_err(|e| invalid(e.to_string()))?,
        65536,
    )?;
    let manifest: Manifest = serde_json::from_slice(&body).map_err(|e| invalid(e.to_string()))?;
    if manifest.version != 1 || manifest.files.is_empty() || manifest.files.len() > MAX_FILES {
        return Err(invalid("unsupported or empty audio patch"));
    }
    let mut seen = HashSet::new();
    let mut files = Vec::new();
    let mut total = 0;
    for name in manifest.files {
        let Some(leaf) = name.strip_prefix("Sounds/") else {
            return Err(invalid("audio patches may only supply Sounds/*.wav"));
        };
        if leaf.is_empty()
            || leaf.contains(['/', '\\', ':'])
            || !leaf.to_ascii_lowercase().ends_with(".wav")
            || !seen.insert(name.to_ascii_lowercase())
        {
            return Err(invalid("unsafe or duplicate audio patch path"));
        }
        let entry = zip.by_name(&name).map_err(|e| invalid(e.to_string()))?;
        let bytes = bounded(entry, MAX_FILE_BYTES)?;
        if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err(invalid("audio patch contains a non-WAVE file"));
        }
        total += bytes.len();
        if total > MAX_TOTAL_BYTES {
            return Err(invalid("audio patch exceeds total size limit"));
        }
        files.push((leaf.to_string(), bytes));
    }
    // No hidden executables, duplicate ZIP entries or undeclared files.
    if zip.len() != files.len() + 1 {
        return Err(invalid("audio patch contains undeclared entries"));
    }
    Ok(Some(Patch {
        game: manifest.game,
        files,
    }))
}

impl Patch {
    /// Add missing sounds only. Existing sounds must match, and save data is untouched.
    pub fn apply(&self, extracted: &Path) -> io::Result<usize> {
        let mut directory = extracted.join("Sounds");
        for entry in fs::read_dir(extracted)? {
            let entry = entry?;
            if entry.file_name().eq_ignore_ascii_case("Sounds") {
                if !entry.file_type()?.is_dir() {
                    return Err(invalid("installed Sounds path is not a directory"));
                }
                directory = entry.path();
            }
        }
        let mut pending = Vec::new();
        let existing: Vec<_> = if directory.exists() {
            fs::read_dir(&directory)?.collect::<io::Result<_>>()?
        } else {
            Vec::new()
        };
        for (name, bytes) in &self.files {
            if let Some(entry) = existing
                .iter()
                .find(|e| e.file_name().eq_ignore_ascii_case(name))
            {
                if !entry.file_type()?.is_file() || fs::read(entry.path())? != *bytes {
                    return Err(invalid(format!(
                        "installed audio differs; keeping {}",
                        entry.path().display()
                    )));
                }
            } else {
                pending.push((directory.join(name), bytes));
            }
        }
        fs::create_dir_all(&directory)?;
        let mut created: Vec<PathBuf> = Vec::new();
        let result = (|| {
            for (path, bytes) in &pending {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(path)?;
                created.push(path.clone());
                file.write_all(bytes)?;
            }
            Ok(pending.len())
        })();
        if result.is_err() {
            for path in created {
                let _ = fs::remove_file(path);
            }
        }
        result
    }
}
