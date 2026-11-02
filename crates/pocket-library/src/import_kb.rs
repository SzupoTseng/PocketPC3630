//! The import knowledge base, and the engine that uses it to get a game out of a
//! download that does not import as it is.
//!
//! Pocket PC software reached players wrapped in desktop installers, zips of one
//! cabinet per processor, self-extracting archives and the like. Instead of code
//! for each wrapper, the engine knows a few primitive ways to look inside a file
//! (open it as a zip, cut cabinets or zips out of it) and the knowledge base
//! says which to use on what, and how to rank what comes out. When no rule fits,
//! the engine tries every primitive; if that finds a game, the path that worked
//! is written to the library's own `import_kb.json`, so the next download of the
//! same kind goes straight to it. Nothing in a download is ever executed.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::import_problem::Problem;
use crate::LibraryError;

/// The file in a library root that extends the built-in knowledge base.
pub const USER_KB_FILE: &str = "import_kb.json";

const BUILTIN: &str = include_str!("../data/import_kb.json");
/// How deep wrappers may nest: installer in a zip in a zip.
const MAX_DEPTH: usize = 3;
/// Downloads are small; anything bigger than this is not looked inside.
const LARGEST_FILE: u64 = 128 * 1024 * 1024;
/// A bound on how many files exploring one import may look at.
const MOST_FILES: usize = 4000;
/// A candidate scored at or below this is never imported: desktop installers'
/// own cabinets, x86 programs.
const NEVER_IMPORT: i32 = -150;
/// A bound on how much exploring one import may write, against zip bombs.
const MOST_BYTES: u64 = 1024 * 1024 * 1024;
/// A bound on how long exploring may take before giving up.
const LONGEST: std::time::Duration = std::time::Duration::from_secs(60);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Exact downloads already seen, each with the steps that get its game out.
    #[serde(default)]
    pub known: Vec<Known>,
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub ranking: Ranking,
    #[serde(default)]
    pub problems: Vec<ProblemEntry>,
}

/// One download, recognised by its fingerprint, and how to get its game out:
/// each step names the action and the file it produced to carry on with.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Known {
    pub fingerprint: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Step {
    pub action: Action,
    /// The produced file to continue with, relative to what the action wrote.
    pub take: String,
}

/// When a file looks like `when`, look inside it with `actions`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
    #[serde(default)]
    pub when: Matcher,
    pub actions: Vec<Action>,
}

/// Every field given must hold; an empty matcher matches nothing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Matcher {
    /// Hex of the file's first bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magic: Option<String>,
    /// Hex sequences that must all occur somewhere in the file.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contains_hex: Vec<String>,
    /// The file is a PE for one of these: `arm`, `mips`, `sh`, `x86`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pe_machine: Vec<String>,
    /// The lower-case file name contains one of these.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name_has: Vec<String>,
}

/// The primitive ways to look inside a file.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Read it as a zip, including one appended to a program.
    OpenZip,
    /// Cut out every cabinet stored in it.
    CarveCab,
    /// Cut out a zip stored in it.
    CarveZip,
    /// Copy a cabinet with its data checksums cleared, which the format allows
    /// and which makes a reader skip them. Cabinets edited after they were
    /// built, or slightly damaged, still install.
    ClearCabChecksums,
    /// Cut out and expand every file packed with Microsoft's `compress.exe`
    /// (`SZDD`), which many installers use for the cabinets they carry.
    ExpandSzdd,
    /// Write out every resource of a desktop program; installers keep their
    /// payload there.
    ExtractResources,
    /// Inflate a zlib stream that a file starts with.
    InflateZlib,
}

