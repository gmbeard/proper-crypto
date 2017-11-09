use std::io::{self, BufRead};
use proper_crypto::Source;

#[cfg(target_os = "windows")]
use proper_crypto::native::EncryptionError;

pub struct StdinSource(Vec<u8>);

impl StdinSource {
    pub fn new() -> StdinSource {
        StdinSource(vec![])
    }
}

pub enum StdinTransformError {
    Io(io::Error),
    Transform
}

impl From<io::Error> for StdinTransformError {
    fn from(e: io::Error) -> StdinTransformError {
        StdinTransformError::Io(e)
    }
}

impl From<()> for StdinTransformError {
    fn from(_: ()) -> StdinTransformError {
        StdinTransformError::Transform
    }
}

#[cfg(target_os = "windows")]
impl From<EncryptionError> for StdinTransformError {
    fn from(_: EncryptionError) -> StdinTransformError {
        StdinTransformError::Transform
    }
}

impl Source for StdinSource {
    type Data = [u8];
    type Error = StdinTransformError;

    fn read(&mut self) -> Result<&Self::Data, Self::Error> {
        let mut s = String::with_capacity(64);
        let stdin = io::stdin();
        let mut stdin_lock = stdin.lock();

        stdin_lock.read_line(&mut s)?;

        self.0.clear();
        let end_pos = s.as_bytes()
                       .iter()
                       .position(|&s| s == b'\r' || s == b'\n')
                       .unwrap_or(s.len());

        self.0.extend_from_slice(&s.as_bytes()[..end_pos]);

        Ok(&self.0)
    }
}

