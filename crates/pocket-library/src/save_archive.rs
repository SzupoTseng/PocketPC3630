//! A player's progress as one zip, the same on every frontend.
//!
//! Progress is the game's save folder (`saves/`, where the emulator sends the
//! guest's writes) plus the registry file the kernel keeps beside the
//! executable. The game itself is left out, so a save stays small and moves
//! between the desktop, Android and browser builds.
//!
//! An imported zip is untrusted: only those two places may receive files, and
//! no name may climb out of them.

use std::fs;
use std::io::{self, Read, Seek, Write};
use std::path::{Component, Path, PathBuf};

use crate::save_data::SAVE_DIR_NAME;

/// The registry the kernel persists next to the guest executable.
pub const REGISTRY_FILE: &str = "pockethle-registry.dat";

const EXTRACTED_DIR: &str = "extracted";
/// A save is a few files of a few kilobytes; anything near this is not one.
const LARGEST_ENTRY: u64 = 64 * 1024 * 1024;

/// Every file that makes up the progress of the game stored in `game_dir`, as
/// (name inside a save zip, file on disk), sorted by name.
pub fn save_files(game_dir: &Path) -> Vec<(String, PathBuf)> {
    let mut found = Vec::new();
    collect(&game_dir.join(SAVE_DIR_NAME), SAVE_DIR_NAME, &mut found);
    let registry = game_dir.join(EXTRACTED_DIR).join(REGISTRY_FILE);
    if registry.is_file() {
        found.push((REGISTRY_FILE.to_string(), registry));
    }
    found.sort();
    found
}

fn collect(dir: &Path, prefix: &str, out: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = format!("{prefix}/{}", entry.file_name().to_string_lossy());
        if path.is_dir() {
            collect(&path, &name, out);
        } else if path.is_file() {
            out.push((name, path));
        }
    }
}

/// Where a file named `name` in a save zip belongs, or `None` if a save may not
/// write there.
pub fn target_for(game_dir: &Path, name: &str) -> Option<PathBuf> {
    if name == REGISTRY_FILE {
        return Some(game_dir.join(EXTRACTED_DIR).join(REGISTRY_FILE));
    }
    let relative = Path::new(name);
    let mut parts = relative.components();
    if parts.next() != Some(Component::Normal(SAVE_DIR_NAME.as_ref())) {
        return None;
    }
    let mut depth = 0;
    for part in parts {
        match part {
            Component::Normal(_) => depth += 1,
            _ => return None,
        }
    }
    (depth > 0 && !name.contains('\\')).then(|| game_dir.join(relative))
}

/// Write the progress of the game in `game_dir` as a zip. Returns how many
/// files it holds; zero means the game has not saved anything yet.
pub fn export<W: Write + Seek>(game_dir: &Path, out: W) -> io::Result<usize> {
    let files = save_files(game_dir);
    let mut zip = zip::ZipWriter::new(out);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, path) in &files {
        zip.start_file(name.as_str(), options).map_err(to_io)?;
        zip.write_all(&fs::read(path)?)?;
    }
    zip.finish().map_err(to_io)?;
    Ok(files.len())
}

/// Delete the progress of the game in `game_dir`. Returns how many files went.
pub fn reset(game_dir: &Path) -> io::Result<usize> {
    let files = save_files(game_dir);
    for (_, path) in &files {
        fs::remove_file(path)?;
    }
    Ok(files.len())
}

/// Replace the progress of the game in `game_dir` with a save zip. Nothing is
/// touched unless the whole zip is acceptable. Returns how many files it held.
pub fn import<R: Read + Seek>(game_dir: &Path, input: R) -> io::Result<usize> {
    let mut zip = zip::ZipArchive::new(input).map_err(to_io)?;
    let mut accepted = Vec::new();
    for index in 0..zip.len() {
        let mut entry = zip.by_index(index).map_err(to_io)?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().to_string();
        let target = target_for(game_dir, &name).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("not part of a save: {name}"),
            )
        })?;
        if entry.size() > LARGEST_ENTRY {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("too large: {name}"),
            ));
        }
        let mut bytes = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut bytes)?;
        accepted.push((target, bytes));
    }
    if accepted.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "the zip holds no save",
        ));
    }
    reset(game_dir)?;
    for (target, bytes) in &accepted {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(target, bytes)?;
    }
    Ok(accepted.len())
}

fn to_io(error: zip::result::ZipError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn game_with_progress() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let saves = dir.path().join("saves").join("slots");
        fs::create_dir_all(&saves).unwrap();
        fs::write(saves.join("hero.sav"), b"level 9").unwrap();
        fs::create_dir_all(dir.path().join("extracted")).unwrap();
        fs::write(
            dir.path().join("extracted").join(REGISTRY_FILE),
            b"HKCU|x|1",
        )
        .unwrap();
        fs::write(dir.path().join("extracted").join("game.exe"), b"MZ").unwrap();
        dir
    }

    #[test]
    fn a_save_moves_to_another_install_and_leaves_the_game_out() {
        let source = game_with_progress();
        let mut zip = Cursor::new(Vec::new());
        assert_eq!(export(source.path(), &mut zip).unwrap(), 2);

        let target = tempfile::tempdir().unwrap();
        fs::create_dir_all(target.path().join("saves")).unwrap();
        fs::write(target.path().join("saves").join("stale.sav"), b"old").unwrap();
        zip.set_position(0);
        assert_eq!(import(target.path(), zip).unwrap(), 2);

        let names: Vec<String> = save_files(target.path()).into_iter().map(|f| f.0).collect();
        assert_eq!(names, ["pockethle-registry.dat", "saves/slots/hero.sav"]);
        assert!(!target.path().join("extracted").join("game.exe").exists());
    }

    #[test]
    fn reset_removes_only_progress() {
        let game = game_with_progress();
        assert_eq!(reset(game.path()).unwrap(), 2);
        assert!(save_files(game.path()).is_empty());
        assert!(game.path().join("extracted").join("game.exe").exists());
    }

    #[test]
    fn names_outside_the_save_are_refused() {
        let game = Path::new("/g");
        assert!(target_for(game, "saves/a/b.sav").is_some());
        assert!(target_for(game, REGISTRY_FILE).is_some());
        for bad in [
            "saves/../game.exe",
            "/etc/passwd",
            "game.exe",
            "saves",
            "saves\\..\\x",
            "extracted/x",
        ] {
            assert!(target_for(game, bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn a_zip_with_anything_else_changes_nothing() {
        let game = game_with_progress();
        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::FileOptions::default();
        zip.start_file("saves/new.sav", options).unwrap();
        zip.write_all(b"new").unwrap();
        zip.start_file("extracted/game.exe", options).unwrap();
        zip.write_all(b"evil").unwrap();
        let bytes = zip.finish().unwrap().into_inner();
        assert!(import(game.path(), Cursor::new(bytes)).is_err());
        assert_eq!(save_files(game.path()).len(), 2);
    }
}
