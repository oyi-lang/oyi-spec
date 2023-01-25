//! A module for reading Java (JVM) types from a raw byte stream

use crate::error::ReadError;
use std::{
    convert::From,
    default::Default,
    io::Read,
    ops::{BitOrAssign, Mul, Shl},
};

pub type ReadResult<T> = Result<T, ReadError>;

/// The `Reader` is used to read bytes off of a stream
/// over raw ``class` file objects.
struct Reader<R: Read> {
    reader: R,
}

impl<R: Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Reader { reader }
    }

    fn read_n<
        T: Shl + Default + BitOrAssign<<T as Shl>::Output> + From<u8> + Mul<T, Output = T>,
        const N: usize,
    >(
        &mut self,
    ) -> ReadResult<T> {
        let mut buf = [0u8; N];
        self.reader.read_exact(&mut buf)?;

        let mut r = T::default();
        for i in 0..N {
            r |= <u8 as Into<T>>::into(buf[i])
                << (<u8 as Into<T>>::into(8u8) * <u8 as Into<T>>::into(N as u8 - i as u8 - 1u8));
        }

        Ok(r)
    }

    /// Read an unsigned byte (8 bits) from the byte stream.
    pub fn read_unsigned_byte(&mut self) -> ReadResult<u8> {
        self.read_n::<u8, 1>()
    }

    /// Read an unsigned short (16 bits) from the byte stream.
    pub fn read_unsigned_short(&mut self) -> ReadResult<u16> {
        self.read_n::<u16, 2>()
    }

    /// Read an unsigned int (32 bits) from the byte stream.
    pub fn read_unsigned_int(&mut self) -> ReadResult<u32> {
        self.read_n::<u32, 4>()
    }

    /// Read an unsigned long (64 bits) from the byte stream.
    pub fn read_unsigned_long(&mut self) -> ReadResult<u64> {
        self.read_n::<u64, 8>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_unsigned_byte() {
        let bytes = [0xca, 0xfe, 0xba, 0xbe];
        let mut reader = Reader::new(Cursor::new(bytes));

        assert_eq!(reader.read_unsigned_byte().unwrap(), 0xca);
        assert_eq!(reader.read_unsigned_byte().unwrap(), 0xfe);
        assert_eq!(reader.read_unsigned_byte().unwrap(), 0xba);
        assert_eq!(reader.read_unsigned_byte().unwrap(), 0xbe);
    }

    #[test]
    fn test_read_unsigned_short() {
        let bytes = [0xca, 0xfe, 0xba, 0xbe];
        let mut reader = Reader::new(Cursor::new(bytes));

        assert_eq!(reader.read_unsigned_short().unwrap(), 0xcafe);
        assert_eq!(reader.read_unsigned_short().unwrap(), 0xbabe);
    }

    #[test]
    fn test_read_unsigned_int() {
        let bytes = [0xca, 0xfe, 0xba, 0xbe];
        let mut reader = Reader::new(Cursor::new(bytes));

        assert_eq!(reader.read_unsigned_int().unwrap(), 0xcafebabe);
    }

    #[test]
    fn test_read_unsigned_long() {
        let bytes = [0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00];
        let mut reader = Reader::new(Cursor::new(bytes));

        assert_eq!(reader.read_unsigned_long().unwrap(), 0x00000041000f0a00);
    }
}