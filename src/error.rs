// Axel '0vercl0k' Souchet - December 6th 2024
use std::ffi::FromVecWithNulError;
use std::fmt::{self, Display, Formatter};
use std::num::TryFromIntError;
use std::string::{FromUtf8Error, FromUtf16Error};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Win32(windows_core::Error),
    Other(Box<dyn std::error::Error>),
}

impl From<windows_core::Error> for Error {
    fn from(value: windows_core::Error) -> Self {
        Error::Win32(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::Other(value.into())
    }
}

impl From<FromUtf16Error> for Error {
    fn from(value: FromUtf16Error) -> Self {
        Error::Other(value.into())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::Other(value.into())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Other(value.into())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value.into())
    }
}

impl From<FromVecWithNulError> for Error {
    fn from(value: FromVecWithNulError) -> Self {
        Error::Other(value.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Win32(error) => write!(f, "{error:?}"),
            Error::Other(error) => write!(f, "{error:?}"),
        }
    }
}
