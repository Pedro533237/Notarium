use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dynamic {
    Ppp,
    Pp,
    P,
    Mp,
    Mf,
    F,
    Ff,
    Fff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Articulation {
    Legato,
    Staccato,
    Tenuto,
    Marcato,
    Accent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaybackStyle {
    pub note_length_factor: f32,
    pub attack_ms: u16,
    pub release_ms: u16,
    pub cc11_target: u8,
}

#[derive(Debug, Clone)]
pub struct ExpressionEngine {
    pub velocity_curve: Vec<f32>,
    pub dynamic_map: HashMap<Dynamic, f32>,
    pub articulation_map: HashMap<Articulation, PlaybackStyle>,
    pub rubato_enabled: bool,
    pub humanize_timing_ms: u8,
    pub humanize_velocity: u8,
}

impl Default for ExpressionEngine {
    fn default() -> Self {
        let velocity_curve = (1..=127).map(|v| (v as f32 / 127.0).powf(0.85)).collect();

        let dynamic_map = HashMap::from([
            (Dynamic::Ppp, 0.15),
            (Dynamic::Pp, 0.23),
            (Dynamic::P, 0.35),
            (Dynamic::Mp, 0.50),
            (Dynamic::Mf, 0.68),
            (Dynamic::F, 0.80),
            (Dynamic::Ff, 0.92),
            (Dynamic::Fff, 1.0),
        ]);

        let articulation_map = HashMap::from([
            (
                Articulation::Legato,
                PlaybackStyle {
                    note_length_factor: 1.0,
                    attack_ms: 8,
                    release_ms: 70,
                    cc11_target: 100,
                },
            ),
            (
                Articulation::Staccato,
                PlaybackStyle {
                    note_length_factor: 0.4,
                    attack_ms: 2,
                    release_ms: 20,
                    cc11_target: 96,
                },
            ),
            (
                Articulation::Tenuto,
                PlaybackStyle {
                    note_length_factor: 0.95,
                    attack_ms: 5,
                    release_ms: 55,
                    cc11_target: 102,
                },
            ),
            (
                Articulation::Marcato,
                PlaybackStyle {
                    note_length_factor: 0.7,
                    attack_ms: 1,
                    release_ms: 35,
                    cc11_target: 114,
                },
            ),
            (
                Articulation::Accent,
                PlaybackStyle {
                    note_length_factor: 0.85,
                    attack_ms: 2,
                    release_ms: 45,
                    cc11_target: 108,
                },
            ),
        ]);

        Self {
            velocity_curve,
            dynamic_map,
            articulation_map,
            rubato_enabled: false,
            humanize_timing_ms: 7,
            humanize_velocity: 6,
        }
    }
}
