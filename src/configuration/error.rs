use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct ConfError {
    message: String
}

impl ConfError {
    pub fn new(msg: &str) -> Box<ConfError> {
        Box::new(ConfError {
            message: msg.to_owned()
        })
    }
}

impl Error for ConfError {}
impl Display for ConfError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}