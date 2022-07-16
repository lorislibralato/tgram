use std::io::Read;

pub type Result<T> = std::result::Result<T, crate::Err>;

pub struct Cursor<'a>(std::io::Cursor<&'a [u8]>);

impl<'a> Cursor<'a> {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.0.read_exact(buf).map_err(crate::Err::from)
    }
}

pub type Buf<'a> = &'a mut Cursor<'a>;

pub trait Deserializable: Sized {
    fn deserialize(buf: Buf) -> Result<Self>;
}

impl Deserializable for i32 {
    fn deserialize(buf: Buf) -> Result<Self> {
        let mut buffer = [0u8; 4];
        buf.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Deserializable for u32 {
    fn deserialize(buf: Buf) -> Result<Self> {
        let mut buffer = [0u8; 4];
        buf.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Deserializable for i64 {
    fn deserialize(buf: Buf) -> Result<Self> {
        let mut buffer = [0u8; 8];
        buf.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Deserializable for f64 {
    fn deserialize(buf: Buf) -> Result<Self> {
        let mut buffer = [0u8; 8];
        buf.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl Deserializable for Vec<u8> {
    fn deserialize(buf: Buf) -> Result<Self> {
        let mut buffer = [0u8; 1];
        buf.read_exact(&mut buffer)?;
        let first_byte = buffer[0];
        let (len, padding) = if first_byte == 254 {
            let mut buffer = [0u8; 3];
            buf.read_exact(&mut buffer)?;
            let len =
                (buffer[0] as usize) | ((buffer[1] as usize) << 8) | ((buffer[2] as usize) << 16);
            (len, len % 4)
        } else {
            let len = first_byte as usize;
            (len, (len + 1) % 4)
        };

        let mut result = vec![0u8; len];
        buf.read_exact(&mut result)?;

        if padding > 0 {
            let mut buffer = [0u8; 1];
            for _ in 0..(4 - padding) {
                buf.read_exact(&mut buffer)?;
            }
        }

        Ok(result)
    }
}

impl Deserializable for String {
    fn deserialize(buf: Buf) -> Result<Self> {
        String::from_utf8(Vec::<u8>::deserialize(buf)?).map_err(crate::Err::from)
    }
}
