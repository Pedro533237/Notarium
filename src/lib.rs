//! Notarium core library.
//!
//! Arquitetura modular para notação musical profissional com foco em
//! compatibilidade com hardware legado (CPU-first, sem dependência em GPU moderna).

pub mod audio;
pub mod common;
pub mod core;
pub mod engraving;
pub mod export;
pub mod instruments;
pub mod playback;
pub mod plugins;
pub mod theme;
pub mod ui;

use std::sync::Arc;

use audio::engine::{AudioBackend, AudioEngine, AudioEngineConfig};
use instruments::repository::InstrumentRepository;
use theme::notarium::{NotariumTheme, ThemeMode};

/// Estado principal da aplicação.
#[derive(Debug)]
pub struct NotariumApp {
    pub theme: NotariumTheme,
    pub audio_engine: Arc<AudioEngine>,
    pub instruments: Arc<InstrumentRepository>,
}

impl NotariumApp {
    /// Cria a aplicação com configurações otimizadas para hardware legado.
    pub fn bootstrap_for_legacy_hardware() -> Self {
        let config = AudioEngineConfig::low_performance();
        let backend = AudioBackend::CpuOnly;
        let engine = Arc::new(AudioEngine::new(config, backend));

        Self {
            theme: NotariumTheme::new(ThemeMode::DarkRed),
            audio_engine: engine,
            instruments: Arc::new(InstrumentRepository::with_builtin()),
        }
    }
}
