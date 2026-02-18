use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vst3PluginDescriptor {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vst3Error {
    MissingPath(String),
}

impl fmt::Display for Vst3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPath(path) => write!(f, "plugin path does not exist: {path}"),
        }
    }
}

impl Error for Vst3Error {}

/// Host mínimo para VST3 com foco em separação de responsabilidades.
#[derive(Debug, Default)]
pub struct Vst3Host {
    plugins: Vec<Vst3PluginDescriptor>,
}

impl Vst3Host {
    pub fn register_plugin<P: AsRef<Path>>(
        &mut self,
        id: String,
        name: String,
        vendor: String,
        path: P,
    ) -> Result<(), Vst3Error> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Err(Vst3Error::MissingPath(path_ref.display().to_string()));
        }

        self.plugins.push(Vst3PluginDescriptor {
            id,
            name,
            vendor,
            path: path_ref.to_path_buf(),
        });
        Ok(())
    }

    pub fn plugins(&self) -> &[Vst3PluginDescriptor] {
        &self.plugins
    }
}
