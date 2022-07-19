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

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, buf: Buf) {
        self.iter().for_each(|t| t.serialize(buf))
    }
}