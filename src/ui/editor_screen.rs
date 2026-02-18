use crate::core::project::{KeySignature, TimeSignature};
use crate::instruments::registry::Instrument;

#[derive(Debug, Clone)]
pub struct Score {
    pub title: String,
    pub instruments: Vec<Instrument>,
    pub tempo_bpm: u32,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
}

#[derive(Debug, Clone)]
pub struct EditorState {
    pub score: Score,
    pub zoom: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub current_measure: u32,
}

impl EditorState {
    pub fn new(score: Score) -> Self {
        Self {
            score,
            zoom: 1.0,
            scroll_x: 0.0,
            scroll_y: 0.0,
            current_measure: 1,
        }
    }
}
