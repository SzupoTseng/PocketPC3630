//! The separate tables that decide how an imported function is answered
//! must agree with each other: the dispatcher's handlers, the native ARM
//! thunks, the interpreter's intrinsics, the constant-return stubs and the
//! ordinal tables.

use std::collections::BTreeSet;

use pocket_winceapi::{ordinals, WinCeDispatcher};

/// Every coredll name we know of: the ordinal table plus what is registered.
fn coredll_names(d: &WinCeDispatcher) -> BTreeSet<String> {
    let mut names: BTreeSet<String> = (0..=4095u16)
        .filter_map(|ordinal| ordinals::lookup("coredll.dll", ordinal))
        .collect();
    names.extend(
        d.registered_iter()
            .filter(|(dll, _)| *dll == "coredll.dll")
            .map(|(_, name)| name.to_string()),
    );
    names
}

#[test]
fn no_name_is_registered_twice_with_different_answers() {
    let d = WinCeDispatcher::new();
    assert!(
        d.registration_conflicts().is_empty(),
        "registered twice with different handlers or constants: {:?}",
        d.registration_conflicts()
    );
}

/// A native thunk replaces the call on the Unicorn path; the handler is
/// what answers it everywhere else (the interpreter, `GetProcAddress`).
#[test]
fn every_native_thunk_has_a_handler() {
    let d = WinCeDispatcher::new();
    let missing: Vec<_> = coredll_names(&d)
        .into_iter()
        .filter(|name| {
            pocket_kernel::native_thunks::native_thunk_for("coredll.dll", name).is_some()
        })
        .filter(|name| !d.has_handler("coredll.dll", name))
        .collect();
    assert!(
        missing.is_empty(),
        "native thunks without a handler: {missing:?}"
    );
}

/// An intrinsic answers the call inside the interpreter; the handler must
/// exist for the same name, and the two are tested to agree elsewhere
/// (pocket-cpu's `interp_intrinsics`).
#[test]
fn every_interpreter_intrinsic_has_a_handler() {
    let d = WinCeDispatcher::new();
    let names: Vec<_> = coredll_names(&d)
        .into_iter()
        .filter(|name| pocket_cpu::helper_intrinsic("coredll.dll", name).is_some())
        .collect();
    assert!(names.len() >= 36, "intrinsics found: {names:?}");
    let missing: Vec<_> = names
        .into_iter()
        .filter(|name| !d.has_handler("coredll.dll", name))
        .collect();
    assert!(
        missing.is_empty(),
        "intrinsics without a handler: {missing:?}"
    );
}

/// `ord:N` and `#N` answer exactly like the name the ordinal table gives N.
#[test]
fn ordinal_aliases_share_the_named_handler_and_constant() {
    let d = WinCeDispatcher::new();
    for ordinal in 0..=4095u16 {
        let Some(name) = ordinals::lookup("coredll.dll", ordinal) else {
            continue;
        };
        if !d.has_handler("coredll.dll", &name) {
            continue;
        }
        for alias in [format!("ord:{ordinal}"), format!("#{ordinal}")] {
            assert!(
                d.same_handler("coredll.dll", &name, &alias),
                "{alias} does not answer like {name}"
            );
            assert_eq!(
                d.registered_constant("coredll.dll", &alias),
                d.registered_constant("coredll.dll", &name),
                "{alias} and {name} return different constants"
            );
        }
    }
}

/// Ordinals games import that the PPC2002 tables do not list: Gameloft's
/// SDL port (Sonic Unleashed) imports aygshell 341 and 344 for its
/// full-screen and task-bar handling.
const BEYOND_THE_TABLES: &[(&str, u16)] = &[("aygshell.dll", 341), ("aygshell.dll", 344)];

/// A handler registered by ordinal for a DLL with an ordinal table must be
/// an ordinal that table knows, or it can never be reached by name.
#[test]
fn ordinal_registrations_are_in_their_tables() {
    let d = WinCeDispatcher::new();
    let mut unknown = Vec::new();
    for (dll, name) in d.registered_iter() {
        if dll == "coredll.dll" || ordinals::entry_count(dll) == 0 {
            continue;
        }
        let Some(number) = name
            .strip_prefix("ord:")
            .or_else(|| name.strip_prefix('#'))
            .and_then(|n| n.parse::<u16>().ok())
        else {
            continue;
        };
        if ordinals::lookup(dll, number).is_none() && !BEYOND_THE_TABLES.contains(&(dll, number)) {
            unknown.push(format!("{dll}!{name}"));
        }
    }
    unknown.sort();
    assert!(
        unknown.is_empty(),
        "ordinals not in their tables: {unknown:?}"
    );
}
