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
    Marcato,
    Tenuto,
    Tremolo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaybackStyle {
    pub gate_ratio: f32,
    pub velocity_multiplier: f32,
    pub timing_offset_ms: f32,
}

#[derive(Debug, Clone)]
pub struct ExpressionEngine {
    pub velocity_curve: Vec<f32>,
    pub dynamic_map: HashMap<Dynamic, f32>,
    pub articulation_map: HashMap<Articulation, PlaybackStyle>,
    pub humanize_timing_ms: f32,
    pub humanize_velocity: f32,
    pub rubato_enabled: bool,
}

impl Default for ExpressionEngine {
    fn default() -> Self {
        let velocity_curve = (0..128)
            .map(|step| {
                let normalized = step as f32 / 127.0;
                normalized.powf(0.85)
            })
            .collect();

        let dynamic_map = HashMap::from([
            (Dynamic::Ppp, 0.18),
            (Dynamic::Pp, 0.25),
            (Dynamic::P, 0.35),
            (Dynamic::Mp, 0.48),
            (Dynamic::Mf, 0.62),
            (Dynamic::F, 0.78),
            (Dynamic::Ff, 0.90),
            (Dynamic::Fff, 1.0),
        ]);

        let articulation_map = HashMap::from([
            (
                Articulation::Legato,
                PlaybackStyle {
                    gate_ratio: 0.98,
                    velocity_multiplier: 0.95,
                    timing_offset_ms: -8.0,
                },
            ),
            (
                Articulation::Staccato,
                PlaybackStyle {
                    gate_ratio: 0.45,
                    velocity_multiplier: 1.1,
                    timing_offset_ms: 4.0,
                },
            ),
            (
                Articulation::Marcato,
                PlaybackStyle {
                    gate_ratio: 0.65,
                    velocity_multiplier: 1.2,
                    timing_offset_ms: 0.0,
                },
            ),
            (
                Articulation::Tenuto,
                PlaybackStyle {
                    gate_ratio: 0.92,
                    velocity_multiplier: 1.0,
                    timing_offset_ms: -2.0,
                },
            ),
            (
                Articulation::Tremolo,
                PlaybackStyle {
                    gate_ratio: 0.5,
                    velocity_multiplier: 0.85,
                    timing_offset_ms: 0.0,
                },
            ),
        ]);

        Self {
            velocity_curve,
            dynamic_map,
            articulation_map,
            humanize_timing_ms: 8.0,
            humanize_velocity: 0.05,
            rubato_enabled: false,
        }
    }
}

impl ExpressionEngine {
    pub fn cc11_for_dynamic(&self, dynamic: Dynamic) -> u8 {
        let value = self.dynamic_map.get(&dynamic).copied().unwrap_or(0.5);
        (value * 127.0).round().clamp(0.0, 127.0) as u8
    }

    pub fn velocity_for_layer(&self, normalized: f32, dynamic: Dynamic) -> u8 {
        let idx = (normalized.clamp(0.0, 1.0) * 127.0).round() as usize;
        let base = self.velocity_curve.get(idx).copied().unwrap_or(0.5);
        let dynamic_scale = self.dynamic_map.get(&dynamic).copied().unwrap_or(0.5);
        (base * dynamic_scale * 127.0).round().clamp(1.0, 127.0) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{Dynamic, ExpressionEngine};

    #[test]
    fn expression_maps_cc11_in_valid_range() {
        let expr = ExpressionEngine::default();
        let cc = expr.cc11_for_dynamic(Dynamic::Mf);
        assert!((1..=127).contains(&cc));
    }
}
