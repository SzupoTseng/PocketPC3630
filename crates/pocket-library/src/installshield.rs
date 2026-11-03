//! Reading InstallShield 5 and 6 cabinets (`data1.hdr` + `data1.cab`, ...).
//!
//! Many Pocket PC downloads from 2000-2003 are desktop setup programs built
//! with InstallShield. The Pocket PC cabinets the game really is (one per
//! processor, e.g. `Game.arm.CAB`) sit inside InstallShield's own cabinet
//! format, which starts with `ISc(` and has nothing to do with Microsoft's
//! `MSCF`. This module only reads that format; it never runs anything.
//!
//! The layout follows the unshield project's description:
//! - every volume (`dataN.cab`) and the header (`data1.hdr`, or `data1.cab`
//!   when there is no separate header) starts with a 20-byte common header:
//!   signature, version, volume info, descriptor offset, descriptor size;
//! - the descriptor holds the file table: directory name offsets, then (up to
//!   version 5) offsets of file descriptors, or (version 6) descriptors of
//!   0x57 bytes each;
//! - a volume header after the common header gives the first and last file
//!   index stored in that volume;
//! - a compressed file is a run of chunks, each a 16-bit length followed by
//!   raw deflate data.
//!
//! Files split across volumes and obfuscated files are skipped: they are
//! rare in Pocket PC installers, and skipping costs one file, not the game.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const SIGNATURE: &[u8; 4] = b"ISc(";
const FILE_SPLIT: u16 = 0x1;
const FILE_OBFUSCATED: u16 = 0x2;
const FILE_COMPRESSED: u16 = 0x4;
const FILE_INVALID: u16 = 0x8;
/// No Pocket PC installer comes close; a header claiming more is damaged.
const MOST_FILES: usize = 20_000;
const VOLUMES: usize = 32;

/// Whether `bytes` start an InstallShield cabinet or header.
pub fn is_installshield(bytes: &[u8]) -> bool {
    bytes.starts_with(SIGNATURE)
}

/// Whether `path` is the file to read a set of InstallShield cabinets from:
/// `data1.hdr`, or `data1.cab` when no `data1.hdr` sits beside it.
pub fn is_header_file(path: &Path) -> bool {
    let Some(name) = path
        .file_name()
        .map(|n| n.to_string_lossy().to_ascii_lowercase())
    else {
        return false;
    };
    match name.as_str() {
        "data1.hdr" => true,
        "data1.cab" => sibling(path, "data1.hdr").is_none(),
        _ => false,
    }
}

