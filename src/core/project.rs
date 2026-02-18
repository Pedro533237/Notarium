use crate::ui::start_screen::StartScreenState;
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub type Uuid = String;

fn next_uuid() -> Uuid {
    format!(
        "notarium-{:016x}",
        ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectError {
    MissingField(&'static str),
    InvalidBpm(u16),
    InvalidTimeSignature { numerator: u8, denominator: u8 },
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(f, "campo obrigatório ausente: {field}"),
            Self::InvalidBpm(bpm) => write!(f, "BPM fora do intervalo válido: {bpm}"),
            Self::InvalidTimeSignature {
                numerator,
                denominator,
            } => {
                write!(f, "fórmula de compasso inválida: {numerator}/{denominator}")
            }
        }
    }
}

impl std::error::Error for ProjectError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySignature {
    C,
    G,
    D,
    A,
    E,
    B,
    FSharp,
    CSharp,
    F,
    Bb,
    Eb,
    Ab,
    Db,
    Gb,
    Cb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl TimeSignature {
    pub fn new(numerator: u8, denominator: u8) -> Result<Self, ProjectError> {
        let valid_denom = [1, 2, 4, 8, 16, 32, 64].contains(&denominator);
        if numerator == 0 || !valid_denom {
            return Err(ProjectError::InvalidTimeSignature {
                numerator,
                denominator,
            });
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectMetadata {
    pub title: String,
    pub subtitle: String,
    pub composer: String,
    pub arranger: String,
    pub copyright: String,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub bpm: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotariumProject {
    pub id: Uuid,
    pub metadata: ProjectMetadata,
    pub instrument_ids: Vec<Uuid>,
    pub audio_device: Option<String>,
    pub low_performance_mode: bool,
}

impl NotariumProject {
    pub fn from_start_screen(state: &StartScreenState) -> Result<Self, ProjectError> {
        validate_metadata(&state.metadata)?;

        Ok(Self {
            id: next_uuid(),
            metadata: state.metadata.clone(),
            instrument_ids: state.selected_instruments.clone(),
            audio_device: state.audio_device.clone(),
            low_performance_mode: state.low_performance_mode,
        })
    }
}

fn validate_metadata(metadata: &ProjectMetadata) -> Result<(), ProjectError> {
    if metadata.title.trim().is_empty() {
        return Err(ProjectError::MissingField("title"));
    }
    if metadata.composer.trim().is_empty() {
        return Err(ProjectError::MissingField("composer"));
    }
    if !(20..=300).contains(&metadata.bpm) {
        return Err(ProjectError::InvalidBpm(metadata.bpm));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_metadata, KeySignature, ProjectMetadata, TimeSignature};

    #[test]
    fn validates_metadata_happy_path() {
        let metadata = ProjectMetadata {
            title: "Sinfonia".to_owned(),
            subtitle: String::new(),
            composer: "Composer".to_owned(),
            arranger: String::new(),
            copyright: String::new(),
            key_signature: KeySignature::C,
            time_signature: TimeSignature::new(4, 4).expect("time signature should be valid"),
            bpm: 120,
        };

        assert!(validate_metadata(&metadata).is_ok());
    }
}
