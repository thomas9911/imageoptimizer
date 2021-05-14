#[derive(Debug)]
pub struct Error {
    info: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(info: String) -> Error {
        Error { info }
    }

    pub fn boxed(info: String) -> Box<Error> {
        Box::new(Error::new(info))
    }
}
