use std::primitive;

trait HasBeBytes<const N: usize>
where
    Self: Sized,
{
    fn from_be_bytes(buf: [u8; N]) -> Self;
    fn to_be_bytes(self) -> [u8; N];
}

impl HasBeBytes<1> for u8 {
    fn from_be_bytes(buf: [u8; 1]) -> u8 {
        primitive::u8::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 1] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<2> for u16 {
    fn from_be_bytes(buf: [u8; 2]) -> u16 {
        primitive::u16::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<4> for u32 {
    fn from_be_bytes(buf: [u8; 4]) -> u32 {
        primitive::u32::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 4] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<8> for u64 {
    fn from_be_bytes(buf: [u8; 8]) -> u64 {
        primitive::u64::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}

mod reader {
    //! A module for reading Java (JVM) types from a raw byte stream

    use super::HasBeBytes;
    use crate::error::ReadError;

    use std::io::Read;

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

        fn read_n<T: HasBeBytes<N>, const N: usize>(&mut self) -> ReadResult<T> {
            let mut buf = [0u8; N];
            self.reader.read_exact(&mut buf)?;
            Ok(T::from_be_bytes(buf))
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
}

mod writer {
    //! A module for writing Java (JVM) types to a byte stream.

    use super::HasBeBytes;
    use crate::error::WriteError;
    use std::io::Write;

    pub type WriteResult<T> = Result<T, WriteError>;

    /// The `Writer` is used to write the bytes of a class file to a stream
    struct Writer<'a, W: Write> {
        writer: &'a mut W,
    }

    impl<'a, W: Write> Writer<'a, W> {
        pub fn new(writer: &'a mut W) -> Self {
            Writer { writer }
        }

        fn write_n<T: HasBeBytes<N>, const N: usize>(&mut self, data: T) -> WriteResult<()> {
            let buf = data.to_be_bytes();
            let len = std::mem::size_of::<T>();

            if buf.len() != len {
                return Err(WriteError::new(format!(
                    "error while writing bytes to stream: expected {} bytes, but found {}",
                    len,
                    buf.len()
                )));
            }

            self.writer.write(&buf)?;
            Ok(())
        }

        /// Write an unsigned byte (8 bits) to the byte stream.
        pub fn write_unsigned_byte(&mut self, b: u8) -> WriteResult<()> {
            self.write_n::<u8, 1>(b)
        }

        /// Write an unsigned short (16 bits) to the byte stream.
        pub fn write_unsigned_short(&mut self, s: u16) -> WriteResult<()> {
            self.write_n::<u16, 2>(s)
        }

        /// Write an unsigned int (32 bits) to the byte stream.
        pub fn write_unsigned_int(&mut self, i: u32) -> WriteResult<()> {
            self.write_n::<u32, 4>(i)
        }

        /// Write an unsigned long (64 bits) to the byte stream.
        pub fn write_unsigned_long(&mut self, l: u64) -> WriteResult<()> {
            self.write_n::<u64, 8>(l)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_write_unsigned_byte() {
            let mut buf = Vec::new();

            let mut writer = Writer::new(&mut buf);
            let _ = writer.write_unsigned_byte(0xca);
            let _ = writer.write_unsigned_byte(0xfe);
            let _ = writer.write_unsigned_byte(0xba);
            let _ = writer.write_unsigned_byte(0xbe);

            assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
        }

        #[test]
        fn test_write_unsigned_short() {
            let mut buf = Vec::new();

            let mut writer = Writer::new(&mut buf);
            let _ = writer.write_unsigned_short(0xcafe);
            let _ = writer.write_unsigned_short(0xbabe);

            assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
        }

        #[test]
        fn test_write_unsigned_int() {
            let mut buf = Vec::new();

            let mut writer = Writer::new(&mut buf);
            let _ = writer.write_unsigned_int(0xcafebabe);

            assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
        }

        #[test]
        fn test_write_unsigned_long() {
            let mut buf = Vec::new();

            let mut writer = Writer::new(&mut buf);
            let _ = writer.write_unsigned_long(0x00000041000f0a00);

            assert_eq!(buf, &[0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00]);
        }
    }
}
