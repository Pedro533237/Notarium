#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationValue {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl DurationValue {
    pub const ALL: [Self; 7] = [
        Self::Whole,
        Self::Half,
        Self::Quarter,
        Self::Eighth,
        Self::Sixteenth,
        Self::ThirtySecond,
        Self::SixtyFourth,
    ];

    pub fn beats(self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::Half => 2.0,
            Self::Quarter => 1.0,
            Self::Eighth => 0.5,
            Self::Sixteenth => 0.25,
            Self::ThirtySecond => 0.125,
            Self::SixtyFourth => 0.0625,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Whole => "Semibreve",
            Self::Half => "Mínima",
            Self::Quarter => "Semínima",
            Self::Eighth => "Colcheia",
            Self::Sixteenth => "Semicolcheia",
            Self::ThirtySecond => "Fusa",
            Self::SixtyFourth => "Semifusa",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clef {
    Treble,
    Bass,
    Alto,
    Tenor,
}

impl Clef {
    pub const ALL: [Self; 4] = [Self::Treble, Self::Bass, Self::Alto, Self::Tenor];

    pub fn label(self) -> &'static str {
        match self {
            Self::Treble => "Clave de Sol",
            Self::Bass => "Clave de Fá",
            Self::Alto => "Clave de Dó (Alto)",
            Self::Tenor => "Clave de Dó (Tenor)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicMark {
    Ppp,
    Pp,
    P,
    Mp,
    Mf,
    F,
    Ff,
    Fff,
    Cresc,
    Dim,
    Sfz,
}

impl DynamicMark {
    pub const ALL: [Self; 11] = [
        Self::Ppp,
        Self::Pp,
        Self::P,
        Self::Mp,
        Self::Mf,
        Self::F,
        Self::Ff,
        Self::Fff,
        Self::Cresc,
        Self::Dim,
        Self::Sfz,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Ppp => "ppp",
            Self::Pp => "pp",
            Self::P => "p",
            Self::Mp => "mp",
            Self::Mf => "mf",
            Self::F => "f",
            Self::Ff => "ff",
            Self::Fff => "fff",
            Self::Cresc => "cresc.",
            Self::Dim => "dim.",
            Self::Sfz => "sfz",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Articulation {
    None,
    Staccato,
    Tenuto,
    Accent,
    Marcato,
    Fermata,
}

impl Articulation {
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::Staccato,
        Self::Tenuto,
        Self::Accent,
        Self::Marcato,
        Self::Fermata,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "—",
            Self::Staccato => "Staccato",
            Self::Tenuto => "Tenuto",
            Self::Accent => "Acento",
            Self::Marcato => "Marcato",
            Self::Fermata => "Fermata",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ornament {
    None,
    Trill,
    Mordent,
    Turn,
    Grace,
    Tremolo,
}

impl Ornament {
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::Trill,
        Self::Mordent,
        Self::Turn,
        Self::Grace,
        Self::Tremolo,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "—",
            Self::Trill => "Trinado",
            Self::Mordent => "Mordente",
            Self::Turn => "Grupeto",
            Self::Grace => "Acciaccatura",
            Self::Tremolo => "Tremolo",
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
pub enum KeySignature {
    C,
    G,
    D,
    A,
    E,
    F,
    Bb,
    Eb,
    Ab,
}

impl KeySignature {
    pub const ALL: [Self; 9] = [
        Self::C,
        Self::G,
        Self::D,
        Self::A,
        Self::E,
        Self::F,
        Self::Bb,
        Self::Eb,
        Self::Ab,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::C => "Dó maior / Lá menor",
            Self::G => "Sol maior / Mi menor",
            Self::D => "Ré maior / Si menor",
            Self::A => "Lá maior / Fá# menor",
            Self::E => "Mi maior / Dó# menor",
            Self::F => "Fá maior / Ré menor",
            Self::Bb => "Si♭ maior / Sol menor",
            Self::Eb => "Mi♭ maior / Dó menor",
            Self::Ab => "Lá♭ maior / Fá menor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSignature {
    FourFour,
    ThreeFour,
    TwoFour,
    SixEight,
}

impl TimeSignature {
    pub const ALL: [Self; 4] = [
        Self::FourFour,
        Self::ThreeFour,
        Self::TwoFour,
        Self::SixEight,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::FourFour => "4/4",
            Self::ThreeFour => "3/4",
            Self::TwoFour => "2/4",
            Self::SixEight => "6/8",
        }
    }

    pub fn beats_per_measure(self) -> f32 {
        match self {
            Self::FourFour => 4.0,
            Self::ThreeFour => 3.0,
            Self::TwoFour => 2.0,
            Self::SixEight => 3.0,
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

    pub fn total_measures(&self, time_signature: TimeSignature) -> f32 {
        self.total_beats() / time_signature.beats_per_measure()
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
        assert_relative_eq!(
            score.total_measures(TimeSignature::ThreeFour),
            1.0,
            epsilon = f32::EPSILON
        );
    }
}
