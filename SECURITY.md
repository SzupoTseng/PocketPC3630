<!--
Copyright (C) 2026 PocketHLE Emulator Project
SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Security

PocketPC3630 opens files from the internet: cabinets, zips, rars, desktop
setup programs and the Pocket PC programs inside them. All of that is treated
as untrusted.

## What the importer promises

- **Nothing is run to unpack it.** Setup programs (Wise, InstallShield, Inno
  Setup, Setup Factory, NSIS) are read as data. Only the guest program itself
  runs, inside the emulated CPU, with the emulated Windows CE as its only way
  out.
- **Files stay inside the library.** Names from an archive are made safe
  before anything is written: `..`, drive letters and absolute paths are
  refused or flattened. In a cabinet, two names that become the same keep
  both files; in a zip or rar a later entry with the same path replaces the
  earlier one, as unzipping it would.
- **Limits on size.** The library's importer (Windows, Android and the
  browser) counts everything one import writes against one budget: at most
  20,000 files, 512 MB per file and 2 GB in all for a cabinet, zip or rar
  (`crates/pocket-cab/src/limits.rs`). When a download is not a package and
  is looked inside, every step of that search shares one smaller budget:
  4,000 files, 128 MB per file, 1 GB in all, 60 seconds
  (`crates/pocket-library/src/import_kb.rs`). A file past a limit is dropped,
  never kept cut short. (A damaged stream inside a setup program can still
  give the part that came out before the damage.) The setup-program readers check the sizes a file
  declares about itself before setting memory aside for them.
- **Limits on programs.** A program's header may not claim more than 256 MB
  of memory, name more than 1,024 DLLs or list more than 65,536 imports,
  exports or resources. Offsets in its import and resource directories are
  checked before they are used (`crates/pocket-pe`).
- **A failed import changes nothing.** Folders it created are removed, and a
  game it was replacing is put back as it was, with its place in the
  library. An import cut short by a crash is settled the next time.
- **Saves are checked.** A save zip may only hold the game's save folder and
  its registry, each file at most 64 MB.

The command-line tool (`pockethle play`) unpacks with the same per-file and
total limits, counted separately for each layer it opens (a setup program,
then the cabinet inside it).

The guest program can read and write only its own game folder and the
emulated device's folders under the library. It has no network access:
Winsock calls are answered by a model inside the emulator that passes
datagrams between the program's own sockets and never opens a host socket.

## Reporting a problem

Please report security problems privately through GitHub's
**Security → Report a vulnerability** on this repository, not in a public
issue. Include the file (or how to make it) and what happened. Expect an
answer within two weeks.

Do not attach game files you do not have the right to share; a description of
the file's structure, or a small file made to show the problem, is enough.
