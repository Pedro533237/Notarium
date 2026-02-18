use crate::core::project::{KeySignature, ProjectError, ProjectMetadata, TimeSignature, Uuid};
use crate::instruments::registry::InstrumentRegistry;
use crate::theme::notarium_theme::NotariumTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiBackend {
    EguiSoftware,
    IcedSoftware,
    TinySkiaCpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTemplate {
    Orquestra,
    Quarteto,
    Banda,
    Coral,
    Vazio,
}

#[derive(Debug, Clone)]
pub struct StartScreenState {
    pub metadata: ProjectMetadata,
    pub selected_instruments: Vec<Uuid>,
    pub template: ProjectTemplate,
    pub audio_device: Option<String>,
    pub ui_backend: UiBackend,
    pub theme: NotariumTheme,
    pub low_performance_mode: bool,
}

impl StartScreenState {
    pub fn new(theme: NotariumTheme) -> Self {
        Self {
            metadata: ProjectMetadata {
                title: String::new(),
                subtitle: String::new(),
                composer: String::new(),
                arranger: String::new(),
                copyright: String::new(),
                key_signature: KeySignature::C,
                time_signature: TimeSignature {
                    numerator: 4,
                    denominator: 4,
                },
                bpm: 120,
            },
            selected_instruments: Vec::new(),
            template: ProjectTemplate::Vazio,
            audio_device: None,
            ui_backend: UiBackend::TinySkiaCpu,
            theme,
            low_performance_mode: true,
        }
    }

    pub fn select_template_defaults(
        &mut self,
        registry: &InstrumentRegistry,
    ) -> Result<(), ProjectError> {
        let names: &[&str] = match self.template {
            ProjectTemplate::Orquestra => &["Violin I", "Violin II", "Viola", "Cello", "Flute"],
            ProjectTemplate::Quarteto => &["Violin I", "Violin II", "Viola", "Cello"],
            ProjectTemplate::Banda => &["Trumpet", "Tenor Sax", "Trombone", "Drum Set"],
            ProjectTemplate::Coral => &["Soprano Voice", "Alto Voice", "Tenor Voice", "Bass Voice"],
            ProjectTemplate::Vazio => &[],
        };

        self.selected_instruments = names
            .iter()
            .filter_map(|name| {
                registry
                    .find_by_name(name)
                    .map(|instrument| instrument.id.clone())
            })
            .collect();

        Ok(())
    }
}
