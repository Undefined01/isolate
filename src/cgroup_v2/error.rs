use std::fmt;

#[derive(Debug)]
pub struct CGroupError {
    description: String,
    inner: Option<nix::Error>,
}

impl CGroupError {
    pub fn new<T: Into<String>>(description: T) -> Self {
        Self {
            description: description.into(),
            inner: None,
        }
    }
    pub fn fromInnerError(inner: nix::Error) -> Self {
        Self {
            description: format!("nix error: {}", inner),
            inner: Some(inner),
        }
    }
}

impl fmt::Display for CGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description);
    }
}

impl std::error::Error for CGroupError {
}

impl serde::de::Error for CGroupError {
    fn custom<T>(msg: T) -> Self where T: fmt::Display{ 
        Self::new("Deserialize error: {}", msg)
    }
}

impl From<nix::Error> for CGroupError {
    fn from(error: nix::Error) -> Self {
        Self::fromInnerError(error)
    }
}

