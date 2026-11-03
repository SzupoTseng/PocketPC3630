//! Reading InstallShield 3 archives (`data.z`, `files.z`).
//!
//! Setup programs made with InstallShield 3 keep everything they install in
//! one archive that starts with `13 5D 65 8C`. Pocket PC downloads from
//! 1999-2001 often ship the device cabinets (`game.PPC_ARM.CAB`, ...) this
//! way. Each file is compressed with PKWARE's Data Compression Library
//! ("implode"); [`blast`] undoes that, after Mark Adler's `blast.c` in
//! zlib's contrib folder.
//!
//! Archive layout (all little-endian):
//! - header: file count (u16) at 0x0C, archive size (u32) at 0x12, table
//!   offset (u32) at 0x29, directory count (u16) at 0x31;
//! - at the table, one entry per directory: file count (u16), entry size
//!   (u16), name length (u16), name;
//! - then one entry per file: directory index (u16) at 1, size (u32) at 3,
//!   compressed size (u32) at 7, data offset (u32) at 11, entry size (u16)
//!   at 0x17, name length (u8) at 0x1D, name at 0x1E.
//!
//! Nothing here runs anything.

use std::fs;
use std::path::{Path, PathBuf};

const SIGNATURE: [u8; 4] = [0x13, 0x5D, 0x65, 0x8C];
const MAX_BITS: usize = 13;

/// Whether `bytes` start an InstallShield 3 archive.
pub fn is_is3(bytes: &[u8]) -> bool {
    bytes.starts_with(&SIGNATURE)
}

/// The InstallShield 3 archives stored whole inside `bytes`, such as the
/// ones a self-extracting setup program carries. Each starts with the
/// signature and declares its own size at 0x12.
pub fn carve(bytes: &[u8]) -> Vec<&[u8]> {
    let mut found = Vec::new();
    let mut at = 1;
    while let Some(offset) = bytes
        .get(at..)
        .and_then(|rest| rest.windows(4).position(|w| w == SIGNATURE))
    {
        let start = at + offset;
        let size = u32_at(bytes, start + 0x12).unwrap_or(0) as usize;
        if size > 0x100 && size <= bytes.len() - start {
            let archive = &bytes[start..start + size];
            if list(archive).is_some_and(|files| !files.is_empty()) {
                found.push(archive);
                at = start + size;
                continue;
            }
        }
        at = start + 4;
    }
    found
}

/// A canonical Huffman code given as counts per length and symbols in order.
struct Huffman {
    count: [u16; MAX_BITS + 1],
    symbol: Vec<u16>,
}

impl Huffman {
    /// Build from blast's compact form: each byte is a code length (low four
    /// bits) repeated (high four bits + 1) times.
    fn from_compact(compact: &[u8]) -> Self {
        let mut lengths = Vec::new();
        for &byte in compact {
            let repeat = usize::from(byte >> 4) + 1;
            lengths.extend(std::iter::repeat_n(byte & 15, repeat));
        }
        let mut count = [0u16; MAX_BITS + 1];
        for &length in &lengths {
            count[usize::from(length)] += 1;
        }
        let mut offset = [0usize; MAX_BITS + 1];
        for length in 1..MAX_BITS {
            offset[length + 1] = offset[length] + usize::from(count[length]);
        }
        let mut symbol = vec![0u16; lengths.len()];
        for (value, &length) in lengths.iter().enumerate() {
            if length != 0 {
                symbol[offset[usize::from(length)]] = value as u16;
                offset[usize::from(length)] += 1;
            }
        }
        Huffman { count, symbol }
    }
}

// The fixed codes of the format, as in blast.c.
const LITERAL_LENGTHS: [u8; 98] = [
    11, 124, 8, 7, 28, 7, 188, 13, 76, 4, 10, 8, 12, 10, 12, 10, 8, 23, 8, 9, 7, 6, 7, 8, 7, 6, 55,
    8, 23, 24, 12, 11, 7, 9, 11, 12, 6, 7, 22, 5, 7, 24, 6, 11, 9, 6, 7, 22, 7, 11, 38, 7, 9, 8,
    25, 11, 8, 11, 9, 12, 8, 12, 5, 38, 5, 38, 5, 11, 7, 5, 6, 21, 6, 10, 53, 8, 7, 24, 10, 27, 44,
    253, 253, 253, 252, 252, 252, 13, 12, 45, 12, 45, 12, 61, 12, 45, 44, 173,
];
const LENGTH_LENGTHS: [u8; 6] = [2, 35, 36, 53, 38, 23];
const DISTANCE_LENGTHS: [u8; 7] = [2, 20, 53, 230, 247, 151, 248];
const LENGTH_BASE: [u16; 16] = [3, 2, 4, 5, 6, 7, 8, 9, 10, 12, 16, 24, 40, 72, 136, 264];
const LENGTH_EXTRA: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8];

