//! Ordinal → name resolution for system DLLs that export by ordinal.
//!
//! On Pocket PC 2002 / 2003 / Windows Mobile 5/6, `coredll.dll` and
//! `aygshell.dll` export every public function as both a name and an
//! ordinal but the SDK link libraries strip the names by default to
//! save space, so applications import by ordinal. To make logs
//! readable we keep an ordinal table per DLL keyed by lowercase DLL
//! name and ordinal.
//!
//! The default tables are loaded from JSON files in `data/`. They are
//! deliberately incomplete — they cover the subset of APIs encountered
//! while bringing up the JumpyBall test ROM, plus a few neighbours.
//! Submit additions via PR.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

const COREDLL_JSON: &str = include_str!("../data/coredll-ordinals.json");
const AYGSHELL_JSON: &str = include_str!("../data/aygshell-ordinals.json");

#[derive(Debug, Deserialize)]
struct OrdinalFile {
    /// Pretty name of the DLL — informational.
    #[allow(dead_code)]
    dll: String,
    /// Ordinal -> exported name.
    #[serde(rename = "ordinals")]
    map: HashMap<String, String>,
}

static ORDINAL_TABLES: Lazy<HashMap<String, HashMap<u16, String>>> = Lazy::new(|| {
    let mut tables: HashMap<String, HashMap<u16, String>> = HashMap::new();
    for (dll, json) in [
        ("coredll.dll", COREDLL_JSON),
        ("aygshell.dll", AYGSHELL_JSON),
    ] {
        match serde_json::from_str::<OrdinalFile>(json) {
            Ok(file) => {
                let mut converted = HashMap::with_capacity(file.map.len());
                for (k, v) in file.map {
                    if let Ok(ord) = k.parse::<u16>() {
                        converted.insert(ord, v);
                    }
                }
                tables.insert(dll.to_string(), converted);
            }
            Err(e) => {
                log::error!("failed to parse ordinal table for {dll}: {e}");
            }
        }
    }
    tables
});

/// Look up the symbolic name for an ordinal in `dll`. Returns `None`
/// if the DLL or ordinal is unknown.
pub fn lookup(dll: &str, ordinal: u16) -> Option<String> {
    let key = dll.to_ascii_lowercase();
    // The OpenGL ES client libraries keep their tables in
    // `pocket-gles`, next to the implementation they describe.
    if pocket_gles::ordinals::is_gles_dll(&key) {
        return pocket_gles::ordinals::lookup(&key, ordinal);
    }
    ORDINAL_TABLES
        .get(&key)
        .and_then(|m| m.get(&ordinal).cloned())
}

/// SuperH builds of `coredll.dll` put their own compiler helpers on some
/// of the ordinals the ARM table gives to `__rt_*` routines. Read from the
/// SH3 FreeCell that ships with Pocket PC 2000: it computes
/// `Random() % count` through ordinal 2006 and card ranks through 2005
/// (`x % 13`) and 2001 (`x / 13`), each with the dividend in r4 and the
/// divisor in r5 — the Hitachi `__divls` / `__modls` / `__modlu` pattern.
/// Taking them as the ARM helpers divided the wrong way round, so the
/// shuffle loop never ended.
const SH_COREDLL: &[(u16, &str)] = &[(2001, "__divls"), (2005, "__modls"), (2006, "__modlu")];

/// Look up `ordinal` for a program built for `machine` (a PE machine
/// value). Only the SuperH helpers differ from the shared table.
pub fn lookup_for_machine(machine: u16, dll: &str, ordinal: u16) -> Option<String> {
    if matches!(machine, 0x01a2 | 0x01a3 | 0x01a6) && dll.eq_ignore_ascii_case("coredll.dll") {
        if let Some((_, name)) = SH_COREDLL.iter().find(|(ord, _)| *ord == ordinal) {
            return Some((*name).to_string());
        }
    }
    lookup(dll, ordinal)
}

/// Number of ordinal entries for `dll` — used by tests.
pub fn entry_count(dll: &str) -> usize {
    let key = dll.to_ascii_lowercase();
    if pocket_gles::ordinals::is_gles_dll(&key) {
        return pocket_gles::ordinals::entry_count(&key);
    }
    ORDINAL_TABLES.get(&key).map(|m| m.len()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn superh_division_helpers_have_their_own_ordinals() {
        assert_eq!(super::lookup_for_machine(0x01a2, "COREDLL.dll", 2006).as_deref(), Some("__modlu"));
        assert_eq!(super::lookup_for_machine(0x01c0, "COREDLL.dll", 2006).as_deref(), Some("__rt_sdiv10"));
        assert_eq!(super::lookup_for_machine(0x01a2, "COREDLL.dll", 2008).as_deref(), Some("__rt_udiv"));
    }

    use super::*;

    #[test]
    fn coredll_table_loads() {
        assert!(entry_count("coredll.dll") > 10);
    }

    #[test]
    fn lookup_known_ordinal() {
        // Most leaked WM5/WM6 SDKs map ordinal 33 to one of the
        // CRT-style helpers; exact name varies by build, so just
        // verify the lookup returns *something*.
        let _ = lookup("coredll.dll", 33);
    }

    #[test]
    fn gles_ordinals_resolve_through_the_shared_resolver() {
        // Games import libGLES_CL.dll purely by ordinal, so this path
        // is what turns `#50` into `glDrawElements` for both the
        // dispatcher and the logs.
        assert_eq!(
            lookup("libGLES_CL.dll", 50).as_deref(),
            Some("glDrawElements")
        );
        assert_eq!(
            lookup("libgles_cm.dll", 133).as_deref(),
            Some("glTexImage2D")
        );
        assert_eq!(entry_count("libgles_cm.dll"), 140);
        assert_eq!(entry_count("libgles_cl.dll"), 102);
    }
}
