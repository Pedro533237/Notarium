use crate::core::project::NotariumProject;
use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioBackend {
    CpuOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioEngineConfig {
    pub sample_rate: u32,
    pub buffer_size: NonZeroUsize,
    pub max_polyphony: usize,
}

impl Default for AudioEngineConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            buffer_size: NonZeroUsize::new(256).expect("non-zero buffer constant"),
            max_polyphony: 256,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioEngineError {
    EmptyProject,
}

impl Display for AudioEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProject => write!(f, "projeto sem instrumentos"),
        }
    }
}

impl std::error::Error for AudioEngineError {}

#[derive(Debug)]
pub struct AudioEngine {
    pub config: AudioEngineConfig,
    pub backend: AudioBackend,
    pub low_performance_mode: bool,
    loaded_project: Option<NotariumProject>,
}

impl AudioEngine {
    pub fn new(config: AudioEngineConfig, backend: AudioBackend) -> Self {
        Self {
            config,
            backend,
            low_performance_mode: true,
            loaded_project: None,
        }
    }

    pub fn load_project(&mut self, project: &NotariumProject) -> Result<(), AudioEngineError> {
        if project.instrument_ids.is_empty() {
            return Err(AudioEngineError::EmptyProject);
        }

        self.low_performance_mode = project.low_performance_mode;
        self.loaded_project = Some(project.clone());
        Ok(())
    }

    pub fn idle_cpu_budget_percent(&self) -> f32 {
        if self.low_performance_mode {
            1.5
        } else {
            4.0
        }
    }
}
