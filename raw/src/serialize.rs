pub type Buf<'a> = &'a mut Vec<u8>;

pub trait Serializable {
    fn serialize(&self, buf: Buf);
}

impl Serializable for i32 {
    fn serialize(&self, buf: Buf) {
        buf.extend(self.to_le_bytes())
    }
}

impl Serializable for u32 {
    fn serialize(&self, buf: Buf) {
        buf.extend(self.to_le_bytes())
    }
}

impl Serializable for i64 {
    fn serialize(&self, buf: Buf) {
        buf.extend(self.to_le_bytes())
    }
}

impl Serializable for f64 {
    fn serialize(&self, buf: Buf) {
        buf.extend(self.to_le_bytes())
    }
}

impl Serializable for [u8; 16] {
    fn serialize(&self, buf: Buf) {
        buf.extend(self)
    }
}

impl Serializable for [u8; 32] {
    fn serialize(&self, buf: Buf) {
        buf.extend(self)
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, buf: Buf) {
        0x1cb5c415u32.serialize(buf);
        (self.len() as i32).serialize(buf);
        self.iter().for_each(|e| e.serialize(buf))
    }
}

impl<T: Serializable> Serializable for crate::RawVec<T> {
    fn serialize(&self, buf: Buf) {
        (self.0.len() as i32).serialize(buf);
        self.0.iter().for_each(|e| e.serialize(buf))
    }
}

impl Serializable for &[u8] {
    fn serialize(&self, buf: Buf) {
        let len = if self.len() <= 253 {
            buf.push(self.len() as u8);
            self.len() + 1
        } else {
            buf.extend(&[
                254,
                (self.len() & 0xff) as u8,
                ((self.len() >> 8) & 0xff) as u8,
                ((self.len() >> 16) & 0xff) as u8,
            ]);
            self.len()
        };
        let padding = (4 - (len % 4)) % 4;

        buf.extend(*self);
        (0..padding).for_each(|_| buf.push(0));
    }
}

impl Serializable for String {
    fn serialize(&self, buf: Buf) {
        self.as_bytes().serialize(buf)
    }
}

impl Serializable for &str {
    fn serialize(&self, buf: Buf) {
        self.as_bytes().serialize(buf)
    }
}

impl Serializable for Vec<u8> {
    fn serialize(&self, buf: Buf) {
        (&self[..]).serialize(buf)
    }
}
