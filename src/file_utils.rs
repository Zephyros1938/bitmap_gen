use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::os::unix::fs::FileExt;

use crate::bitwise_util::{u16_to_u8_arr, u32_to_u8_arr};

pub struct FileEditor {
    file: File,
    wpos: usize,
}

impl FileEditor {
    pub fn new(loc: &str) -> Self {
        Self {
            file: OpenOptions::new()
                .create(true)
                .write(true)
                .open(loc)
                .unwrap(),
            wpos: 0,
        }
    }
}

impl FileEditor {
    pub fn write_bytes(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.file.write_all_at(data, self.wpos as u64)?;
        self.wpos += data.len();
        Ok(())
    }

    pub fn write_u8(&mut self, data: u8) -> std::io::Result<()> {
        self.file.write_all_at(&[data], self.wpos as u64)?;
        self.wpos += 1;
        Ok(())
    }

    pub fn write_u16(&mut self, data: u16) -> std::io::Result<()> {
        self.file
            .write_all_at(&u16_to_u8_arr(data), self.wpos as u64)?;
        self.wpos += 2;
        Ok(())
    }

    pub fn write_u32(&mut self, data: u32) -> std::io::Result<()> {
        self.file
            .write_all_at(&u32_to_u8_arr(data), self.wpos as u64)?;
        self.wpos += 2;
        Ok(())
    }
}
