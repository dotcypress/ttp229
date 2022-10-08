//! A platform agnostic Rust driver for the TTP229, based on the
//! [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits.
//!
//! ## The Device
//!
//! The TTP229 IC is capacitive sensing design specifically for touch pad controls.
//! The device built in regulator for touch sensor.
//! The main application is focused at replacing of the mechanical switch or button.
//!
//! - [Details and datasheet](https://www.tontek.com.tw/uploads/product/106/TTP229-LSF_V1.0_EN.pdf)

#![no_std]

use core::ops::Shl;
use embedded_hal::blocking::spi::*;

/// Keyboard state
#[derive(Debug, Clone, Copy)]
pub struct Keyboard {
    raw: usize,
    cursor: usize,
}

impl Keyboard {
    fn new(raw: usize) -> Self {
        Self { raw, cursor: 0 }
    }

    /// Checks if provided key is pressed
    pub fn is_active(&self, key: Key) -> bool {
        let mask = 1_usize.shl(key as usize);
        self.raw & mask == 0
    }
}

impl Iterator for Keyboard {
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

/// Available keys
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
    fn new(offset: usize) -> Self {
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

/// Driver for the TTP229
pub struct TTP229<SPI> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> TTP229<SPI> {
    /// Initialize the TTP229 driver.
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Realeses SPI bus.
    pub fn release(self) -> SPI {
        self.spi
    }

    /// Gets keyboard state
    pub fn keys(&mut self) -> Result<Keyboard, SPI::Error> {
        let mut buf = [0_u8; 1];
        self.spi.transfer(&mut buf)?;
        Ok(Keyboard::new(buf[0] as _))
    }
}
