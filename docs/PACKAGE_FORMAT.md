# Game package format

What PocketHLE accepts when you import a game, on Windows, Android and in the
browser. All three use the same importer, so a package that works on one works
on the others.

## Accepted files

| You give it | What happens |
|---|---|
| A Pocket PC `.cab` | Installed the way the device would: files go to the folders the CAB names, and its registry settings are applied. |
| A `.zip` with one `.cab` at its top level, plus other files | The `.cab` is installed, then the other files are placed beside the game. This is how to add files the device itself had, such as `mfcce300.dll`. |
| A `.zip` of an installed game's folders | The folder holding the ARM `.exe` becomes the install folder, `\Program Files\<folder>\`. In the browser version the top-level folder also appears at the root of the device, so `quake/PocketQuake.exe` is found at `\quake\` too, where Pocket Quake looks for its data. |
| An ARM `.exe` | Run from `\Program Files\Game\`, with the files next to it. |
| A `.rar` | Read like a `.zip`. |
| Anything else from a download site | Looked inside, as described below. |

The importer reads the first bytes of a file before its extension, so a
download saved without an extension, or with the wrong one, still imports.

## Downloads that are not packages

Many Pocket PC programs were sold as desktop installers (`setup.exe`), as zips
with one CAB per processor, or as self-extracting archives. When a file does
not import as it is, the importer looks inside it, guided by the import
knowledge base (`crates/pocket-library/data/import_kb.json`):

1. **Rules** say how to look inside a kind of file: open it as a zip (also a
   zip appended to a program), cut out the cabinets stored in it, or cut out a
   zip stored in it. Nothing in a download is ever run.
2. **Ranking** decides between several builds. Names with `arm`, `2577`
   (StrongARM), `xscale` or `.sa.` win; `mips`, then `sh3` / `sh4` builds are
   taken only when there is no ARM one; `x86` builds and desktop installers'
   `data1.cab` are never imported.
3. When no rule fits, every way of looking inside is tried. If that finds a
   game, the step that worked is written to `import_kb.json` in your library
   folder, and the next file of the same kind is handled directly.

Your library's `import_kb.json` has the same layout as the built-in one and is
read first, so you can add rules, ranking terms or explanations without a new
release. A file that cannot be parsed is ignored.

## Which build to use

Pocket PC 2000 software often shipped one build per processor. From Pocket PC
2002 on, every device is ARM. The importer prefers the ARM build of a
multi-build download, and takes another processor's build only when there is
no ARM one.

| Processor | Devices | Names in downloads | PocketHLE |
|---|---|---|---|
| ARM: StrongARM SA-1100/1110 | Compaq iPAQ 36xx/38xx/31xx, HP Jornada 56x | `ARM`, `StrongARM`, `SA`, `2577` | Runs |
| ARM: XScale PXA25x/26x/27x | HP iPAQ, Dell Axim, ASUS MyPal | `XScale`, `PXA`, `ARMV4`, `ARMV4I` | Runs |
| ARM: Samsung S3C24xx, TI OMAP | Later PDAs, GPS units, Windows Mobile phones | `ARM`, `ARMV4I` | Runs |
| MIPS: NEC VR41xx | Casio Cassiopeia E-115, E-125, EM500 | `MIPS`, `4000`, `VR41xx` | Runs (MIPS16 code not yet) |
| SH3: Hitachi SuperH | HP Jornada 54x | `SH3`, `SH`, `10003` | Runs (interpreter) |
| SH4: Hitachi SuperH | A few Windows CE devices | `SH4` | Runs if the program uses no floating-point instructions |

## Files the device provided

Some games use libraries that were part of the Pocket PC itself, not of the
game. PocketHLE does not include them. If a game needs one, the importer still
succeeds, and the player is told which file is missing before the game starts.

| File | Used by | Where to put it |
|---|---|---|
| `mfcce300.dll` | Programs written with MFC (for example EverQuest, Brain Stadium, Pocket Painter) | In the package zip, next to the `.cab` |
| `olece300.dll` | MFC programs that use OLE | Same |

Only use copies taken from a device you own.

## Saves

A save zip holds the game's `saves/` folder and its `pockethle-registry.dat`.
Every version can download, import and reset saves, and a save made on one
works on the others. An import accepts nothing else, and replaces the game's
progress only when the whole zip is acceptable.

## When an import fails

Every failure is explained with what happened, what to do next, and the
technical detail. The explanations come from the `problems` section of the
knowledge base:

| Problem | What to do |
|---|---|
| Not a package / damaged archive | Download again, or repack the files as a plain zip. |
| A desktop installer's CAB (`data1.cab`, `_sys1.cab`) | Use the CAB named for ARM, StrongARM, PPC_2577 or Pocket PC. |
| A desktop program with no Pocket PC game inside | Open it with 7-Zip and take the ARM `.cab` out. |
| No Pocket PC program found | Check the package against this page. |
| A DLL chosen instead of the game | Import the `.exe` or `.cab`. |
| A .NET Compact Framework program | Not supported yet; look for a native ARM version. |
| A build for another processor (SH-4 floating point, SH-3E, x86) | Import the ARM, MIPS or SH3 build. |
| A device library is missing | Add it to the package as above. |
| Not enough storage | Remove games you no longer play. |
