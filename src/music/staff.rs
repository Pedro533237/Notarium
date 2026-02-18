use super::measure::{Measure, TimeSignature};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clef {
    Treble,
    Bass,
    Alto,
}

impl Clef {
    pub const ALL: [Self; 3] = [Self::Treble, Self::Bass, Self::Alto];

    pub fn label(self) -> &'static str {
        match self {
            Self::Treble => "Clave de Sol",
            Self::Bass => "Clave de Fá",
            Self::Alto => "Clave de Dó",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Treble => "𝄞",
            Self::Bass => "𝄢",
            Self::Alto => "𝄡",
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

    pub fn accidental_count(self) -> i8 {
        match self {
            Self::C => 0,
            Self::G => 1,
            Self::D => 2,
            Self::A => 3,
            Self::E => 4,
            Self::F => -1,
            Self::Bb => -2,
            Self::Eb => -3,
            Self::Ab => -4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Staff {
    pub index: usize,
    pub line_count: u8,
    pub line_spacing_ssu: f32,
    pub clef: Clef,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub measures: Vec<Measure>,
}

impl Staff {
    pub fn new(
        index: usize,
        clef: Clef,
        key_signature: KeySignature,
        time_signature: TimeSignature,
    ) -> Self {
        Self {
            index,
            line_count: 5,
            line_spacing_ssu: 1.0,
            clef,
            key_signature,
            time_signature,
            measures: Vec::new(),
        }
    }

    pub fn ensure_measure_count(&mut self, total_beats: f32) {
        let beats_per_measure = self.time_signature.beats_per_measure();
        let needed = (total_beats / beats_per_measure).ceil().max(1.0) as usize;

        if self.measures.len() >= needed {
            return;
        }

        let start = self.measures.len();
        for idx in start..needed {
            self.measures.push(Measure::new(
                idx,
                idx as f32 * beats_per_measure,
                beats_per_measure * 4.0,
            ));
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StaffSystem {
    pub staffs: Vec<Staff>,
}

impl StaffSystem {
    pub fn with_standard_pair(key_signature: KeySignature, time_signature: TimeSignature) -> Self {
        Self {
            staffs: vec![
                Staff::new(0, Clef::Treble, key_signature, time_signature),
                Staff::new(1, Clef::Bass, key_signature, time_signature),
            ],
        }
    }

    pub fn ensure_layout_for_beats(&mut self, total_beats: f32) {
        for staff in &mut self.staffs {
            staff.ensure_measure_count(total_beats);
        }
    }
}
