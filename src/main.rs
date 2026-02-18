use notarium::audio::engine::{AudioBackend, AudioEngine, AudioEngineConfig};
use notarium::core::project::{KeySignature, NotariumProject, ProjectMetadata, TimeSignature};
use notarium::instruments::registry::InstrumentRegistry;
use notarium::theme::notarium_theme::NotariumTheme;
use notarium::ui::start_screen::{ProjectTemplate, StartScreenState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let theme = NotariumTheme::noturno();

    let mut start_screen = StartScreenState::new(theme);
    start_screen.metadata = ProjectMetadata {
        title: "Nova Partitura".to_owned(),
        subtitle: "Notarium Production Ready".to_owned(),
        composer: "Compositor".to_owned(),
        arranger: "Arranjador".to_owned(),
        copyright: "© 2026".to_owned(),
        key_signature: KeySignature::C,
        time_signature: TimeSignature::new(4, 4)?,
        bpm: 96,
    };
    start_screen.template = ProjectTemplate::Orquestra;

    let registry = InstrumentRegistry::load_embedded()?;
    start_screen.select_template_defaults(&registry)?;

    let project = NotariumProject::from_start_screen(&start_screen)?;

    let mut audio = AudioEngine::new(AudioEngineConfig::default(), AudioBackend::CpuOnly);
    audio.load_project(&project)?;

    println!(
        "Projeto '{}' inicializado com {} instrumentos (tema accent #{:02x}{:02x}{:02x}).",
        project.metadata.title,
        project.instrument_ids.len(),
        theme.accent.r,
        theme.accent.g,
        theme.accent.b
    );

    Ok(())
}
