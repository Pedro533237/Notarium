use crate::common::id::Id;

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
    BFlat,
    EFlat,
    AFlat,
    DFlat,
    GFlat,
    CFlat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature {
    pub numerator: u8,
    pub denominator: u8,
}

impl TimeSignature {
    pub fn common_time() -> Self {
        Self {
            numerator: 4,
            denominator: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectMetadata {
    pub title: String,
    pub subtitle: String,
    pub composer: String,
    pub arranger: String,
    pub copyright: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreProject {
    pub id: Id,
    pub metadata: ProjectMetadata,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub bpm: u16,
    pub instrument_ids: Vec<Id>,
}

impl ScoreProject {
    pub fn new(metadata: ProjectMetadata) -> Self {
        Self {
            id: Id::new(),
            metadata,
            key_signature: KeySignature::C,
            time_signature: TimeSignature::common_time(),
            bpm: 120,
            instrument_ids: Vec::new(),
        }
    }
}
