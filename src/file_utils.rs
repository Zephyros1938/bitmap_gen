use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

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
        self.file.write_all(data)?;
        self.wpos += data.len();
        Ok(())
    }

    pub fn write_u8(&mut self, data: u8) -> std::io::Result<()> {
        self.file.write_all(&[data])?;
        self.wpos += 1;
        Ok(())
    }

    pub fn write_u16(&mut self, data: u16) -> std::io::Result<()> {
        self.file
            .write_all(&u16_to_u8_arr(data))?;
        self.wpos += 2;
        Ok(())
    }

    pub fn write_u32(&mut self, data: u32) -> std::io::Result<()> {
        self.file
            .write_all(&u32_to_u8_arr(data))?;
        self.wpos += 4;
        Ok(())
    }
}
