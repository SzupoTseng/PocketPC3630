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
  refused or flattened, and two names that become the same keep both files.
- **Limits on size.** One import may write at most 20,000 files, 512 MB per
  file and 2 GB in all (`crates/pocket-cab/src/limits.rs`). A program's
  header may not claim more than 256 MB of memory or list more than 65,536
  imports (`crates/pocket-pe/src/lib.rs`). A download that goes past a limit
  stops with a plain explanation instead of filling the disk or the browser's
  memory.
- **A failed import leaves nothing behind.** The game folder it started is
  removed.
- **Saves are checked.** A save zip may only hold the game's save folder and
  its registry, each file at most 64 MB.

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
