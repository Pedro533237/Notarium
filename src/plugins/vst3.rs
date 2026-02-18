use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vst3PluginDescriptor {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessBlockConfig {
    pub sample_rate: u32,
    pub max_buffer_frames: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vst3HostError {
    NotFound(String),
    AlreadyLoaded,
}

impl Display for Vst3HostError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "VST3 não encontrado: {name}"),
            Self::AlreadyLoaded => write!(f, "plugin já carregado"),
        }
    }
}

impl std::error::Error for Vst3HostError {}

pub trait Vst3Host: Send {
    fn scan_paths(&self, paths: &[String]) -> Result<Vec<Vst3PluginDescriptor>, Vst3HostError>;
    fn load(&mut self, plugin: Vst3PluginDescriptor) -> Result<(), Vst3HostError>;
    fn configure_processing(&mut self, config: ProcessBlockConfig);
}

#[derive(Debug, Default)]
pub struct StubVst3Host {
    loaded: Option<Vst3PluginDescriptor>,
    config: Option<ProcessBlockConfig>,
}

impl Vst3Host for StubVst3Host {
    fn scan_paths(&self, paths: &[String]) -> Result<Vec<Vst3PluginDescriptor>, Vst3HostError> {
        let mut discovered = Vec::new();
        for path in paths {
            if path.to_ascii_lowercase().contains("noteperformer") {
                discovered.push(Vst3PluginDescriptor {
                    name: "NotePerformer".to_owned(),
                    vendor: "Wallander Instruments".to_owned(),
                    version: "4.x".to_owned(),
                    path: path.clone(),
                });
            }
        }
        Ok(discovered)
    }

    fn load(&mut self, plugin: Vst3PluginDescriptor) -> Result<(), Vst3HostError> {
        if self.loaded.is_some() {
            return Err(Vst3HostError::AlreadyLoaded);
        }
        self.loaded = Some(plugin);
        Ok(())
    }

    fn configure_processing(&mut self, config: ProcessBlockConfig) {
        self.config = Some(config);
    }
}
