pub mod duration;
pub mod measure;
pub mod note;
pub mod staff;

pub use duration::NoteDuration;
pub use measure::TimeSignature;
pub use note::{Accidental, Note, NoteEvent, NoteId, Pitch, PitchClass, StemDirection};
pub use staff::{Clef, KeySignature, Staff, StaffSystem};

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
pub enum PaperSize {
    A4,
    A3,
    Letter,
}

impl PaperSize {
    pub const ALL: [Self; 3] = [Self::A4, Self::A3, Self::Letter];

    pub fn label(self) -> &'static str {
        match self {
            Self::A4 => "A4",
            Self::A3 => "A3",
            Self::Letter => "Letter",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreSettings {
    pub title: String,
    pub composer: String,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub paper_size: PaperSize,
}

impl Default for ScoreSettings {
    fn default() -> Self {
        Self {
            title: "Nova Partitura".to_owned(),
            composer: "Compositor".to_owned(),
            key_signature: KeySignature::C,
            time_signature: TimeSignature::FourFour,
            paper_size: PaperSize::A4,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Score {
    pub notes: Vec<NoteEvent>,
}

impl Score {
    pub fn total_beats(&self) -> f32 {
        self.notes.iter().map(|n| n.duration.beats()).sum()
    }

    pub fn total_measures(&self, time_signature: TimeSignature) -> f32 {
        self.total_beats() / time_signature.beats_per_measure().max(1.0)
    }
}

pub type DurationValue = NoteDuration;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn a4_frequency_is_440() {
        let pitch = Pitch {
            class: PitchClass::A,
            octave: 4,
            accidental: Accidental::None,
        };

        assert_relative_eq!(pitch.frequency_hz(), 440.0, epsilon = 0.001);
    }

    #[test]
    fn score_beats_sum_correctly() {
        let score = Score {
            notes: vec![
                NoteEvent::new(
                    Pitch {
                        class: PitchClass::C,
                        octave: 4,
                        accidental: Accidental::None,
                    },
                    NoteDuration::Half,
                    Instrument::Piano,
                ),
                NoteEvent::new(
                    Pitch {
                        class: PitchClass::G,
                        octave: 4,
                        accidental: Accidental::None,
                    },
                    NoteDuration::Quarter,
                    Instrument::Piano,
                ),
            ],
        };

        assert_relative_eq!(score.total_beats(), 3.0, epsilon = f32::EPSILON);
        assert_relative_eq!(
            score.total_measures(TimeSignature::ThreeFour),
            1.0,
            epsilon = f32::EPSILON
        );
    }
}
