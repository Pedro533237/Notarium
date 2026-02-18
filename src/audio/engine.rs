use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioBackend {
    CpuOnly,
    Auto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioEngineConfig {
    pub sample_rate: u32,
    pub buffer_size: u16,
    pub worker_threads: usize,
    pub low_performance_mode: bool,
}

impl AudioEngineConfig {
    pub fn low_performance() -> Self {
        Self {
            sample_rate: 44_100,
            buffer_size: 512,
            worker_threads: 1,
            low_performance_mode: true,
        }
    }
}

/// Engine de áudio com fallback para execução totalmente em CPU.
#[derive(Debug)]
pub struct AudioEngine {
    config: AudioEngineConfig,
    backend: AudioBackend,
    running: AtomicBool,
}

impl AudioEngine {
    pub fn new(config: AudioEngineConfig, requested_backend: AudioBackend) -> Self {
        let backend = match requested_backend {
            AudioBackend::CpuOnly => AudioBackend::CpuOnly,
            AudioBackend::Auto => AudioBackend::CpuOnly,
        };

        Self {
            config,
            backend,
            running: AtomicBool::new(false),
        }
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn backend(&self) -> AudioBackend {
        self.backend
    }

    pub fn config(&self) -> AudioEngineConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_backend_falls_back_to_cpu() {
        let engine = AudioEngine::new(AudioEngineConfig::low_performance(), AudioBackend::Auto);
        assert_eq!(engine.backend(), AudioBackend::CpuOnly);
    }
}
