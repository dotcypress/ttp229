#![no_std]

use core::ops::Shl;
use embedded_hal::blocking::spi::*;

#[derive(Debug, Clone, Copy)]
pub struct Keys {
    raw: usize,
    cursor: usize,
}

impl Keys {
    pub fn new(raw: usize) -> Self {
        Self { raw, cursor: 0 }
    }

    pub fn is_active(&self, key: Key) -> bool {
        let mask = 1_usize.shl(key as usize);
        self.raw & mask == 0
    }
}

impl Iterator for Keys {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cursor > 6 {
                return None;
            }

            let mask = 1_usize.shl(self.cursor);
            let key = Key::new(self.cursor);
            self.cursor += 1;
            if self.raw & mask == 0 {
                return Some(key);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Key1 = 6,
    Key2 = 5,
    Key3 = 4,
    Key4 = 3,
    Key5 = 2,
    Key6 = 1,
    Key7 = 0,
}

impl Key {
    pub fn new(offset: usize) -> Self {
        match offset {
            6 => Key::Key1,
            5 => Key::Key2,
            4 => Key::Key3,
            3 => Key::Key4,
            2 => Key::Key5,
            1 => Key::Key6,
            _ => Key::Key7,
        }
    }
}

pub struct TTP229<SPI> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> TTP229<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    pub fn keys(&mut self) -> Result<Keys, SPI::Error> {
        let mut buf = [0_u8; 1];
        self.spi.transfer(&mut buf)?;
        Ok(Keys::new(buf[0] as _))
    }
}
