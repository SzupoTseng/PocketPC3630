# Local rmp3 0.3.1 patch

Origin: crates.io rmp3 0.3.1, upstream https://github.com/notviri/rmp3.
Original registry checksum: d3d9fb2c89a819645c57007782fd4db5bfa1abe290e1df5f530a441dc21ab1ad.
The upstream CC0-1.0 LICENSE and minimp3 source are preserved.

Changes are limited to src/lib.rs:

- Zero-initialize mp3dec_frame_info_t, whose fields are all C integers. minimp3
  can return non-audio data without filling every field. Reading the old
  uninitialized frame_offset caused the FMOD garbage-input test to abort on
  Rust 1.97's unsafe slice precondition check.
- Use checked source ranges; negative, reversed or out-of-input decoder spans
  stop decoding rather than creating an invalid Rust slice.

Both FMOD file decoding and the emulated MAS1 stream device use this patched
crate through the workspace patch. Public APIs, features and C decoder remain
unchanged. Regression tests live with the emulator's audio callers.
