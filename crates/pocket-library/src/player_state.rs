//! Preserve player-owned files while replacing an installation. EQ's registry
//! and SAV/CSS files may still live beside its EXE in older libraries.
use std::{fs, io, path::Path};

fn copy(source: &Path, target: &Path) -> io::Result<()> {
    let meta = fs::symlink_metadata(source)?;
    if meta.file_type().is_symlink() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "player data contains a symbolic link",
        ));
    }
    if meta.is_dir() {
        fs::create_dir_all(target)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            copy(&entry.path(), &target.join(entry.file_name()))?;
        }
    } else if meta.is_file() {
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(source, target)?;
    }
    Ok(())
}
fn legacy(source: &Path, target: &Path) -> io::Result<()> {
    if !source.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let kind = entry.file_type()?;
        if kind.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "legacy data contains a symbolic link",
            ));
        }
        if kind.is_dir() {
            legacy(&path, &target.join(entry.file_name()))?;
        } else {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            let extension = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if name == "pockethle-registry.dat"
                || matches!(extension.as_str(), "sav" | "nam" | "css" | "ini" | "cfg")
            {
                copy(&path, &target.join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}
pub(crate) fn preserve(old: &Path, new: &Path) -> io::Result<()> {
    for name in [
        "saves",
        "controls.json",
        "web-controls.json",
        "presentation.json",
    ] {
        let source = old.join(name);
        if source.exists() {
            copy(&source, &new.join(name))?;
        }
    }
    legacy(&old.join("extracted"), &new.join("extracted"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn installing_new_assets_preserves_progress_and_control_overrides() {
        let dir = tempfile::tempdir().unwrap();
        let old = dir.path().join("old");
        let new = dir.path().join("new");
        for name in [
            "saves/roles/hero.sav",
            "saves/pockethle-registry.dat",
            "extracted/old/hero.css",
            "controls.json",
            "presentation.json",
        ] {
            let path = old.join(name);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, name).unwrap();
        }
        fs::create_dir_all(new.join("extracted")).unwrap();
        fs::write(old.join("extracted/game.exe"), "old exe").unwrap();
        fs::write(new.join("extracted/game.exe"), "new exe").unwrap();
        preserve(&old, &new).unwrap();
        assert_eq!(
            fs::read(new.join("saves/roles/hero.sav")).unwrap(),
            b"saves/roles/hero.sav"
        );
        assert_eq!(
            fs::read(new.join("extracted/old/hero.css")).unwrap(),
            b"extracted/old/hero.css"
        );
        assert_eq!(
            fs::read(new.join("controls.json")).unwrap(),
            b"controls.json"
        );
        assert_eq!(
            fs::read(new.join("extracted/game.exe")).unwrap(),
            b"new exe"
        );
        assert_eq!(
            fs::read(old.join("extracted/game.exe")).unwrap(),
            b"old exe"
        );
    }
}
