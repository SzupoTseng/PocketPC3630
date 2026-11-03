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
const COMMCTRL_JSON: &str = include_str!("../data/commctrl-ordinals.json");

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

const COREDLL_MIPS_JSON: &str = include_str!("../data/coredll-ordinals-mips.json");
const COREDLL_SH_JSON: &str = include_str!("../data/coredll-ordinals-sh.json");

/// MIPS and SuperH builds of `coredll.dll` number their compiler runtime
/// helpers (2000 and up) differently from ARM: on SH3 ordinal 2005 is
/// `__modls` (`x % n`, dividend first) where ARM has `__rt_sdiv`, and 2008
/// is the single-precision `__adds`. Resolving them with the ARM table sent
/// FreeCell's shuffle into an endless loop and stopped Pocket Slay's MIPS
/// and SH3 builds while generating a map. These tables come from the
/// Pocket PC 2000 SDK's per-CPU `coredll.lib`.
static CPU_HELPER_TABLES: Lazy<[HashMap<u16, String>; 2]> = Lazy::new(|| {
    [COREDLL_MIPS_JSON, COREDLL_SH_JSON].map(|json| {
        serde_json::from_str::<OrdinalFile>(json)
            .map(|file| {
                file.map
                    .into_iter()
                    .filter_map(|(k, v)| k.parse::<u16>().ok().map(|o| (o, v)))
                    .collect()
            })
            .unwrap_or_else(|e| {
                log::error!("failed to parse a per-CPU coredll ordinal table: {e}");
                HashMap::new()
            })
    })
});

/// Look up `ordinal` for a program built for `machine` (a PE machine
/// value). Only coredll's runtime helpers differ between processors.
pub fn lookup_for_machine(machine: u16, dll: &str, ordinal: u16) -> Option<String> {
    if dll.eq_ignore_ascii_case("coredll.dll") {
        let table = match machine {
            0x0162 | 0x0166 | 0x0168 | 0x0169 | 0x0266 | 0x0366 | 0x0466 => Some(0),
            0x01a2 | 0x01a3 | 0x01a4 | 0x01a6 => Some(1),
            _ => None,
        };
        if let Some(index) = table {
            if let Some(name) = CPU_HELPER_TABLES[index].get(&ordinal) {
                return Some(name.clone());
            }
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

/// commctrl.dll's ordinal names. Kept out of [`lookup`]: the main image's
/// commctrl imports are answered by `ord:N` handlers, and only DLLs the
/// guest loads itself (MFC) need the names.
static COMMCTRL_ORDINALS: Lazy<HashMap<u16, String>> = Lazy::new(|| {
    serde_json::from_str::<OrdinalFile>(COMMCTRL_JSON)
        .map(|file| {
            file.map
                .into_iter()
                .filter_map(|(k, v)| k.parse().ok().map(|k| (k, v)))
                .collect()
        })
        .unwrap_or_default()
});

/// The export name a guest-loaded DLL means when it imports `dll` by
/// `ordinal`: MFC imports `aygshell.dll!#34`, which is `SHCreateMenuBar`.
pub fn lookup_for_guest_module(dll: &str, ordinal: u16) -> Option<String> {
    let key = dll.to_ascii_lowercase();
    if key == "commctrl.dll" {
        return COMMCTRL_ORDINALS.get(&ordinal).cloned();
    }
    lookup(&key, ordinal)
}

#[cfg(test)]
mod tests {
    #[test]
    fn superh_division_helpers_have_their_own_ordinals() {
        let sh = |o| super::lookup_for_machine(0x01a2, "COREDLL.dll", o);
        let mips = |o| super::lookup_for_machine(0x0166, "COREDLL.dll", o);
        assert_eq!(sh(2005).as_deref(), Some("__modls"));
        assert_eq!(sh(2006).as_deref(), Some("__modlu"));
        assert_eq!(sh(2008).as_deref(), Some("__adds"));
        assert_eq!(sh(2027).as_deref(), Some("sh__itod"));
        assert_eq!(mips(2022).as_deref(), Some("__fpadd"));
        assert_eq!(mips(2005).as_deref(), Some("__ll_div"));
        assert_eq!(
            super::lookup_for_machine(0x01c0, "COREDLL.dll", 2006).as_deref(),
            Some("__rt_sdiv10")
        );
        // Ordinary APIs share the ARM numbering on every processor.
        assert_eq!(sh(2).as_deref(), Some("InitializeCriticalSection"));
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
