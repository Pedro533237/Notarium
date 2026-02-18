use crate::core::project::{KeySignature, TimeSignature};
use crate::instruments::registry::{Instrument, InstrumentRegistry};

#[derive(Debug, Clone)]
pub struct NewScoreData {
    pub title: String,
    pub instruments: Vec<Instrument>,
    pub tempo_bpm: u32,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub create_empty_score: bool,
    pub insert_initial_measure: bool,
}

#[derive(Debug, Clone)]
pub struct NewScoreUiState {
    pub query: String,
    pub available: Vec<Instrument>,
    pub selected_index: Option<usize>,
}

impl NewScoreData {
    pub fn default_with_time_signature() -> Result<Self, crate::core::project::ProjectError> {
        Ok(Self {
            title: "Nova Partitura".to_owned(),
            instruments: Vec::new(),
            tempo_bpm: 120,
            key_signature: KeySignature::C,
            time_signature: TimeSignature::new(4, 4)?,
            create_empty_score: true,
            insert_initial_measure: true,
        })
    }
}

impl NewScoreUiState {
    pub fn from_registry(registry: &InstrumentRegistry) -> Self {
        let names = [
            "Violin I",
            "Violin II",
            "Viola",
            "Cello",
            "Flute",
            "Clarinet",
            "Trumpet",
            "Piano",
        ];

        let available = names
            .iter()
            .filter_map(|name| registry.find_by_name(name).cloned())
            .collect::<Vec<_>>();

        Self {
            query: String::new(),
            available,
            selected_index: None,
        }
    }

    pub fn add_selected(&self, data: &mut NewScoreData) {
        if let Some(index) = self.selected_index {
            if let Some(instrument) = self.available.get(index) {
                data.instruments.push(instrument.clone());
            }
        }
    }

    pub fn remove_selected(data: &mut NewScoreData, index: usize) {
        if index < data.instruments.len() {
            let _ = data.instruments.remove(index);
        }
    }

    pub fn reorder_selected(data: &mut NewScoreData, from: usize, to: usize) {
        if from >= data.instruments.len() || to >= data.instruments.len() || from == to {
            return;
        }

        let item = data.instruments.remove(from);
        data.instruments.insert(to, item);
    }
}
