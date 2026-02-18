#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContemporaryNotationLevel {
    Disabled,
    Basic,
    Full,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngravingRules {
    pub enable_slurs: bool,
    pub enable_ornaments: bool,
    pub enable_trills: bool,
    pub enable_tremolos: bool,
    pub enable_harmonics: bool,
    pub enable_glissando: bool,
    pub enable_microtones: bool,
    pub max_polyrhythm_depth: u8,
    pub irregular_meter_support: bool,
    pub contemporary_notation: ContemporaryNotationLevel,
}

impl Default for EngravingRules {
    fn default() -> Self {
        Self {
            enable_slurs: true,
            enable_ornaments: true,
            enable_trills: true,
            enable_tremolos: true,
            enable_harmonics: true,
            enable_glissando: true,
            enable_microtones: true,
            max_polyrhythm_depth: 4,
            irregular_meter_support: true,
            contemporary_notation: ContemporaryNotationLevel::Full,
        }
    }
}
