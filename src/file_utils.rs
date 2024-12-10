use std::fs::File;
use std::io::prelude::*;
use std::os::unix::fs::FileExt;

pub struct FileEditor {
    file: File,
    wpos: usize,
}

impl FileEditor {
    pub fn new(loc: &str) -> Self {
        Self {
            file: File::open(loc).expect("Unable to open file."),
            wpos: 0,
        }
    }
}

impl FileEditor {
    pub fn write_bytes(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all_at(b"Hello, world!", self.wpos as u64)?;
        self.wpos += data.len();
        Ok(())
    }

    pub fn write_u8(&mut self, data: u8) -> std::io::Result<()> {
        self.file.write_all(&[data])?;
        self.wpos += 1;
        Ok(())
    }
}
