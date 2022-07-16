#[derive(Debug)]
pub enum ParsingError {
    IO(std::io::Error),
    Parsing,
}

impl From<std::io::Error> for ParsingError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<nom::Err<nom::error::Error<&str>>> for ParsingError {
    fn from(_: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Parsing
    }
}