const ALL_ACTIONS: [Action; 7] = [
    Action::OpenZip,
    Action::CarveCab,
    Action::CarveZip,
    Action::ExpandSzdd,
    Action::ExtractResources,
    Action::InflateZlib,
    Action::ClearCabChecksums,
];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ranking {
    #[serde(default)]
    pub name: Vec<RankTerm>,
    #[serde(default)]
    pub machine: HashMap<String, i32>,
    /// Per kind of candidate: `cab` or `exe`.
    #[serde(default)]
    pub kind: HashMap<String, i32>,
    /// Executables smaller than this are usually launchers or stubs.
    #[serde(default)]
    pub tiny_exe: Option<TinyExe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TinyExe {
    pub below: u64,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankTerm {
    pub has: String,
    pub score: i32,
}

/// Explains failures whose message contains one of `match`, or games missing a
/// library whose name contains one of `when_missing`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemEntry {
    pub id: String,
    #[serde(default, rename = "match")]
    pub matches: Vec<String>,
    #[serde(default)]
    pub when_missing: Vec<String>,
    pub title: BTreeMap<String, String>,
    pub hint: BTreeMap<String, String>,
}

impl KnowledgeBase {
    /// The knowledge base shipped with PocketHLE.
    pub fn builtin() -> &'static KnowledgeBase {
        static KB: OnceLock<KnowledgeBase> = OnceLock::new();
        KB.get_or_init(|| serde_json::from_str(BUILTIN).expect("built-in import_kb.json is valid"))
    }

    /// The built-in knowledge base extended by `library_root/import_kb.json`.
    /// A user file that cannot be read is skipped, never fatal.
    pub fn load(library_root: &Path) -> KnowledgeBase {
        let mut kb = Self::builtin().clone();
        if let Some(user) = read_user_kb(library_root) {
            kb.extend_with(user);
        }
        kb
    }

    /// Put `user`'s entries in front, so they win over the built-in ones.
    pub fn extend_with(&mut self, user: KnowledgeBase) {
        let mut known = user.known;
        known.append(&mut self.known);
        self.known = known;
        let mut rules = user.rules;
        rules.append(&mut self.rules);
        self.rules = rules;
        let mut terms = user.ranking.name;
        terms.append(&mut self.ranking.name);
        self.ranking.name = terms;
        self.ranking.machine.extend(user.ranking.machine);
        self.ranking.kind.extend(user.ranking.kind);
        if user.ranking.tiny_exe.is_some() {
            self.ranking.tiny_exe = user.ranking.tiny_exe;
        }
        let mut problems = user.problems;
        problems.append(&mut self.problems);
        self.problems = problems;
    }

    /// Explain a failed import.
    pub fn explain_import(&self, error: &LibraryError) -> Problem {
        self.explain_text(&format!("{error:?} {error}"), &error.to_string(), &[])
    }

    /// Explain a failed start or a stop, given the libraries the game misses.
    pub fn explain(&self, message: &str, missing: &[String]) -> Problem {
        self.explain_text(message, message, missing)
    }

    fn explain_text(&self, haystack: &str, detail: &str, missing: &[String]) -> Problem {
        let haystack = haystack.to_lowercase();
        let entry = self.problems.iter().find(|entry| {
            if !entry.when_missing.is_empty() {
                return missing.iter().any(|name| {
                    entry.when_missing.iter().any(|part| name.contains(&part.to_lowercase()))
                });
            }
            entry.matches.iter().any(|part| haystack.contains(&part.to_lowercase()))
        });
        let (id, title, hint) = match entry {
            Some(entry) => (
                entry.id.clone(),
                entry.title.clone().into_iter().collect(),
                entry.hint.clone().into_iter().collect(),
            ),
            None => ("unknown".to_string(), Vec::new(), Vec::new()),
        };
        Problem::new(id, detail.to_string(), missing.to_vec(), title, hint)
    }

    /// The actions the rules give for a file, in rule order, without repeats.
    fn actions_for(&self, name: &str, bytes: &[u8]) -> Vec<Action> {
        let mut out = Vec::new();
        for rule in &self.rules {
            if rule.when.matches(name, bytes) {
                for action in &rule.actions {
                    if !out.contains(action) {
                        out.push(*action);
                    }
                }
            }
        }
        out
    }

    fn kind_score(&self, kind: &str) -> i32 {
        self.ranking.kind.get(kind).copied().unwrap_or(0)
    }

    /// How much a candidate looks like the build to import; below zero never.
    pub fn score(&self, path: &str, machine: Option<&str>) -> i32 {
        let lower = path.to_lowercase();
        let name: i32 = self
            .ranking
            .name
            .iter()
            .filter(|term| lower.contains(&term.has.to_lowercase()))
            .map(|term| term.score)
            .sum();
        name + machine.and_then(|m| self.ranking.machine.get(m)).copied().unwrap_or(0)
    }
}

impl Matcher {
    fn is_empty(&self) -> bool {
        self.magic.is_none()
            && self.contains_hex.is_empty()
            && self.pe_machine.is_empty()
            && self.name_has.is_empty()
    }

    pub fn matches(&self, name: &str, bytes: &[u8]) -> bool {
        if self.is_empty() {
            return false;
        }
        if let Some(magic) = &self.magic {
            match hex(magic) {
                Some(prefix) if bytes.starts_with(&prefix) => {}
                _ => return false,
            }
        }
        for sequence in &self.contains_hex {
            match hex(sequence) {
                Some(needle) if find(bytes, &needle, 0).is_some() => {}
                _ => return false,
            }
        }
        if !self.pe_machine.is_empty() {
            let machine = pe_machine(bytes);
            if !self.pe_machine.iter().any(|m| Some(m.as_str()) == machine) {
                return false;
            }
        }
        if !self.name_has.is_empty() {
            let lower = name.to_lowercase();
            if !self.name_has.iter().any(|part| lower.contains(&part.to_lowercase())) {
                return false;
            }
        }
        true
    }
}

fn read_user_kb(library_root: &Path) -> Option<KnowledgeBase> {
    let text = fs::read_to_string(library_root.join(USER_KB_FILE)).ok()?;
    match serde_json::from_str(&text) {
        Ok(kb) => Some(kb),
        Err(error) => {
            log::warn!("ignoring {USER_KB_FILE}: {error}");
            None
        }
    }
}

/// Add `rule` to the library's own knowledge base unless one like it is there.
pub fn learn(library_root: &Path, rule: Rule) -> io::Result<bool> {
    let mut user = read_user_kb(library_root).unwrap_or_default();
    if user.rules.iter().any(|known| known.when == rule.when) {
        return Ok(false);
    }
    user.rules.push(rule);
    let text = serde_json::to_string_pretty(&user).map_err(io::Error::other)?;
    fs::write(library_root.join(USER_KB_FILE), text)?;
    Ok(true)
}

