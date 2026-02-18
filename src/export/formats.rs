use std::error::Error;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioExportFormat {
    Wav,
    Mp3,
    Flac,
    Ogg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiExportFormat {
    Type0,
    Type1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportError {
    InvalidPath,
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "invalid export path"),
        }
    }
}

impl Error for ExportError {}

pub fn validate_export_target(path: &Path) -> Result<(), ExportError> {
    if path.as_os_str().is_empty() {
        return Err(ExportError::InvalidPath);
    }
    Ok(())
}
