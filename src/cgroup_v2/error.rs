use std::fmt;

#[derive(Debug)]
pub struct CGroupError {
    description: String,
    inner: Option<CGroupErrorKind>,
}

#[derive(Debug)]
pub enum CGroupErrorKind {
    CGroupErr(),
    IoErr(std::io::Error),
    NixErr(nix::Error),
    ParseErr(String),
}

impl CGroupError {
    pub fn new<T: Into<String>>(description: T) -> Self {
        Self {
            description: description.into(),
            inner: None,
        }
    }
    pub fn from_inner_error(inner: CGroupErrorKind) -> Self {
        Self {
            description: "Inner error".into(),
            inner: Some(inner),
        }
    }
}

impl fmt::Display for CGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        if let Some(err) = &self.inner {
            write!(f, "{:?}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for CGroupError {}

impl serde::de::Error for CGroupError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::from_inner_error(CGroupErrorKind::ParseErr(format!("{}", msg)))
    }
}

impl From<std::io::Error> for CGroupError {
    fn from(error: std::io::Error) -> Self {
        Self::from_inner_error(CGroupErrorKind::IoErr(error))
    }
}

impl From<nix::Error> for CGroupError {
    fn from(error: nix::Error) -> Self {
        Self::from_inner_error(CGroupErrorKind::NixErr(error))
    }
}