struct Bits<'a> {
    data: &'a [u8],
    at: usize,
    buffer: u32,
    count: u32,
}

impl Bits<'_> {
    fn take(&mut self, need: u32) -> Option<u32> {
        while self.count < need {
            let byte = *self.data.get(self.at)?;
            self.at += 1;
            self.buffer |= u32::from(byte) << self.count;
            self.count += 8;
        }
        let value = self.buffer & ((1u32 << need) - 1);
        self.buffer >>= need;
        self.count -= need;
        Some(value)
    }

    /// The format stores codes with their bits inverted.
    fn decode(&mut self, code: &Huffman) -> Option<u16> {
        let (mut value, mut first, mut index) = (0usize, 0usize, 0usize);
        for length in 1..=MAX_BITS {
            value |= (self.take(1)? ^ 1) as usize;
            let count = usize::from(code.count[length]);
            if value < first + count {
                return code.symbol.get(index + value - first).copied();
            }
            index += count;
            first = (first + count) << 1;
            value <<= 1;
        }
        None
    }
}

/// Decompress PKWARE DCL ("implode") data. `None` when the data is damaged or
/// the output would pass `limit` bytes.
pub fn blast(data: &[u8], limit: usize) -> Option<Vec<u8>> {
    let literal = Huffman::from_compact(&LITERAL_LENGTHS);
    let length_code = Huffman::from_compact(&LENGTH_LENGTHS);
    let distance_code = Huffman::from_compact(&DISTANCE_LENGTHS);
    let mut bits = Bits {
        data,
        at: 0,
        buffer: 0,
        count: 0,
    };
    let coded = bits.take(8)?;
    let dictionary = bits.take(8)?;
    if coded > 1 || !(4..=6).contains(&dictionary) {
        return None;
    }
    let mut out = Vec::new();
    loop {
        if bits.take(1)? == 1 {
            let symbol = usize::from(bits.decode(&length_code)?);
            let length = usize::from(LENGTH_BASE[symbol])
                + bits.take(u32::from(LENGTH_EXTRA[symbol]))? as usize;
            if length == 519 {
                return Some(out);
            }
            let shift = if length == 2 { 2 } else { dictionary };
            let distance = ((usize::from(bits.decode(&distance_code)?)) << shift)
                + bits.take(shift)? as usize
                + 1;
            if distance > out.len() || out.len() + length > limit {
                return None;
            }
            let start = out.len() - distance;
            for i in 0..length {
                out.push(out[start + i]);
            }
        } else {
            let byte = if coded == 1 {
                bits.decode(&literal)? as u8
            } else {
                bits.take(8)? as u8
            };
            if out.len() >= limit {
                return None;
            }
            out.push(byte);
        }
    }
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// One file listed in an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Is3File {
    pub directory: String,
    pub name: String,
    pub size: u32,
    compressed: u32,
    offset: u32,
}

/// The files an archive lists. `None` when it is not an archive this module
/// understands.
pub fn list(bytes: &[u8]) -> Option<Vec<Is3File>> {
    if !is_is3(bytes) {
        return None;
    }
    let count = usize::from(u16_at(bytes, 0x0C)?);
    let table = u32_at(bytes, 0x29)? as usize;
    let directories = usize::from(u16_at(bytes, 0x31)?);
    let name = |at: usize, length: usize| -> Option<String> {
        Some(
            bytes
                .get(at..at + length)?
                .iter()
                .map(|&c| c as char)
                .collect(),
        )
    };
    let mut at = table;
    let mut directory_names = Vec::with_capacity(directories);
    for _ in 0..directories {
        let size = usize::from(u16_at(bytes, at + 2)?);
        let length = usize::from(u16_at(bytes, at + 4)?);
        directory_names.push(name(at + 6, length)?);
        if size == 0 {
            return None;
        }
        at += size;
    }
    let mut files = Vec::with_capacity(count);
    for _ in 0..count {
        let directory = usize::from(u16_at(bytes, at + 1)?);
        let size = u32_at(bytes, at + 3)?;
        let compressed = u32_at(bytes, at + 7)?;
        let offset = u32_at(bytes, at + 11)?;
        let entry = usize::from(u16_at(bytes, at + 0x17)?);
        let length = usize::from(*bytes.get(at + 0x1D)?);
        if entry == 0 {
            return None;
        }
        files.push(Is3File {
            directory: directory_names.get(directory).cloned().unwrap_or_default(),
            name: name(at + 0x1E, length)?,
            size,
            compressed,
            offset,
        });
        at += entry;
    }
    Some(files)
}

