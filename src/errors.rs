use std::fmt;

#[derive(Debug, Clone)]
pub struct VerificationError{
    source: String,
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature is invalid")
    }
}
impl VerificationError{
    pub fn new(source: String) -> Self {
        Self { source: source }
    }
}

#[derive(Debug, Clone)]
pub struct DecryptionError{
    source: String,
}

impl DecryptionError{
    pub fn new(source: String) -> Self {
        Self { source: source }
    }
}
impl fmt::Display for DecryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

#[derive(Debug, Clone)]
pub struct PubKeyDeserializeError{
    source: String,
}

impl PubKeyDeserializeError{
    pub fn new(source: String) -> Self {
        Self { source: source }
    }
}
impl fmt::Display for PubKeyDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}