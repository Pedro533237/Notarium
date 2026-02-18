use crate::core::project::NotariumProject;
use crate::engraving::rules::EngravingRules;
use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiType {
    Type0,
    Type1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Mp3,
    Flac,
    Ogg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportError {
    EmptyProject,
}

impl Display for ExportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProject => write!(f, "projeto sem instrumentos para exportação"),
        }
    }
}

impl std::error::Error for ExportError {}

pub trait ScoreExporter {
    fn export_pdf(
        &self,
        project: &NotariumProject,
        _engraving: &EngravingRules,
        _output: &Path,
    ) -> Result<(), ExportError>;

    fn export_audio_offline(
        &self,
        project: &NotariumProject,
        _format: AudioFormat,
        _output: &Path,
    ) -> Result<(), ExportError>;

    fn export_midi(
        &self,
        project: &NotariumProject,
        _midi_type: MidiType,
        _output: &Path,
    ) -> Result<(), ExportError>;
}

#[derive(Debug, Default)]
pub struct CpuExporter;

impl ScoreExporter for CpuExporter {
    fn export_pdf(
        &self,
        project: &NotariumProject,
        _engraving: &EngravingRules,
        _output: &Path,
    ) -> Result<(), ExportError> {
        if project.instrument_ids.is_empty() {
            return Err(ExportError::EmptyProject);
        }
        Ok(())
    }

    fn export_audio_offline(
        &self,
        project: &NotariumProject,
        _format: AudioFormat,
        _output: &Path,
    ) -> Result<(), ExportError> {
        if project.instrument_ids.is_empty() {
            return Err(ExportError::EmptyProject);
        }
        Ok(())
    }

    fn export_midi(
        &self,
        project: &NotariumProject,
        _midi_type: MidiType,
        _output: &Path,
    ) -> Result<(), ExportError> {
        if project.instrument_ids.is_empty() {
            return Err(ExportError::EmptyProject);
        }
        Ok(())
    }
}
