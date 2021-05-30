use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ReactorParseError {
    pub(super) error: Box<dyn Error>,
    pub(super) file: String,
}

impl fmt::Display for ReactorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error parsing reactor {}, Gave error {}",
            self.file, self.error
        )
    }
}

impl Error for ReactorParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}

#[derive(Debug)]
pub struct TooManyMutatingReactorsError {
    pub(super) structure: String,
}

impl fmt::Display for TooManyMutatingReactorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Too many mutating reactors for the structure {}",
            self.structure
        )
    }
}

impl Error for TooManyMutatingReactorsError {}

#[derive(Debug)]
pub struct NoStructureError;

impl fmt::Display for NoStructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Please provide a structure definition")
    }
}

impl Error for NoStructureError {}
