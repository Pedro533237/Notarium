#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotationFeature {
    Slur,
    Ornament,
    Trill,
    Tremolo,
    Harmonic,
    Glissando,
    Contemporary,
    Microtonal,
    IrregularMeter,
    Polyrhythm,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EngravingRules {
    pub min_system_spacing_px: f32,
    pub adaptive_margins: bool,
    pub automatic_system_breaks: bool,
    pub enabled_features: Vec<NotationFeature>,
}

impl Default for EngravingRules {
    fn default() -> Self {
        Self {
            min_system_spacing_px: 72.0,
            adaptive_margins: true,
            automatic_system_breaks: true,
            enabled_features: vec![
                NotationFeature::Slur,
                NotationFeature::Ornament,
                NotationFeature::Trill,
                NotationFeature::Tremolo,
                NotationFeature::Harmonic,
                NotationFeature::Glissando,
                NotationFeature::Contemporary,
                NotationFeature::Microtonal,
                NotationFeature::IrregularMeter,
                NotationFeature::Polyrhythm,
            ],
        }
    }
}