/// The file called `name` (any case) in the same folder as `path`.
fn sibling(path: &Path, name: &str) -> Option<PathBuf> {
    let dir = path.parent()?;
    fs::read_dir(dir).ok()?.flatten().find_map(|entry| {
        let file = entry.file_name().to_string_lossy().to_ascii_lowercase();
        (file == name).then(|| entry.path())
    })
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn u64_at(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

fn c_string(b: &[u8], at: usize) -> Option<String> {
    let rest = b.get(at..)?;
    let end = rest.iter().position(|&c| c == 0)?;
    Some(rest[..end].iter().map(|&c| c as char).collect())
}

/// The format's major version, from the version field of the common header.
fn major_version(version: u32) -> u32 {
    match version >> 24 {
        1 => (version >> 12) & 0xf,
        2 | 4 => (version & 0xffff) / 100,
        _ => 0,
    }
}

/// One file listed in the header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsFile {
    pub index: usize,
    pub directory: String,
    pub name: String,
    flags: u16,
    expanded: u64,
    compressed: u64,
    offset: u64,
    /// The volume holding the file (version 6); 0 when unknown.
    volume: u16,
}

/// The files a header lists, in order. `None` when it is not a header this
/// module understands.
pub fn list(header: &[u8]) -> Option<Vec<IsFile>> {
    if !is_installshield(header) {
        return None;
    }
    let major = match major_version(u32_at(header, 4)?) {
        0 => 5,
        m => m,
    };
    let descriptor = u32_at(header, 12)? as usize;
    let table = descriptor + u32_at(header, descriptor + 0x0c)? as usize;
    let directories = u32_at(header, descriptor + 0x1c)? as usize;
    let count = u32_at(header, descriptor + 0x28)? as usize;
    let table2 = u32_at(header, descriptor + 0x2c)? as usize;
    if directories > MOST_FILES || count > MOST_FILES {
        return None;
    }
    let directory_names: Vec<String> = (0..directories)
        .map(|i| {
            u32_at(header, table + 4 * i)
                .and_then(|at| c_string(header, table + at as usize))
                .unwrap_or_default()
        })
        .collect();
    let mut files = Vec::new();
    for index in 0..count {
        let (name_at, directory, flags, expanded, compressed, offset, volume);
        if major <= 5 {
            let Some(at) = u32_at(header, table + 4 * (directories + index)) else {
                continue;
            };
            let p = table + at as usize;
            let (Some(n), Some(d), Some(f), Some(e), Some(c), Some(o)) = (
                u32_at(header, p),
                u32_at(header, p + 4),
                u16_at(header, p + 8),
                u32_at(header, p + 10),
                u32_at(header, p + 14),
                u32_at(header, p + 0x26),
            ) else {
                continue;
            };
            (
                name_at, directory, flags, expanded, compressed, offset, volume,
            ) = (n, d as usize, f, e as u64, c as u64, o as u64, 0);
        } else {
            let p = table + table2 + index * 0x57;
            let (Some(f), Some(e), Some(c), Some(o), Some(n), Some(d), Some(v)) = (
                u16_at(header, p),
                u64_at(header, p + 2),
                u64_at(header, p + 10),
                u64_at(header, p + 18),
                u32_at(header, p + 0x3a),
                u16_at(header, p + 0x3e),
                u16_at(header, p + 0x55),
            ) else {
                continue;
            };
            (
                name_at, directory, flags, expanded, compressed, offset, volume,
            ) = (n, d as usize, f, e, c, o, v);
        }
        if flags & FILE_INVALID != 0 || name_at == 0 {
            continue;
        }
        let Some(name) = c_string(header, table + name_at as usize) else {
            continue;
        };
        files.push(IsFile {
            index,
            directory: directory_names.get(directory).cloned().unwrap_or_default(),
            name,
            flags,
            expanded,
            compressed,
            offset,
            volume,
        });
    }
    Some(files)
}

/// The first and last file index a volume holds.
fn volume_range(volume: &[u8]) -> Option<(usize, usize)> {
    // Both layouts put them at the same place after the common header.
    Some((u32_at(volume, 28)? as usize, u32_at(volume, 32)? as usize))
}

/// Unpack one file from the volume holding it. `None` for files this module
/// skips (split, obfuscated) or whose data does not add up.
pub fn unpack(file: &IsFile, volume: &[u8], limit: u64) -> Option<Vec<u8>> {
    if file.flags & (FILE_SPLIT | FILE_OBFUSCATED) != 0 || file.expanded > limit {
        return None;
    }
    let start = usize::try_from(file.offset).ok()?;
    let end = start.checked_add(usize::try_from(file.compressed).ok()?)?;
    let raw = volume.get(start..end)?;
    let expanded = file.expanded as usize;
    if file.flags & FILE_COMPRESSED == 0 {
        return (raw.len() == expanded).then(|| raw.to_vec());
    }
    let mut out = Vec::with_capacity(expanded);
    let mut at = 0;
    while at + 2 <= raw.len() && out.len() < expanded {
        let length = u16_at(raw, at)? as usize;
        at += 2;
        let chunk = raw.get(at..at + length)?;
        at += length;
        let mut decoder = flate2::read::DeflateDecoder::new(chunk);
        let room = (expanded - out.len()) as u64;
        decoder.by_ref().take(room).read_to_end(&mut out).ok()?;
    }
    (out.len() == expanded).then_some(out)
}

/// A name safe to create under the output folder: no separators, no drive
/// letters, no `..`, no InstallShield `<TARGETDIR>`-style placeholders.
fn safe_component(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || " ._-()".contains(c) {
                c
            } else {
                '_'
            }
        })
        .collect();
    let trimmed = cleaned.trim_matches(|c: char| c == '.' || c == ' ');
    if trimmed.is_empty() {
        "_".to_string()
    } else {
        trimmed.to_string()
    }
}

