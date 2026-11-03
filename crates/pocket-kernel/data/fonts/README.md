# Fonts

`DejaVuSans-subset.ttf`, `DejaVuSans-Bold-subset.ttf` and `DejaVuSansMono-subset.ttf` are
DejaVu Sans and DejaVu Sans Mono 2.37 (https://dejavu-fonts.github.io/), cut down with
fontTools to Basic Latin, Latin-1, general punctuation, arrows and a few
shapes, without hinting. They stand in for Tahoma, the Pocket PC system font,
which may not be redistributed. The regular width wraps EverQuest's text
where Device Emulator's Tahoma does; the condensed cut was too narrow.

License: `LICENSE-DejaVu.txt` (Bitstream Vera license; DejaVu changes are in
the public domain). The subset keeps the DejaVu names, as the license asks of
modified fonts (no "Bitstream" or "Vera" in the name).

The mono face is used when a program asks for a fixed-pitch font (FIXED_PITCH,
FF_MODERN, or a Courier/Mono/Console face): BASIC interpreters and terminals
lay text out on a character grid.
