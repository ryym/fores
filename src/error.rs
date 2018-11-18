// https://github.com/rust-lang-nursery/failure
// https://rust-lang-nursery.github.io/failure/error-errorkind.html

use diesel::result::Error as DieselError;
use failure::{Backtrace, Context, Fail};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

#[derive(Debug, Clone, Fail)]
pub enum ErrorKind {
    #[fail(display = "database operation failure")]
    Db,

    #[fail(display = "record not found")]
    NotFound,

    #[fail(display = "{}", _0)]
    Misc(String),
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl<S> From<Context<S>> for Error
where
    S: Into<String> + Display + Sync + Send,
{
    fn from(ctx: Context<S>) -> Error {
        Error {
            inner: ctx.map(|s| ErrorKind::Misc(s.into())),
        }
    }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Error {
        match err {
            DieselError::NotFound => err.context(ErrorKind::NotFound),
            _ => err.context(ErrorKind::Db),
        }.into()
    }
}