/// Write out every file of the InstallShield cabinets whose header is
/// `header_path`, under `out`. Returns the files written. `limit` caps each
/// file and the total.
pub fn extract(header_path: &Path, out: &Path, limit: u64) -> Vec<PathBuf> {
    let mut written = Vec::new();
    let Ok(header) = fs::read(header_path) else {
        return written;
    };
    let Some(files) = list(&header) else {
        return written;
    };
    // data1.cab, data2.cab, ... beside the header; read as they are needed.
    let mut volumes: Vec<Option<Option<Vec<u8>>>> = vec![None; VOLUMES + 1];
    let mut load = |n: usize| -> Option<Vec<u8>> {
        let slot = volumes.get_mut(n)?;
        if slot.is_none() {
            *slot =
                Some(sibling(header_path, &format!("data{n}.cab")).and_then(|p| fs::read(p).ok()));
        }
        slot.clone().flatten()
    };
    let mut total = 0u64;
    for file in &files {
        let volume = if file.volume > 0 {
            load(file.volume as usize)
        } else {
            (1..=VOLUMES).map_while(&mut load).find(|v| {
                volume_range(v)
                    .is_some_and(|(first, last)| first <= file.index && file.index <= last)
            })
        };
        let Some(data) = volume.and_then(|v| unpack(file, &v, limit)) else {
            continue;
        };
        total += data.len() as u64;
        if total > limit {
            break;
        }
        let mut target = out.to_path_buf();
        for part in file.directory.split(['\\', '/']).filter(|p| !p.is_empty()) {
            target.push(safe_component(part));
        }
        target.push(safe_component(&file.name));
        if let Some(parent) = target.parent() {
            let _ = fs::create_dir_all(parent);
        }
        // Two files can share a name in different components; keep the first.
        if !target.exists() && fs::write(&target, data).is_ok() {
            written.push(target);
        }
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A version-5 header (common header, descriptor, file table, names) and
    /// its volume, holding one stored and one compressed file.
    fn sample() -> (Vec<u8>, Vec<u8>) {
        let stored = b"plain bytes".to_vec();
        let text = b"Game.arm.CAB contents ".repeat(20);
        let mut deflated =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        deflated.write_all(&text).unwrap();
        let deflated = deflated.finish().unwrap();
        let mut chunked = (deflated.len() as u16).to_le_bytes().to_vec();
        chunked.extend_from_slice(&deflated);

        // Volume: common header (20) + volume header, then the data.
        let mut volume = vec![0u8; 64];
        volume[..4].copy_from_slice(SIGNATURE);
        volume[28..32].copy_from_slice(&0u32.to_le_bytes());
        volume[32..36].copy_from_slice(&1u32.to_le_bytes());
        let stored_at = volume.len() as u32;
        volume.extend_from_slice(&stored);
        let packed_at = volume.len() as u32;
        volume.extend_from_slice(&chunked);

        // Header: descriptor at 0x20; file table at descriptor + 0x100.
        let descriptor = 0x20usize;
        let table = descriptor + 0x100;
        let mut h = vec![0u8; table];
        h[..4].copy_from_slice(SIGNATURE);
        h[4..8].copy_from_slice(&0x0100_5000u32.to_le_bytes()); // major 5
        h[12..16].copy_from_slice(&(descriptor as u32).to_le_bytes());
        h[descriptor + 0x0c..descriptor + 0x10].copy_from_slice(&0x100u32.to_le_bytes());
        h[descriptor + 0x1c..descriptor + 0x20].copy_from_slice(&1u32.to_le_bytes());
        h[descriptor + 0x28..descriptor + 0x2c].copy_from_slice(&2u32.to_le_bytes());
        // Table: one directory offset, two descriptor offsets (relative).
        let mut t = vec![0u8; 12];
        let dir_name = t.len() + 0x60;
        t[0..4].copy_from_slice(&(dir_name as u32).to_le_bytes());
        let fd = |name_at: u32, flags: u16, expanded: u32, compressed: u32, offset: u32| {
            let mut d = vec![0u8; 0x2a];
            d[0..4].copy_from_slice(&name_at.to_le_bytes());
            d[8..10].copy_from_slice(&flags.to_le_bytes());
            d[10..14].copy_from_slice(&expanded.to_le_bytes());
            d[14..18].copy_from_slice(&compressed.to_le_bytes());
            d[0x26..0x2a].copy_from_slice(&offset.to_le_bytes());
            d
        };
        let names_at = 0x80u32;
        let first = fd(
            names_at,
            0,
            stored.len() as u32,
            stored.len() as u32,
            stored_at,
        );
        let second = fd(
            names_at + 16,
            FILE_COMPRESSED,
            text.len() as u32,
            chunked.len() as u32,
            packed_at,
        );
        t[4..8].copy_from_slice(&12u32.to_le_bytes());
        t[8..12].copy_from_slice(&(12 + 0x2a_u32).to_le_bytes());
        t.extend_from_slice(&first);
        t.extend_from_slice(&second);
        t.resize(dir_name, 0);
        t.extend_from_slice(b"..\\Program\0");
        t.resize(names_at as usize, 0);
        t.extend_from_slice(b"readme.txt\0");
        t.resize(names_at as usize + 16, 0);
        t.extend_from_slice(b"Game.arm.CAB\0");
        h.extend_from_slice(&t);
        (h, volume)
    }

    #[test]
    fn lists_and_unpacks_stored_and_compressed_files() {
        let (header, volume) = sample();
        let files = list(&header).unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name, "readme.txt");
        assert_eq!(files[1].name, "Game.arm.CAB");
        assert_eq!(files[1].directory, "..\\Program");
        assert_eq!(unpack(&files[0], &volume, 1 << 20).unwrap(), b"plain bytes");
        assert_eq!(
            unpack(&files[1], &volume, 1 << 20).unwrap(),
            b"Game.arm.CAB contents ".repeat(20)
        );
    }

    #[test]
    fn extracts_beside_the_header_without_leaving_the_folder() {
        let (header, volume) = sample();
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("in");
        fs::create_dir_all(&src).unwrap();
        fs::write(src.join("DATA1.HDR"), &header).unwrap();
        fs::write(src.join("data1.cab"), &volume).unwrap();
        assert!(is_header_file(&src.join("DATA1.HDR")));
        assert!(!is_header_file(&src.join("data1.cab")));
        let out = dir.path().join("out");
        let written = extract(&src.join("DATA1.HDR"), &out, 1 << 20);
        assert_eq!(written.len(), 2);
        for path in &written {
            assert!(path.starts_with(&out), "{path:?} escaped the output folder");
        }
        assert!(written.iter().any(|p| p.ends_with("Game.arm.CAB")));
    }

    #[test]
    fn damaged_headers_are_refused_not_trusted() {
        let (mut header, volume) = sample();
        assert!(list(b"MSCF\0\0\0\0").is_none());
        // A descriptor offset past the end.
        header[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(list(&header).is_none());
        // A compressed size past the end of the volume.
        let (header, _) = sample();
        let mut files = list(&header).unwrap();
        files[1].compressed = u64::MAX / 2;
        assert!(unpack(&files[1], &volume, 1 << 20).is_none());
        // A file larger than the limit.
        let files = list(&header).unwrap();
        assert!(unpack(&files[1], &volume, 8).is_none());
    }
}
