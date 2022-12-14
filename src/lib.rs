#![no_std]

use core::any::Any;
use core::cmp::min;
use acid_io::Read;

pub struct BitReader<'a, T: acid_io::Read> {
    offset: u8,
    buffer: u8,
    read: &'a mut T
}

impl<'a, T: acid_io::Read> BitReader<'a, T> {
    pub fn new(inner: &'a mut T) -> Self {
        Self {
            offset: 8,
            buffer: 0,
            read: inner
        }
    }
    pub fn aligned(&self) -> bool {
        self.offset == 0
    }

    #[inline]
    fn fill_buffer(&mut self) -> acid_io::Result<()> {
        if self.offset == 8 {
            self.read.read_exact(&mut core::slice::from_mut(&mut self.buffer))?;
            self.offset = 0;
        }
        Ok(())
    }

    #[inline]
    pub fn read_bit(&mut self) -> acid_io::Result<bool> {
        self.fill_buffer()?;
        let v = self.buffer & (1 << 7 >> self.offset) != 0;
        self.offset += 1;
        Ok(v)
    }

    #[inline]
    pub fn read_bits<U: core::ops::ShlAssign + core::ops::BitOrAssign + From<u8>>(&mut self, mut bits: u8) -> acid_io::Result<U> {
        Ok(if bits < 8 - self.offset {
            self.fill_buffer()?;
            let v = self.buffer << self.offset >> (8 - bits);
            self.offset += bits;
            U::from(v)
        } else {
            let mut v = U::from(self.buffer << self.offset >> self.offset);
            self.offset = 8;
            while bits > 8 {
                self.fill_buffer()?;
                v <<= U::from(8);
                v |= U::from(self.buffer);
                bits -= 8;
            }
            self.fill_buffer()?;
            v |= U::from(self.buffer << self.offset >> (8 - bits));
            self.offset += bits;
            v
        })
    }
}






















