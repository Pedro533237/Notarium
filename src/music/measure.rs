use super::duration::NoteDuration;

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

    pub fn beat_unit(self) -> NoteDuration {
        match self {
            Self::SixEight => NoteDuration::Eighth,
            _ => NoteDuration::Quarter,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Measure {
    pub index: usize,
    pub start_beat: f32,
    pub width_ssu: f32,
}

impl Measure {
    pub fn new(index: usize, start_beat: f32, width_ssu: f32) -> Self {
        Self {
            index,
            start_beat,
            width_ssu,
        }
    }
}