fn hex(text: &str) -> Option<Vec<u8>> {
    let text: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    (text.len() % 2 == 0)
        .then(|| {
            (0..text.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&text[i..i + 2], 16).ok())
                .collect()
        })
        .flatten()
}

fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn find(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    (from..=haystack.len() - needle.len()).find(|&i| haystack[i..i + needle.len()] == *needle)
}

/// `arm`, `mips`, `sh` or `x86` for a PE image, from its COFF machine field.
pub fn pe_machine(bytes: &[u8]) -> Option<&'static str> {
    if bytes.get(..2)? != b"MZ" {
        return None;
    }
    let pe = u32::from_le_bytes(bytes.get(0x3C..0x40)?.try_into().ok()?) as usize;
    if bytes.get(pe..pe + 4)? != b"PE\0\0" {
        return None;
    }
    let machine = u16::from_le_bytes(bytes.get(pe + 4..pe + 6)?.try_into().ok()?);
    Some(match machine {
        0x01C0 | 0x01C2 | 0x01C4 => "arm",
        0x0162 | 0x0166 | 0x0168 | 0x0169 => "mips",
        0x01A2 | 0x01A3 | 0x01A4 | 0x01A6 => "sh",
        0x014C | 0x8664 => "x86",
        _ => return None,
    })
}

fn is_dll(bytes: &[u8]) -> bool {
    let Some(pe) = bytes
        .get(0x3C..0x40)
        .and_then(|b| b.try_into().ok())
        .map(|b: [u8; 4]| u32::from_le_bytes(b) as usize)
    else {
        return false;
    };
    bytes
        .get(pe + 22..pe + 24)
        .is_some_and(|c| u16::from_le_bytes([c[0], c[1]]) & 0x2000 != 0)
}

/// Every cabinet stored whole inside `bytes`.
fn carve_cabs(bytes: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(start) = find(bytes, b"MSCF\0\0\0\0", at) {
        let size = bytes
            .get(start + 8..start + 12)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
            .unwrap_or(0);
        let version_ok = bytes.get(start + 24..start + 26) == Some(&[3, 1]);
        if version_ok && size > 64 && start + size <= bytes.len() {
            out.push(&bytes[start..start + size]);
            at = start + size;
        } else {
            at = start + 4;
        }
    }
    out
}

/// A zip stored whole inside `bytes`, found from its end record.
fn carve_zip(bytes: &[u8]) -> Option<&[u8]> {
    let mut search = bytes.len();
    while search >= 22 {
        let end = (0..=search - 22).rev().find(|&i| bytes[i..i + 4] == *b"PK\x05\x06")?;
        let field = |offset: usize| -> Option<u32> {
            bytes.get(end + offset..end + offset + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        let comment = bytes
            .get(end + 20..end + 22)
            .map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)
            .unwrap_or(0);
        if let (Some(size), Some(offset)) = (field(12), field(16)) {
            let directory = end.checked_sub(size as usize);
            if let Some(start) = directory.and_then(|d| d.checked_sub(offset as usize)) {
                if bytes.get(start..start + 4) == Some(b"PK\x03\x04") {
                    return bytes.get(start..(end + 22 + comment).min(bytes.len()));
                }
            }
        }
        search = end;
    }
    None
}

/// A copy of a cabinet whose data blocks carry no checksum, or `None` when it
/// is not a cabinet or none of its checksums is set.
fn clear_cab_checksums(bytes: &[u8]) -> Option<Vec<u8>> {
    let u16_at = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);
    let u32_at =
        |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    if !bytes.starts_with(b"MSCF") {
        return None;
    }
    let folders = u16_at(26)?;
    let flags = u16_at(30)?;
    let mut at = 36;
    let (mut folder_reserve, mut data_reserve) = (0, 0);
    if flags & 4 != 0 {
        let header_reserve = u16_at(36)?;
        folder_reserve = *bytes.get(38)? as usize;
        data_reserve = *bytes.get(39)? as usize;
        at = 40 + header_reserve;
    }
    // Names of a previous and a next cabinet in a set, each two strings.
    for flag in [1, 2] {
        if flags & flag != 0 {
            for _ in 0..2 {
                at += bytes.get(at..)?.iter().position(|&b| b == 0)? + 1;
            }
        }
    }
    let mut out = bytes.to_vec();
    let mut changed = false;
    for folder in 0..folders {
        let entry = at + folder * (8 + folder_reserve);
        let mut block = u32_at(entry)?;
        let count = u16_at(entry + 4)?;
        for _ in 0..count {
            let size = u16_at(block + 4)?;
            if out.get(block..block + 4)? != [0, 0, 0, 0] {
                out[block..block + 4].fill(0);
                changed = true;
            }
            block += 8 + data_reserve + size;
        }
    }
    changed.then_some(out)
}

