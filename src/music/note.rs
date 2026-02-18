use super::{duration::NoteDuration, Instrument};

pub type NoteId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StemDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accidental {
    Sharp,
    Flat,
    Natural,
    None,
}

impl Accidental {
    pub const ALL: [Self; 4] = [Self::None, Self::Sharp, Self::Flat, Self::Natural];

    pub fn semitone(self) -> i32 {
        match self {
            Self::Sharp => 1,
            Self::Flat => -1,
            Self::Natural | Self::None => 0,
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Sharp => "♯",
            Self::Flat => "♭",
            Self::Natural => "♮",
            Self::None => "",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Sharp => "Sustenido",
            Self::Flat => "Bemol",
            Self::Natural => "Bequadro",
            Self::None => "Sem acidente",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn semitone_offset(self) -> i32 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    pub class: PitchClass,
    pub octave: i8,
    pub accidental: Accidental,
}

impl Pitch {
    pub fn frequency_hz(self) -> f32 {
        let midi = (self.octave as i32 + 1) * 12
            + self.class.semitone_offset()
            + self.accidental.semitone();
        let semitones_from_a4 = midi - 69;
        440.0 * 2.0_f32.powf(semitones_from_a4 as f32 / 12.0)
    }

    pub fn midi_number(self) -> i32 {
        (self.octave as i32 + 1) * 12 + self.class.semitone_offset() + self.accidental.semitone()
    }

    pub fn label(self) -> String {
        format!(
            "{}{}{}",
            self.class.label(),
            self.accidental.symbol(),
            self.octave
        )
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub id: NoteId,
    pub pitch: Pitch,
    pub duration: NoteDuration,
    pub staff_position: f32,
    pub stem_direction: StemDirection,
    pub selected: bool,
    pub opacity: f32,
    pub velocity: u8,
    pub dotted: bool,
    pub tie_start: bool,
    pub tie_end: bool,
}

impl Note {
    pub fn new(id: NoteId, pitch: Pitch, duration: NoteDuration, staff_position: f32) -> Self {
        let stem_direction = if staff_position >= 0.0 {
            StemDirection::Down
        } else {
            StemDirection::Up
        };

        Self {
            id,
            pitch,
            duration,
            staff_position,
            stem_direction,
            selected: false,
            opacity: 1.0,
            velocity: 100,
            dotted: false,
            tie_start: false,
            tie_end: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub pitch: Pitch,
    pub duration: NoteDuration,
    pub instrument: Instrument,
    pub dotted: bool,
    pub tie_start: bool,
    pub tie_end: bool,
    pub velocity: u8,
}

impl NoteEvent {
    pub fn new(pitch: Pitch, duration: NoteDuration, instrument: Instrument) -> Self {
        Self {
            pitch,
            duration,
            instrument,
            dotted: false,
            tie_start: false,
            tie_end: false,
            velocity: 100,
        }
    }

    pub fn effective_beats(&self) -> f32 {
        if self.dotted {
            self.duration.beats() * 1.5
        } else {
            self.duration.beats()
        }
    }
}
