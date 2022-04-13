#[derive(Debug)]
pub struct CGroupError {
    description: String,
    inner: Option<nix::Error>,
}

impl CGroupError {
    pub fn new<T: Into<String>>(description: T) -> Self {
        Self {
            description.into(),
            None,
        }
    }
    pub fn fromInnerError(inner: nix::Error) -> Self {
        Self {
            format!("nix error: {}", inner),
            Some(inner),
        }
    }
}

impl From<nix::Error> for CGroupError {
    pub fn from(error: nix::Error) -> Self {
        Self::fromInnerError(error)
    }
}