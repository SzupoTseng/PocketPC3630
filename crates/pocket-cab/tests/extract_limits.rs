//! Extraction stays inside its limits and keeps every file it is given.

use std::fs;
use std::io::Write;
use std::path::Path;

use pocket_cab::limits::Budget;

fn cabinet(path: &Path, files: &[(&str, &[u8])]) {
    let mut builder = cab::CabinetBuilder::new();
    let folder = builder.add_folder(cab::CompressionType::MsZip);
    for (name, _) in files {
        folder.add_file(*name);
    }
    let mut writer = builder.build(fs::File::create(path).unwrap()).unwrap();
    let mut index = 0;
    while let Some(mut file) = writer.next_file().unwrap() {
        file.write_all(files[index].1).unwrap();
        index += 1;
    }
    writer.finish().unwrap();
}

#[test]
fn names_that_become_the_same_both_keep_their_contents() {
    let dir = tempfile::tempdir().unwrap();
    let cab = dir.path().join("game.cab");
    cabinet(&cab, &[("a\\b", b"first"), ("a_b", b"second")]);
    let out = dir.path().join("out");
    let files = pocket_cab::extract_all(&cab, &out).unwrap();
    assert_eq!(files.len(), 2);
    assert_eq!(fs::read(&files[0].extracted_path).unwrap(), b"first");
    assert_eq!(fs::read(&files[1].extracted_path).unwrap(), b"second");
    assert_ne!(files[0].extracted_path, files[1].extracted_path);
}

#[test]
fn a_cabinet_that_unpacks_too_large_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let cab = dir.path().join("bomb.cab");
    let zeros = vec![0u8; 64 * 1024];
    cabinet(&cab, &[("ZEROS.DAT", &zeros)]);
    // Compressed, this is a few hundred bytes.
    assert!(fs::metadata(&cab).unwrap().len() < 4096);
    let mut budget = Budget::new(16 * 1024, 1 << 20, 100);
    let error =
        pocket_cab::extract_all_within(&cab, dir.path().join("out"), &mut budget).unwrap_err();
    assert!(error.to_string().contains("too large to import"), "{error}");
}