/// The two-byte header of a zlib stream: deflate, a 32 KiB window, and a check
/// that makes the pair a multiple of 31.
fn is_zlib(bytes: &[u8]) -> bool {
    matches!(bytes, [cmf, flg, ..] if cmf & 0x0F == 8 && cmf >> 4 <= 7 && ((*cmf as u16) << 8 | *flg as u16) % 31 == 0)
}

/// The data of every resource in a PE image, larger than a few bytes. Icons and
/// version records come out too; they are simply not importable.
fn pe_resources(bytes: &[u8]) -> Vec<&[u8]> {
    let read16 = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);
    let read32 =
        |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let mut out = Vec::new();
    let Some(pe) = read32(0x3C) else {
        return out;
    };
    if bytes.get(pe..pe + 4) != Some(b"PE\0\0") {
        return out;
    }
    let (Some(sections), Some(optional_size)) = (read16(pe + 6), read16(pe + 20)) else {
        return out;
    };
    let optional = pe + 24;
    let directories = match read16(optional) {
        Some(0x10B) => optional + 96,
        Some(0x20B) => optional + 112,
        _ => return out,
    };
    let Some(resource_rva) = read32(directories + 16) else {
        return out;
    };
    if resource_rva == 0 {
        return out;
    }
    let table = optional + optional_size;
    let to_offset = |rva: usize| -> Option<usize> {
        (0..sections).find_map(|i| {
            let at = table + i * 40;
            let (virtual_size, address, raw_size, raw) = (read32(at + 8)?, read32(at + 12)?, read32(at + 16)?, read32(at + 20)?);
            (address <= rva && rva < address + virtual_size.max(raw_size)).then(|| rva - address + raw)
        })
    };
    let Some(base) = to_offset(resource_rva) else {
        return out;
    };
    // Directories nest three deep: type, name, language. A visited set guards
    // against a crafted table that loops.
    let mut pending = vec![(0usize, 0usize)];
    let mut seen = std::collections::HashSet::new();
    while let Some((directory, depth)) = pending.pop() {
        if depth > 3 || !seen.insert(directory) || seen.len() > 10_000 {
            continue;
        }
        let at = base + directory;
        let (Some(named), Some(numbered)) = (read16(at + 12), read16(at + 14)) else {
            continue;
        };
        for entry in 0..named + numbered {
            let Some(target) = read32(at + 16 + entry * 8 + 4) else {
                continue;
            };
            if target & 0x8000_0000 != 0 {
                pending.push((target & 0x7FFF_FFFF, depth + 1));
            } else if let (Some(rva), Some(size)) = (read32(base + target), read32(base + target + 4)) {
                if let Some(start) = to_offset(rva) {
                    if size > 64 {
                        if let Some(data) = bytes.get(start..start + size) {
                            out.push(data);
                        }
                    }
                }
            }
        }
    }
    out
}

const SZDD_MAGIC: &[u8] = b"SZDD\x88\xf0\x27\x33";

/// Every `SZDD` stream in `bytes`, expanded. The format is a 14-byte header
/// (magic, method `A`, a character of the name, the expanded size) followed by
/// LZSS with a 4096-byte window that starts filled with spaces.
fn expand_szdd_all(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    let mut at = 0;
    while let Some(start) = find(bytes, SZDD_MAGIC, at) {
        at = start + SZDD_MAGIC.len();
        if bytes.get(start + 8) != Some(&b'A') {
            continue;
        }
        let Some(size) = bytes
            .get(start + 10..start + 14)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize)
        else {
            continue;
        };
        if size == 0 || size as u64 > LARGEST_FILE {
            continue;
        }
        if let Some(expanded) = expand_szdd(&bytes[start + 14..], size) {
            out.push(expanded);
        }
    }
    out
}

fn expand_szdd(data: &[u8], size: usize) -> Option<Vec<u8>> {
    let mut window = [b' '; 4096];
    let mut position = 4096 - 16;
    let mut out = Vec::with_capacity(size);
    let mut input = data.iter().copied();
    while out.len() < size {
        let flags = input.next()?;
        for bit in 0..8 {
            if out.len() >= size {
                break;
            }
            if flags & (1 << bit) != 0 {
                let byte = input.next()?;
                out.push(byte);
                window[position] = byte;
                position = (position + 1) & 4095;
            } else {
                let low = input.next()? as usize;
                let high = input.next()? as usize;
                let mut from = low | ((high & 0xF0) << 4);
                let length = (high & 0x0F) + 3;
                for _ in 0..length {
                    let byte = window[from];
                    out.push(byte);
                    window[position] = byte;
                    position = (position + 1) & 4095;
                    from = (from + 1) & 4095;
                }
            }
        }
    }
    out.truncate(size);
    Some(out)
}

