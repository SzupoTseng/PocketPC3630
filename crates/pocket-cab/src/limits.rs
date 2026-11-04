//! How much one import may unpack.
//!
//! Downloads are untrusted. A cabinet, zip or rar a few kilobytes long can
//! claim to hold gigabytes ("decompression bombs"), or millions of empty
//! files. Without a limit that fills the disk, or in the browser runs the
//! page out of memory. Every extraction path counts what it writes against
//! one [`Budget`] and stops with an error when the import grows past it.
//!
//! The limits are far above any real Pocket PC package: devices had a few
//! tens of megabytes of storage, and the largest program in the library
//! (EverQuest) unpacks to well under 100 MB.

use std::cell::RefCell;
use std::io::{self, Read, Write};
use std::rc::Rc;

/// The most one file may hold.
pub const LARGEST_FILE: u64 = 512 * 1024 * 1024;
/// The most one import may write in all.
pub const MOST_BYTES: u64 = 2 * 1024 * 1024 * 1024;
/// The most files one import may write.
pub const MOST_FILES: usize = 20_000;

/// What one import has written so far.
#[derive(Debug, Clone)]
pub struct Budget {
    files: usize,
    bytes: u64,
    largest_file: u64,
    most_bytes: u64,
    most_files: usize,
}

impl Default for Budget {
    fn default() -> Self {
        Self::new(LARGEST_FILE, MOST_BYTES, MOST_FILES)
    }
}

impl Budget {
    /// A budget with limits of its own (tests use small ones).
    pub fn new(largest_file: u64, most_bytes: u64, most_files: usize) -> Self {
        Self {
            files: 0,
            bytes: 0,
            largest_file,
            most_bytes,
            most_files,
        }
    }

    /// Bytes written so far.
    pub fn bytes(&self) -> u64 {
        self.bytes
    }

    /// Files written so far.
    pub fn files(&self) -> usize {
        self.files
    }

    /// Count one more file, failing once there are too many.
    pub fn add_file(&mut self, name: &str) -> io::Result<()> {
        if self.files >= self.most_files {
            return Err(too_large(format!(
                "more than {} files (at {name})",
                self.most_files
            )));
        }
        self.files += 1;
        Ok(())
    }

    /// Copy one file from `from` to `to`, counting it. Fails, having
    /// written at most one byte past a limit, when the file is larger than
    /// [`LARGEST_FILE`] or the import as a whole larger than [`MOST_BYTES`].
    pub fn copy(
        &mut self,
        name: &str,
        from: &mut impl Read,
        to: &mut impl Write,
    ) -> io::Result<u64> {
        self.add_file(name)?;
        let room = self.largest_file.min(self.most_bytes - self.bytes);
        let copied = io::copy(&mut from.take(room + 1), to)?;
        if copied > room {
            return Err(too_large(if room == self.largest_file {
                format!("{name} is larger than {} MB", self.largest_file >> 20)
            } else {
                format!(
                    "the download unpacks to more than {} MB",
                    self.most_bytes >> 20
                )
            }));
        }
        self.bytes += copied;
        Ok(copied)
    }

    /// Read one file into memory, counting it like [`Budget::copy`].
    pub fn read(&mut self, name: &str, from: &mut impl Read) -> io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.copy(name, from, &mut data)?;
        Ok(data)
    }
}

/// A writer counting what goes through it against a shared [`Budget`],
/// for extractors that ask for a writer per file instead of copying.
pub struct Limited<W> {
    inner: W,
    name: String,
    written: u64,
    budget: Rc<RefCell<Budget>>,
}

impl<W: Write> Limited<W> {
    /// Count one more file against `budget` and write it to `inner`.
    pub fn new(inner: W, name: String, budget: Rc<RefCell<Budget>>) -> io::Result<Self> {
        budget.borrow_mut().add_file(&name)?;
        Ok(Self {
            inner,
            name,
            written: 0,
            budget,
        })
    }
}

impl<W: Write> Write for Limited<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut budget = self.budget.borrow_mut();
        let file_room = budget.largest_file - self.written;
        let room = file_room.min(budget.most_bytes - budget.bytes);
        if buf.len() as u64 > room {
            return Err(too_large(if room == file_room {
                format!(
                    "{} is larger than {} MB",
                    self.name,
                    budget.largest_file >> 20
                )
            } else {
                format!(
                    "the download unpacks to more than {} MB",
                    budget.most_bytes >> 20
                )
            }));
        }
        let n = self.inner.write(buf)?;
        self.written += n as u64;
        budget.bytes += n as u64;
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn too_large(message: String) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("too large to import: {message}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_over_the_limit_is_refused() {
        let mut budget = Budget::new(10, 100, 10);
        assert_eq!(budget.read("a", &mut &[1u8; 10][..]).unwrap().len(), 10);
        let error = budget.read("b", &mut &[1u8; 11][..]).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("b is larger"));
    }

    #[test]
    fn the_whole_import_is_counted() {
        let mut budget = Budget::new(10, 25, 10);
        budget.read("a", &mut &[0u8; 10][..]).unwrap();
        budget.read("b", &mut &[0u8; 10][..]).unwrap();
        let error = budget.read("c", &mut &[0u8; 10][..]).unwrap_err();
        assert!(error.to_string().contains("unpacks to more than"));
        assert_eq!(budget.bytes(), 20);
    }

    #[test]
    fn too_many_files_are_refused() {
        let mut budget = Budget::new(10, 100, 2);
        budget.add_file("a").unwrap();
        budget.read("b", &mut &[][..]).unwrap();
        assert!(budget.add_file("c").is_err());
        assert_eq!(budget.files(), 2);
    }

    #[test]
    fn a_limited_writer_shares_the_budget() {
        let budget = Rc::new(RefCell::new(Budget::new(10, 15, 10)));
        let mut a = Limited::new(Vec::new(), "a".into(), budget.clone()).unwrap();
        a.write_all(&[0; 10]).unwrap();
        assert!(a.write_all(&[0; 1]).is_err());
        let mut b = Limited::new(Vec::new(), "b".into(), budget.clone()).unwrap();
        assert!(b.write_all(&[0; 6]).is_err());
        b.write_all(&[0; 5]).unwrap();
        assert_eq!(budget.borrow().bytes(), 15);
    }
}
