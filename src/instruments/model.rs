use crate::common::id::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clef {
    Treble,
    Bass,
    Alto,
    Tenor,
    Percussion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pitch {
    pub midi_note: u8,
}

impl Pitch {
    pub const fn new(midi_note: u8) -> Self {
        Self { midi_note }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentFamily {
    Orchestral,
    Baroque,
    Medieval,
    Ethnic,
    Electronic,
    VoiceSatb,
    GlobalPercussion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    pub id: Id,
    pub name: String,
    pub family: InstrumentFamily,
    pub range_low: Pitch,
    pub range_high: Pitch,
    pub transposition: i8,
    pub clef: Clef,
    pub midi_program: u8,
}
