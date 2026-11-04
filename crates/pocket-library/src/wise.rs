//! Pulling files out of Wise installers.
//!
//! Many Pocket PC downloads are desktop setup programs built with Wise
//! InstallMaster. After the program itself (its PE sections) comes the data:
//! the install script and every file, each stored as a raw deflate stream
//! followed by the CRC-32 of what it holds. The device cabinets are among
//! those files.
//!
//! Reading the script would tell where each file starts, but its layout
//! changed between Wise versions. Instead this looks for the first stream
//! after the program whose CRC matches, then follows the chain: the next
//! stream starts right after the CRC of the one before. A CRC match is what
//! makes a stream real; a lucky decode of random bytes does not carry one.
//! Nothing here runs the installer.

use flate2::{Crc, Decompress, FlushDecompress, Status};

/// Whether `bytes` look like a Wise installer: a Windows program carrying
/// Wise's own name.
pub fn is_wise(bytes: &[u8]) -> bool {
    bytes.starts_with(b"MZ")
        && overlay_start(bytes).is_some_and(|start| start < bytes.len())
        && [&b"WiseMain"[..], b"Wise Installation"]
            .iter()
            .any(|marker| bytes.windows(marker.len()).any(|w| w == *marker))
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// Where the data appended to a PE program starts: the end of its last
/// section in the file.
fn overlay_start(bytes: &[u8]) -> Option<usize> {
    let pe = u32_at(bytes, 0x3c)? as usize;
    if bytes.get(pe..pe + 4)? != b"PE\0\0" {
        return None;
    }
    let sections = u16_at(bytes, pe + 6)? as usize;
    let optional = u16_at(bytes, pe + 20)? as usize;
    let table = pe + 24 + optional;
    let mut end = 0usize;
    for i in 0..sections.min(96) {
        let entry = table + 40 * i;
        let size = u32_at(bytes, entry + 16)? as usize;
        let at = u32_at(bytes, entry + 20)? as usize;
        end = end.max(at.saturating_add(size));
    }
    Some(end)
}

/// Inflate the raw deflate stream starting at `at`. Returns what it holds
/// and where it ends, or `None` when it is not a complete stream or would
/// grow past `limit`.
fn stream_at(bytes: &[u8], at: usize, limit: usize) -> Option<(Vec<u8>, usize)> {
    let input = bytes.get(at..)?;
    let mut inflate = Decompress::new(false);
    let mut out = Vec::with_capacity(4096);
    loop {
        if out.len() == out.capacity() {
            if out.len() >= limit {
                return None;
            }
            out.reserve(out.len().clamp(4096, 1 << 20));
        }
        let consumed = inflate.total_in() as usize;
        let produced = out.len();
        let status = inflate
            .decompress_vec(&input[consumed..], &mut out, FlushDecompress::None)
            .ok()?;
        if status == Status::StreamEnd {
            return Some((out, at + inflate.total_in() as usize));
        }
        // Out of input before the stream ended, or no progress at all: not a
        // complete stream.
        let stalled = inflate.total_in() as usize == consumed && out.len() == produced;
        if stalled || (inflate.total_in() as usize >= input.len() && out.len() < out.capacity()) {
            return None;
        }
    }
}

/// The files stored in a Wise installer, in order. `tries` caps how many
/// positions are tried while looking for a stream; `limit` caps each file
/// and all of them together.
pub fn files(bytes: &[u8], tries: usize, limit: usize) -> Vec<Vec<u8>> {
    let mut found = Vec::new();
    let Some(mut at) = overlay_start(bytes) else {
        return found;
    };
    let mut left = tries;
    let mut total = 0usize;
    while at + 4 < bytes.len() && left > 0 {
        if let Some((data, end)) = stream_at(bytes, at, limit - total) {
            let mut crc = Crc::new();
            crc.update(&data);
            if !data.is_empty() && u32_at(bytes, end) == Some(crc.sum()) {
                total += data.len();
                found.push(data);
                if total >= limit {
                    break;
                }
                at = end + 4;
                continue;
            }
        }
        at += 1;
        left -= 1;
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn deflate(data: &[u8]) -> Vec<u8> {
        let mut encoder =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }

    /// A program of one section followed by Wise-style data: some script
    /// bytes, then two stored files, each a deflate stream and its CRC.
    fn installer(payloads: &[&[u8]]) -> Vec<u8> {
        let mut b = vec![0u8; 0x200];
        b[..2].copy_from_slice(b"MZ");
        b[0x3c..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        b[0x80..0x84].copy_from_slice(b"PE\0\0");
        b[0x84..0x86].copy_from_slice(&0x014cu16.to_le_bytes());
        b[0x86..0x88].copy_from_slice(&1u16.to_le_bytes());
        b[0x94..0x96].copy_from_slice(&0u16.to_le_bytes()); // no optional header
                                                            // Section table at 0x98: raw size 0x100 at 0x100.
        b[0x98 + 16..0x98 + 20].copy_from_slice(&0x100u32.to_le_bytes());
        b[0x98 + 20..0x98 + 24].copy_from_slice(&0x100u32.to_le_bytes());
        b[0x150..0x158].copy_from_slice(b"WiseMain");
        b.extend_from_slice(b"\x01\x02script\xff");
        for payload in payloads {
            b.extend_from_slice(&deflate(payload));
            let mut crc = Crc::new();
            crc.update(payload);
            b.extend_from_slice(&crc.sum().to_le_bytes());
        }
        b
    }

    #[test]
    fn follows_the_chain_of_stored_files() {
        let cab = b"MSCF\0\0\0\0 a device cabinet ".repeat(30);
        let bytes = installer(&[&cab, b"readme text"]);
        assert!(is_wise(&bytes));
        let found = files(&bytes, 1000, 1 << 20);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0], cab);
        assert_eq!(found[1], b"readme text");
    }

    #[test]
    fn a_wrong_crc_or_a_size_limit_keeps_a_stream_out() {
        let mut bytes = installer(&[b"payload that matters"]);
        let last = bytes.len() - 1;
        bytes[last] ^= 0xff;
        assert!(files(&bytes, 1000, 1 << 20).is_empty());
        let big = vec![7u8; 100_000];
        let bytes = installer(&[&big]);
        assert!(files(&bytes, 1000, 1000).is_empty());
        assert_eq!(files(&bytes, 1000, 1 << 20).len(), 1);
    }

    #[test]
    fn other_programs_are_not_wise() {
        assert!(!is_wise(b"MZ not a program"));
        let mut bytes = installer(&[b"x"]);
        bytes[0x150..0x158].copy_from_slice(b"Nullsoft");
        assert!(!is_wise(&bytes));
    }
}