/// Run `action` on `path`, writing what it finds under `out`.
fn apply(action: Action, path: &Path, bytes: &[u8], out: &Path) -> Vec<PathBuf> {
    let mut produced = Vec::new();
    let _ = fs::create_dir_all(out);
    match action {
        Action::OpenZip => {
            let Ok(file) = fs::File::open(path) else {
                return produced;
            };
            let Ok(mut zip) = zip::ZipArchive::new(file) else {
                return produced;
            };
            for index in 0..zip.len() {
                let Ok(mut entry) = zip.by_index(index) else {
                    continue;
                };
                let Some(name) = entry.enclosed_name().map(Path::to_path_buf) else {
                    continue;
                };
                if entry.is_dir() || entry.size() > LARGEST_FILE {
                    continue;
                }
                let target = out.join(name);
                if let Some(parent) = target.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                // The size a zip declares can lie; count what actually comes out.
                if let Ok(mut file) = fs::File::create(&target) {
                    let mut limited = io::Read::take(&mut entry, LARGEST_FILE);
                    if io::copy(&mut limited, &mut file).is_ok() {
                        produced.push(target);
                    }
                }
            }
        }
        Action::CarveCab => {
            let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            for (index, cab) in carve_cabs(bytes).into_iter().enumerate() {
                let target = out.join(format!("{stem}.{index}.cab"));
                if fs::write(&target, cab).is_ok() {
                    produced.push(target);
                }
            }
        }
        Action::ClearCabChecksums => {
            if let Some(cleared) = clear_cab_checksums(bytes) {
                let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                let target = out.join(format!("{stem}.cab"));
                if fs::write(&target, cleared).is_ok() {
                    produced.push(target);
                }
            }
        }
        Action::ExpandSzdd => {
            let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            for (index, expanded) in expand_szdd_all(bytes).into_iter().enumerate() {
                let target = out.join(format!("{stem}.{index}.bin"));
                if fs::write(&target, expanded).is_ok() {
                    produced.push(target);
                }
            }
        }
        Action::ExtractResources => {
            for (index, data) in pe_resources(bytes).into_iter().enumerate() {
                let target = out.join(format!("resource-{index}.bin"));
                if fs::write(&target, data).is_ok() {
                    produced.push(target);
                }
            }
        }
        Action::InflateZlib => {
            if is_zlib(bytes) {
                let mut inflated = Vec::new();
                let decoder = flate2::read::ZlibDecoder::new(bytes);
                // Bounded: a stream can claim to expand without end.
                if io::Read::read_to_end(&mut io::Read::take(decoder, LARGEST_FILE), &mut inflated).is_ok()
                    && !inflated.is_empty()
                {
                    let stem = path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                    let target = out.join(format!("{stem}.inflated"));
                    if fs::write(&target, inflated).is_ok() {
                        produced.push(target);
                    }
                }
            }
        }
        Action::CarveZip => {
            // A file that is already a zip from its first byte needs no carving.
            if !bytes.starts_with(b"PK\x03\x04") {
                if let Some(zip) = carve_zip(bytes) {
                    let stem =
                        path.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
                    let target = out.join(format!("{stem}.inner.zip"));
                    if fs::write(&target, zip).is_ok() {
                        produced.push(target);
                    }
                }
            }
        }
    }
    produced
}

/// Something found inside a download that may import.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub path: PathBuf,
    pub score: i32,
    /// The steps that led from the download to it.
    pub chain: Vec<Step>,
}

/// What exploring a download found.
#[derive(Debug, Default)]
pub struct Exploration {
    /// Importable files, best first. Never includes the download itself.
    pub candidates: Vec<Candidate>,
    /// Pocket PC libraries found along the way, which a game may need beside it.
    pub libraries: Vec<PathBuf>,
    /// Whether rules decided the first step, rather than trying everything.
    pub guided: bool,
}

