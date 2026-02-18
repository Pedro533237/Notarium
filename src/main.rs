#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use notarium::audio::engine::{AudioBackend, AudioEngine, AudioEngineConfig};
use notarium::core::project::{KeySignature, NotariumProject, ProjectMetadata, TimeSignature};
use notarium::instruments::registry::InstrumentRegistry;
use notarium::theme::notarium_theme::NotariumTheme;
use notarium::ui::desktop::launch_start_screen;
use notarium::ui::start_screen::{ProjectTemplate, StartScreenState};

fn main() {
    if let Err(error) = run() {
        #[cfg(target_os = "windows")]
        {
            let _ = show_error_dialog(&format!("Notarium falhou ao iniciar:\n{error}"));
        }
        #[cfg(not(target_os = "windows"))]
        {
            eprintln!("Notarium falhou ao iniciar: {error}");
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
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

    launch_start_screen(&project, theme)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn show_error_dialog(message: &str) -> Result<(), String> {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Lpcwstr = *const u16;

    const MB_OK: u32 = 0x0000_0000;
    const MB_ICONERROR: u32 = 0x0000_0010;

    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: Hwnd, text: Lpcwstr, caption: Lpcwstr, typ: u32) -> i32;
    }

    let title = "Notarium - Erro"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let text = message
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let result = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR,
        )
    };

    if result == 0 {
        Err("MessageBoxW retornou falha".to_owned())
    } else {
        Ok(())
    }
}
