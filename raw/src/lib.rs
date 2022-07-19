pub mod deserialize;
pub mod generated;
pub mod serialize;

pub use deserialize::{Deserializable, Result};
pub use generated::{enums, functions, types};
pub use serialize::Serializable;

pub enum Err {
    IO(std::io::Error),
    InvalidUtf8,
}

impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<std::string::FromUtf8Error> for Err {
    fn from(_: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8
    }
}

impl From<std::str::Utf8Error> for Err {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::InvalidUtf8
    }
}

pub trait Identificable {
    const ID: u32;
}

pub trait Request: Serializable {
    type Response: Deserializable;
}