/// Look inside `download`, working in `work`, which the caller removes.
pub fn explore(kb: &KnowledgeBase, download: &Path, work: &Path) -> Exploration {
    let mut found = Exploration::default();
    let mut queue: Vec<(PathBuf, Vec<Step>)> = vec![(download.to_path_buf(), Vec::new())];
    let mut seen = 0usize;
    let mut step = 0usize;
    let mut written = 0u64;
    let started = std::time::Instant::now();
    while let Some((path, chain)) = queue.pop() {
        seen += 1;
        if seen > MOST_FILES || written > MOST_BYTES || started.elapsed() > LONGEST {
            log::warn!("stopped exploring {} at its limits", download.display());
            break;
        }
        let Ok(meta) = fs::metadata(&path) else {
            continue;
        };
        if !meta.is_file() || meta.len() > LARGEST_FILE {
            continue;
        }
        let Ok(bytes) = fs::read(&path) else {
            continue;
        };
        let name = path.to_string_lossy().into_owned();
        let relative = path.strip_prefix(work).unwrap_or(&path).to_string_lossy().into_owned();
        let machine = pe_machine(&bytes);
        if !chain.is_empty() {
            if bytes.starts_with(b"MSCF") {
                found.candidates.push(Candidate {
                    score: kb.score(&relative, None) + kb.kind_score("cab"),
                    path: path.clone(),
                    chain: chain.clone(),
                });
            } else if machine == Some("arm") {
                if is_dll(&bytes) {
                    found.libraries.push(path.clone());
                } else {
                    let tiny = kb
                        .ranking
                        .tiny_exe
                        .as_ref()
                        .filter(|t| (bytes.len() as u64) < t.below)
                        .map_or(0, |t| t.score);
                    found.candidates.push(Candidate {
                        score: kb.score(&relative, machine) + kb.kind_score("exe") + tiny,
                        path: path.clone(),
                        chain: chain.clone(),
                    });
                }
            }
        }
        if chain.len() >= MAX_DEPTH {
            continue;
        }
        // Written knowledge first, so a user rule can override; then the basic
        // cases, which are compiled in because they cover most downloads.
        let mut actions = kb.actions_for(&name, &bytes);
        for action in basic_actions(&bytes, machine) {
            if !actions.contains(&action) {
                actions.push(action);
            }
        }
        if chain.is_empty() {
            found.guided = !actions.is_empty();
        }
        if machine == Some("arm") || (bytes.starts_with(b"MSCF") && actions.is_empty()) {
            // Already something to import, not a wrapper.
            continue;
        }
        if bytes.starts_with(b"MSCF") {
            // Only a repair applies to a cabinet; looking for wrappers in it
            // would just find itself.
            actions.retain(|a| *a == Action::ClearCabChecksums);
        }
        // Knowledge decides the order, never what is left out: a rule that is
        // too general, learned or written, only costs time, never a game.
        if !bytes.starts_with(b"MSCF") {
            for action in ALL_ACTIONS {
                if !actions.contains(&action) && action != Action::ClearCabChecksums {
                    actions.push(action);
                }
            }
        }
        for action in actions {
            step += 1;
            let out = work.join(format!("{step}"));
            for produced in apply(action, &path, &bytes, &out) {
                written += fs::metadata(&produced).map(|m| m.len()).unwrap_or(0);
                let mut next = chain.clone();
                next.push(Step { action, take: relative_name(&produced, &out) });
                queue.push((produced, next));
            }
        }
    }
    // Other processors' builds stay as a last resort: the desktop version runs
    // MIPS, and elsewhere the player is told why it does not start.
    found.candidates.retain(|c| c.score > NEVER_IMPORT);
    found.candidates.sort_by(|a, b| b.score.cmp(&a.score).then(a.path.cmp(&b.path)));
    found
}

fn relative_name(path: &Path, base: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

/// The wrappers most downloads come in, recognised from a few bytes. Kept in
/// code because they are checked on every failed import; anything else goes
/// through the knowledge base.
fn basic_actions(bytes: &[u8], machine: Option<&str>) -> Vec<Action> {
    if bytes.starts_with(b"MSCF") {
        // A cabinet that failed to install: often only its checksums disagree.
        return vec![Action::ClearCabChecksums];
    }
    if bytes.starts_with(b"PK\x03\x04") {
        // A download zip: setup.exe, one cabinet per processor, readme files.
        return vec![Action::OpenZip];
    }
    if is_zlib(bytes) {
        return vec![Action::InflateZlib];
    }
    let mut actions = Vec::new();
    if machine == Some("x86") {
        // A self-extracting archive: a desktop program with a zip after it.
        if find(bytes, b"PK\x05\x06", 0).is_some() {
            actions.extend([Action::OpenZip, Action::CarveZip]);
        }
        // A setup program holding the device cabinets as they are.
        if find(bytes, b"MSCF\0\0\0\0", 0).is_some() {
            actions.push(Action::CarveCab);
        }
        // A setup program holding them compressed with compress.exe.
        if find(bytes, SZDD_MAGIC, 0).is_some() {
            actions.push(Action::ExpandSzdd);
        }
        // A setup program carrying them as resources, often zlib-compressed.
        actions.push(Action::ExtractResources);
    }
    actions
}

/// What identifies one exact download: its size and CRC-32.
pub fn fingerprint(bytes: &[u8]) -> String {
    let mut crc = 0xffff_ffffu32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb8_8320 & 0u32.wrapping_sub(crc & 1));
        }
    }
    format!("{}:{:08x}", bytes.len(), !crc)
}

/// Repeat the steps the knowledge base knows for this exact download. Returns
/// the file to import and the Pocket PC libraries found beside it.
pub fn replay(kb: &KnowledgeBase, download: &Path, work: &Path) -> Option<(PathBuf, Vec<PathBuf>)> {
    let bytes = fs::read(download).ok()?;
    let print = fingerprint(&bytes);
    let known = kb.known.iter().find(|known| known.fingerprint == print)?;
    let mut current = download.to_path_buf();
    let mut libraries = Vec::new();
    for (index, step) in known.steps.iter().enumerate() {
        let data = fs::read(&current).ok()?;
        let out = work.join(format!("replay-{index}"));
        let produced = apply(step.action, &current, &data, &out);
        libraries = produced
            .iter()
            .filter(|p| fs::read(p).is_ok_and(|b| pe_machine(&b) == Some("arm") && is_dll(&b)))
            .cloned()
            .collect();
        current = produced.into_iter().find(|p| relative_name(p, &out) == step.take)?;
    }
    Some((current, libraries))
}

/// Remember how this exact download was imported, unless it is known already.
pub fn learn_known(library_root: &Path, known: Known) -> io::Result<bool> {
    let mut user = read_user_kb(library_root).unwrap_or_default();
    if user.known.iter().any(|k| k.fingerprint == known.fingerprint) {
        return Ok(false);
    }
    user.known.push(known);
    let text = serde_json::to_string_pretty(&user).map_err(io::Error::other)?;
    fs::write(library_root.join(USER_KB_FILE), text)?;
    Ok(true)
}

