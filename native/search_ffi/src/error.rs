use std::{error, fmt, io};

#[derive(Copy, Clone, Debug)]
pub struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error While Scrapping this page.")
    }
}

impl error::Error for MyError {}

impl From<anyhow::Error> for MyError {
    fn from(_: anyhow::Error) -> Self {
        Self
    }
}

impl From<io::Error> for MyError {
    fn from(_: io::Error) -> Self {
        Self
    }
}
