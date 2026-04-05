//! Núcleo musical do Notarium.
//! Contém tipos de teoria musical, estruturas de partitura e operações de edição.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PitchClass {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl PitchClass {
    pub const ALL: [Self; 7] = [
        Self::C,
        Self::D,
        Self::E,
        Self::F,
        Self::G,
        Self::A,
        Self::B,
    ];

    pub fn semitone_base(self) -> i32 {
        match self {
            Self::C => 0,
            Self::D => 2,
            Self::E => 4,
            Self::F => 5,
            Self::G => 7,
            Self::A => 9,
            Self::B => 11,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::A => "A",
            Self::B => "B",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Accidental {
    Flat,
    Natural,
    Sharp,
}

impl Accidental {
    pub fn semitone_delta(self) -> i32 {
        match self {
            Self::Flat => -1,
            Self::Natural => 0,
            Self::Sharp => 1,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Flat => "♭",
            Self::Natural => "♮",
            Self::Sharp => "♯",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pitch {
    pub class: PitchClass,
    pub accidental: Accidental,
    pub octave: i8,
}

impl Pitch {
    pub fn midi_number(self) -> i32 {
        ((self.octave as i32 + 1) * 12)
            + self.class.semitone_base()
            + self.accidental.semitone_delta()
    }

    pub fn frequency_hz(self) -> f32 {
        let semitones_from_a4 = self.midi_number() - 69;
        440.0 * 2f32.powf(semitones_from_a4 as f32 / 12.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
}

impl NoteDuration {
    pub const ALL: [Self; 4] = [Self::Whole, Self::Half, Self::Quarter, Self::Eighth];

    pub fn beats(self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::Half => 2.0,
            Self::Quarter => 1.0,
            Self::Eighth => 0.5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Whole => "Semibreve",
            Self::Half => "Mínima",
            Self::Quarter => "Semínima",
            Self::Eighth => "Colcheia",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeSignature {
    pub beats_per_measure: u8,
    pub beat_unit: u8,
}

impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            beats_per_measure: 4,
            beat_unit: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeySignature {
    C,
    G,
    D,
    A,
    E,
    F,
    Bb,
    Eb,
}

impl KeySignature {
    pub const ALL: [Self; 8] = [
        Self::C,
        Self::G,
        Self::D,
        Self::A,
        Self::E,
        Self::F,
        Self::Bb,
        Self::Eb,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::C => "C",
            Self::G => "G",
            Self::D => "D",
            Self::A => "A",
            Self::E => "E",
            Self::F => "F",
            Self::Bb => "Bb",
            Self::Eb => "Eb",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Instrument {
    Piano,
    Violin,
    Viola,
    Cello,
    Flute,
    Clarinet,
    Trumpet,
}

impl Instrument {
    pub const ALL: [Self; 7] = [
        Self::Piano,
        Self::Violin,
        Self::Viola,
        Self::Cello,
        Self::Flute,
        Self::Clarinet,
        Self::Trumpet,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Piano => "Piano",
            Self::Violin => "Violino",
            Self::Viola => "Viola",
            Self::Cello => "Cello",
            Self::Flute => "Flauta",
            Self::Clarinet => "Clarinete",
            Self::Trumpet => "Trompete",
        }
    }

    pub fn midi_program(self) -> u8 {
        match self {
            Self::Piano => 0,
            Self::Violin => 40,
            Self::Viola => 41,
            Self::Cello => 42,
            Self::Flute => 73,
            Self::Clarinet => 71,
            Self::Trumpet => 56,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: NoteDuration,
    pub beat_offset: f32,
    pub voice: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measure {
    pub number: usize,
    pub notes: Vec<Note>,
}

impl Measure {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            notes: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Staff {
    pub name: String,
    pub instrument: Instrument,
    pub midi_channel: u8,
    pub measures: Vec<Measure>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Score {
    pub title: String,
    pub composer: String,
    pub bpm: u16,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub staves: Vec<Staff>,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            title: "Nova Partitura".to_owned(),
            composer: "Compositor".to_owned(),
            bpm: 120,
            key_signature: KeySignature::C,
            time_signature: TimeSignature::default(),
            staves: vec![Staff {
                name: "Piano".to_owned(),
                instrument: Instrument::Piano,
                midi_channel: 0,
                measures: vec![
                    Measure::new(1),
                    Measure::new(2),
                    Measure::new(3),
                    Measure::new(4),
                ],
            }],
        }
    }
}

impl Score {
    pub fn insert_note(&mut self, staff_idx: usize, measure_idx: usize, note: Note) {
        if let Some(measure) = self
            .staves
            .get_mut(staff_idx)
            .and_then(|s| s.measures.get_mut(measure_idx))
        {
            measure.notes.push(note);
            measure
                .notes
                .sort_by(|a, b| a.beat_offset.total_cmp(&b.beat_offset));
        }
    }

    pub fn move_note(
        &mut self,
        staff_idx: usize,
        measure_idx: usize,
        note_idx: usize,
        new_beat: f32,
    ) {
        if let Some(note) = self
            .staves
            .get_mut(staff_idx)
            .and_then(|s| s.measures.get_mut(measure_idx))
            .and_then(|m| m.notes.get_mut(note_idx))
        {
            note.beat_offset = new_beat.max(0.0);
        }
    }

    pub fn delete_note(&mut self, staff_idx: usize, measure_idx: usize, note_idx: usize) {
        if let Some(measure) = self
            .staves
            .get_mut(staff_idx)
            .and_then(|s| s.measures.get_mut(measure_idx))
        {
            if note_idx < measure.notes.len() {
                measure.notes.remove(note_idx);
            }
        }
    }

    pub fn edit_note_duration(
        &mut self,
        staff_idx: usize,
        measure_idx: usize,
        note_idx: usize,
        duration: NoteDuration,
    ) {
        if let Some(note) = self
            .staves
            .get_mut(staff_idx)
            .and_then(|s| s.measures.get_mut(measure_idx))
            .and_then(|m| m.notes.get_mut(note_idx))
        {
            note.duration = duration;
        }
    }

    pub fn ensure_measure_count(&mut self, min_count: usize) {
        for staff in &mut self.staves {
            while staff.measures.len() < min_count {
                let number = staff.measures.len() + 1;
                staff.measures.push(Measure::new(number));
            }
        }
    }
}