/// Decompress one file. `None` when its data is damaged or larger than
/// `limit`.
pub fn unpack(bytes: &[u8], file: &Is3File, limit: usize) -> Option<Vec<u8>> {
    if file.size as usize > limit {
        return None;
    }
    let start = file.offset as usize;
    let data = bytes.get(start..start.checked_add(file.compressed as usize)?)?;
    let out = blast(data, file.size as usize)?;
    (out.len() == file.size as usize).then_some(out)
}

/// A name safe to create under the output folder.
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

/// Write out every file of the archive at `path` under `out`. `limit` caps
/// each file and the total.
pub fn extract(path: &Path, out: &Path, limit: u64) -> Vec<PathBuf> {
    let mut written = Vec::new();
    let Ok(bytes) = fs::read(path) else {
        return written;
    };
    let Some(files) = list(&bytes) else {
        return written;
    };
    let mut total = 0u64;
    for file in &files {
        let Some(data) = unpack(&bytes, file, limit as usize) else {
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
        if !target.exists() && fs::write(&target, data).is_ok() {
            written.push(target);
        }
    }
    written
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example from blast.c: "AIAIAIAIAIAIA" imploded.
    const BLAST_EXAMPLE: [u8; 9] = [0x00, 0x04, 0x82, 0x24, 0x25, 0x8f, 0x80, 0x7f, 0x00];

    #[test]
    fn blast_decodes_the_reference_example() {
        assert_eq!(blast(&BLAST_EXAMPLE, 1 << 10).unwrap(), b"AIAIAIAIAIAIA");
        // Cut short, or over the limit, is refused.
        assert!(blast(&BLAST_EXAMPLE[..5], 1 << 10).is_none());
        assert!(blast(&BLAST_EXAMPLE, 4).is_none());
        assert!(blast(&[0x07, 0x04], 1 << 10).is_none());
    }

    /// An archive holding one directory and one file, `BLAST_EXAMPLE`.
    fn archive() -> Vec<u8> {
        let mut b = vec![0u8; 0xFF];
        b[..4].copy_from_slice(&SIGNATURE);
        b[0x0C..0x0E].copy_from_slice(&1u16.to_le_bytes());
        b[0x31..0x33].copy_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&BLAST_EXAMPLE);
        let table = b.len() as u32;
        b[0x29..0x2D].copy_from_slice(&table.to_le_bytes());
        // Directory "..\game": entry of 6 + 7 bytes.
        b.extend_from_slice(&1u16.to_le_bytes());
        b.extend_from_slice(&13u16.to_le_bytes());
        b.extend_from_slice(&7u16.to_le_bytes());
        b.extend_from_slice(b"..\\game");
        let mut entry = vec![0u8; 0x1E];
        entry[3..7].copy_from_slice(&13u32.to_le_bytes());
        entry[7..11].copy_from_slice(&(BLAST_EXAMPLE.len() as u32).to_le_bytes());
        entry[11..15].copy_from_slice(&0xFFu32.to_le_bytes());
        entry[0x17..0x19].copy_from_slice(&(0x1E + 8u16).to_le_bytes());
        entry[0x1D] = 8;
        entry.extend_from_slice(b"game.cab");
        b.extend_from_slice(&entry);
        b
    }

    #[test]
    fn lists_and_extracts_inside_the_output_folder() {
        let bytes = archive();
        let files = list(&bytes).unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "game.cab");
        assert_eq!(
            unpack(&bytes, &files[0], 1 << 10).unwrap(),
            b"AIAIAIAIAIAIA"
        );
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("data.z"), &bytes).unwrap();
        let out = dir.path().join("out");
        let written = extract(&dir.path().join("data.z"), &out, 1 << 20);
        assert_eq!(written.len(), 1);
        assert!(written[0].starts_with(&out));
    }

    #[test]
    fn archives_inside_a_program_are_carved() {
        let mut program = b"MZ setup program ".to_vec();
        let mut one = archive();
        let size = one.len() as u32;
        one[0x12..0x16].copy_from_slice(&size.to_le_bytes());
        program.extend_from_slice(&one);
        program.extend_from_slice(b"trailing bytes");
        let carved = carve(&program);
        assert_eq!(carved.len(), 1);
        assert_eq!(carved[0], &one[..]);
        // A signature with a size past the end is not an archive.
        assert!(carve(&program[..program.len() - 20]).is_empty());
    }

    #[test]
    fn damaged_archives_are_refused() {
        let mut bytes = archive();
        assert!(list(b"PK\x03\x04").is_none());
        let table = bytes.len() - 0x26;
        bytes.truncate(table);
        assert!(list(&bytes).is_none());
    }
}
