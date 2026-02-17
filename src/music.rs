#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationValue {
    Whole,
    Half,
    Quarter,
    Eighth,
}

impl DurationValue {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instrument {
    Violin,
    Viola,
    Cello,
    Flute,
    Clarinet,
    Trumpet,
    Horn,
    Timpani,
    Piano,
}

impl Instrument {
    pub const ALL: [Self; 9] = [
        Self::Violin,
        Self::Viola,
        Self::Cello,
        Self::Flute,
        Self::Clarinet,
        Self::Trumpet,
        Self::Horn,
        Self::Timpani,
        Self::Piano,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Violin => "Violino",
            Self::Viola => "Viola",
            Self::Cello => "Violoncelo",
            Self::Flute => "Flauta",
            Self::Clarinet => "Clarinete",
            Self::Trumpet => "Trompete",
            Self::Horn => "Trompa",
            Self::Timpani => "Tímpanos",
            Self::Piano => "Piano",
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
}

impl Pitch {
    pub fn frequency_hz(self) -> f32 {
        let midi = (self.octave as i32 + 1) * 12 + self.class.semitone_offset();
        let semitones_from_a4 = midi - 69;
        440.0 * 2.0_f32.powf(semitones_from_a4 as f32 / 12.0)
    }

    pub fn label(self) -> String {
        format!("{}{}", self.class.label(), self.octave)
    }
}

#[derive(Debug, Clone)]
pub struct NoteEvent {
    pub pitch: Pitch,
    pub duration: DurationValue,
    pub instrument: Instrument,
}

#[derive(Debug, Clone, Default)]
pub struct Score {
    pub notes: Vec<NoteEvent>,
}

impl Score {
    pub fn total_beats(&self) -> f32 {
        self.notes.iter().map(|n| n.duration.beats()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn a4_frequency_is_440() {
        let pitch = Pitch {
            class: PitchClass::A,
            octave: 4,
        };

        assert_relative_eq!(pitch.frequency_hz(), 440.0, epsilon = 0.001);
    }

    #[test]
    fn score_beats_sum_correctly() {
        let score = Score {
            notes: vec![
                NoteEvent {
                    pitch: Pitch {
                        class: PitchClass::C,
                        octave: 4,
                    },
                    duration: DurationValue::Half,
                    instrument: Instrument::Piano,
                },
                NoteEvent {
                    pitch: Pitch {
                        class: PitchClass::G,
                        octave: 4,
                    },
                    duration: DurationValue::Quarter,
                    instrument: Instrument::Piano,
                },
            ],
        };

        assert_relative_eq!(score.total_beats(), 3.0, epsilon = f32::EPSILON);
    }
}
