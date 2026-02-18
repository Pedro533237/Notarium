#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl NoteDuration {
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

    pub fn flag_count(self) -> usize {
        match self {
            Self::Whole | Self::Half | Self::Quarter => 0,
            Self::Eighth => 1,
            Self::Sixteenth => 2,
            Self::ThirtySecond => 3,
            Self::SixtyFourth => 4,
        }
    }

    pub fn from_beats(beats: f32) -> Self {
        if beats >= 3.5 {
            Self::Whole
        } else if beats >= 1.5 {
            Self::Half
        } else if beats >= 0.75 {
            Self::Quarter
        } else if beats >= 0.375 {
            Self::Eighth
        } else if beats >= 0.1875 {
            Self::Sixteenth
        } else if beats >= 0.09375 {
            Self::ThirtySecond
        } else {
            Self::SixtyFourth
        }
    }
}
