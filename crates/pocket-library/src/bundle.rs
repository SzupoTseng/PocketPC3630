//! Several files picked together (a game plus the DLLs it needs, or a game
//! and its data) become one package, so nobody has to build a zip by hand.

use std::io::Write;
use std::path::{Path, PathBuf};

/// Write `files` into a new zip in `dir`, each at the zip's root, and return
/// its path. The zip is named after the first file that is not a `.dll`, so
/// the game keeps its own name in the library.
pub fn bundle_files(files: &[PathBuf], dir: &Path) -> std::io::Result<PathBuf> {
    let main = files
        .iter()
        .find(|p| !p.extension().is_some_and(|e| e.eq_ignore_ascii_case("dll")))
        .or_else(|| files.first())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "no files"))?;
    let stem = main
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "game".into());
    std::fs::create_dir_all(dir)?;
    let out = dir.join(format!("{stem}.zip"));
    let mut zip = zip::ZipWriter::new(std::fs::File::create(&out)?);
    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for file in files {
        let name = file
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "a file without a name")
            })?;
        zip.start_file(name, options)
            .map_err(std::io::Error::other)?;
        zip.write_all(&std::fs::read(file)?)?;
    }
    zip.finish().map_err(std::io::Error::other)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_end_up_at_the_root_of_a_zip_named_after_the_game() {
        let dir = tempfile::tempdir().unwrap();
        let cab = dir.path().join("MyGame.cab");
        let dll = dir.path().join("mfcce300.dll");
        std::fs::write(&cab, b"cab").unwrap();
        std::fs::write(&dll, b"dll").unwrap();
        let out = bundle_files(&[dll.clone(), cab.clone()], &dir.path().join("out")).unwrap();
        assert_eq!(out.file_name().unwrap(), "MyGame.zip");
        let mut zip = zip::ZipArchive::new(std::fs::File::open(&out).unwrap()).unwrap();
        let mut names: Vec<String> = (0..zip.len())
            .map(|i| zip.by_index(i).unwrap().name().to_string())
            .collect();
        names.sort();
        assert_eq!(names, ["MyGame.cab", "mfcce300.dll"]);
    }
}