/// A rule for downloads that look like `bytes`, whose first step was `action`.
pub fn rule_for(name: &str, bytes: &[u8], action: Action) -> Rule {
    // Specific enough to describe only files the action can work on: the start
    // of the file and the marker the action looks for inside it.
    let marker = match action {
        Action::CarveCab | Action::ClearCabChecksums => "4d53434600000000",
        Action::ExpandSzdd => "535a444488f02733",
        Action::ExtractResources | Action::InflateZlib => "",
        Action::OpenZip | Action::CarveZip => "504b0506",
    };
    let when = Matcher {
        magic: Some(to_hex(&bytes[..bytes.len().min(4)])),
        contains_hex: if marker.is_empty() { Vec::new() } else { vec![marker.to_string()] },
        pe_machine: pe_machine(bytes).map(|m| vec![m.to_string()]).unwrap_or_default(),
        ..Matcher::default()
    };
    Rule {
        id: format!("learned-{}", when.magic.clone().unwrap_or_default()),
        note: format!("Learned while importing {name}."),
        when,
        actions: vec![action],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn cab(payload: &[u8]) -> Vec<u8> {
        // Only the header fields the carver reads: signature, size, version.
        let mut bytes = vec![0u8; 64];
        bytes[..8].copy_from_slice(b"MSCF\0\0\0\0");
        let size = (64 + payload.len()) as u32;
        bytes[8..12].copy_from_slice(&size.to_le_bytes());
        bytes[24] = 3;
        bytes[25] = 1;
        bytes.extend_from_slice(payload);
        bytes
    }

    fn x86_program(tail: &[u8]) -> Vec<u8> {
        let mut bytes = vec![0u8; 0x80];
        bytes[..2].copy_from_slice(b"MZ");
        bytes[0x3C] = 0x40;
        bytes[0x40..0x44].copy_from_slice(b"PE\0\0");
        bytes[0x44..0x46].copy_from_slice(&0x014Cu16.to_le_bytes());
        bytes.extend_from_slice(tail);
        bytes
    }

    fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        for (name, data) in files {
            zip.start_file(*name, zip::write::FileOptions::default()).unwrap();
            zip.write_all(data).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    #[test]
    fn the_builtin_knowledge_base_parses() {
        let kb = KnowledgeBase::builtin();
        assert!(!kb.problems.is_empty() && !kb.ranking.name.is_empty());
    }

    #[test]
    fn an_installer_gives_up_its_arm_cabinet_first() {
        let dir = tempfile::tempdir().unwrap();
        let inner = zip_of(&[
            ("game.mips.CAB", &cab(b"mips")),
            ("game.arm.CAB", &cab(b"arm")),
            ("setup.ini", b"[x]"),
        ]);
        let installer = dir.path().join("setup.exe");
        fs::write(&installer, x86_program(&inner)).unwrap();
        let work = dir.path().join("work");
        let found = explore(KnowledgeBase::builtin(), &installer, &work);
        assert!(found.guided);
        let best = &found.candidates[0];
        assert_eq!(best.chain[0].action, Action::OpenZip);
        assert!(best.path.to_string_lossy().contains("game.arm.CAB"), "{best:?}");
        assert!(found.candidates.last().unwrap().path.to_string_lossy().contains("mips"));
    }

    #[test]
    fn an_unknown_wrapper_is_explored_and_its_cabinet_found() {
        let dir = tempfile::tempdir().unwrap();
        let mut bytes = b"SOME-NEW-PACKER".to_vec();
        bytes.extend(vec![7u8; 100]);
        bytes.extend(cab(b"payload"));
        let download = dir.path().join("odd.bin");
        fs::write(&download, &bytes).unwrap();
        let found = explore(KnowledgeBase::builtin(), &download, &dir.path().join("work"));
        assert!(!found.guided);
        assert_eq!(found.candidates.len(), 1);
        assert_eq!(found.candidates[0].chain[0].action, Action::CarveCab);
    }

    #[test]
    fn a_cabinet_with_bad_checksums_gets_a_cleared_copy() {
        // One folder of two blocks, each with a checksum set.
        let mut cab = vec![0u8; 36];
        cab[..4].copy_from_slice(b"MSCF");
        cab[26] = 1;
        let folder_at = cab.len();
        cab.extend([0; 8]);
        let first = cab.len() as u32;
        cab[folder_at..folder_at + 4].copy_from_slice(&first.to_le_bytes());
        cab[folder_at + 4] = 2;
        for payload in [&b"abc"[..], &b"defg"[..]] {
            cab.extend([1, 2, 3, 4]);
            cab.extend((payload.len() as u16).to_le_bytes());
            cab.extend((payload.len() as u16).to_le_bytes());
            cab.extend(payload);
        }
        let cleared = clear_cab_checksums(&cab).unwrap();
        assert_eq!(&cleared[first as usize..first as usize + 4], &[0; 4]);
        let second = first as usize + 8 + 3;
        assert_eq!(&cleared[second..second + 4], &[0; 4]);
        assert!(clear_cab_checksums(&cleared).is_none());
    }

    #[test]
    fn a_compressed_cabinet_inside_an_installer_is_expanded() {
        // "abcabcabc": three literals, then a back reference of six bytes
        // to the window position where the literals were written.
        let mut stream = SZDD_MAGIC.to_vec();
        stream.extend([b'A', b'_']);
        stream.extend(9u32.to_le_bytes());
        let start = 4096 - 16;
        stream.extend([0b0000_0111, b'a', b'b', b'c', (start & 0xFF) as u8, (((start >> 4) & 0xF0) | (6 - 3)) as u8]);
        let mut installer = x86_program(b"junk");
        installer.extend(&stream);
        assert_eq!(expand_szdd_all(&installer), [b"abcabcabc".to_vec()]);
        assert_eq!(basic_actions(&installer, Some("x86")), [Action::ExpandSzdd, Action::ExtractResources]);
    }

    #[test]
    fn a_zlib_payload_is_inflated() {
        use std::io::Write as _;
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&cab(b"inside")).unwrap();
        let packed = encoder.finish().unwrap();
        assert!(is_zlib(&packed));
        let dir = tempfile::tempdir().unwrap();
        let download = dir.path().join("payload.bin");
        fs::write(&download, &packed).unwrap();
        let found = explore(KnowledgeBase::builtin(), &download, &dir.path().join("w"));
        assert_eq!(found.candidates.len(), 1);
        assert_eq!(found.candidates[0].chain[0].action, Action::InflateZlib);
    }

    #[test]
    fn a_tiny_launcher_loses_to_the_cabinet_beside_it() {
        let dir = tempfile::tempdir().unwrap();
        let mut launcher = x86_program(&[]);
        launcher[0x44..0x46].copy_from_slice(&0x01C0u16.to_le_bytes());
        let download = dir.path().join("game.zip");
        fs::write(&download, zip_of(&[("Game - ARM.exe", &launcher), ("game.cab", &cab(b"x"))])).unwrap();
        let found = explore(KnowledgeBase::builtin(), &download, &dir.path().join("work"));
        assert!(found.candidates[0].path.to_string_lossy().ends_with("game.cab"), "{:?}", found.candidates);
    }

    #[test]
    fn a_zip_bomb_entry_is_cut_at_the_limit() {
        // A declared size cannot be trusted, so extraction counts real bytes.
        let dir = tempfile::tempdir().unwrap();
        let big = vec![0u8; 1024];
        let download = dir.path().join("x.zip");
        fs::write(&download, zip_of(&[("a.bin", &big)])).unwrap();
        let out = apply(Action::OpenZip, &download, &fs::read(&download).unwrap(), &dir.path().join("o"));
        assert_eq!(fs::metadata(&out[0]).unwrap().len(), 1024);
    }

    #[test]
    fn a_known_download_is_replayed_straight_to_its_game() {
        let dir = tempfile::tempdir().unwrap();
        let inner = zip_of(&[("game.mips.CAB", &cab(b"m")), ("game.arm.CAB", &cab(b"a"))]);
        let download = dir.path().join("setup.exe");
        fs::write(&download, x86_program(&inner)).unwrap();
        let found = explore(KnowledgeBase::builtin(), &download, &dir.path().join("w1"));
        let best = &found.candidates[0];
        let bytes = fs::read(&download).unwrap();
        let known = Known { fingerprint: fingerprint(&bytes), name: "setup.exe".into(), steps: best.chain.clone() };
        assert!(learn_known(dir.path(), known.clone()).unwrap());
        assert!(!learn_known(dir.path(), known).unwrap());
        let kb = KnowledgeBase::load(dir.path());
        let (target, _) = replay(&kb, &download, &dir.path().join("w2")).unwrap();
        assert_eq!(fs::read(target).unwrap(), cab(b"a"));
        // A different file with no record is not replayed.
        fs::write(&download, b"other").unwrap();
        assert!(replay(&kb, &download, &dir.path().join("w3")).is_none());
    }

    #[test]
    fn a_learned_rule_is_kept_once_and_then_guides() {
        let dir = tempfile::tempdir().unwrap();
        let rule = rule_for("odd.bin", b"SOME-NEW-PACKER", Action::CarveCab);
        assert_eq!(rule.when.contains_hex, ["4d53434600000000"]);
        assert!(learn(dir.path(), rule.clone()).unwrap());
        assert!(!learn(dir.path(), rule).unwrap());
        let kb = KnowledgeBase::load(dir.path());
        let mut file = b"SOME-NEW-PACKER".to_vec();
        file.extend_from_slice(b"MSCF\0\0\0\0");
        assert_eq!(kb.actions_for("x", &file), [Action::CarveCab]);
        assert!(kb.actions_for("x", b"SOME-NEW-PACKER without a cabinet").is_empty());
    }

    #[test]
    fn a_broken_user_file_falls_back_to_the_builtin_knowledge() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(USER_KB_FILE), "{ not json").unwrap();
        assert_eq!(KnowledgeBase::load(dir.path()).problems.len(), KnowledgeBase::builtin().problems.len());
    }
}
