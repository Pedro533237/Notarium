use crate::core::project::NotariumProject;
use crate::theme::notarium_theme::NotariumTheme;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopUiError {
    NativeDialogFailure,
}

impl Display for DesktopUiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NativeDialogFailure => write!(f, "falha ao abrir interface nativa do Windows"),
        }
    }
}

impl std::error::Error for DesktopUiError {}

pub fn launch_start_screen(
    project: &NotariumProject,
    theme: NotariumTheme,
) -> Result<(), DesktopUiError> {
    #[cfg(target_os = "windows")]
    {
        return show_windows_dialog(project, theme);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = project;
        let _ = theme;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn show_windows_dialog(
    project: &NotariumProject,
    theme: NotariumTheme,
) -> Result<(), DesktopUiError> {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Lpcwstr = *const u16;

    const MB_OK: u32 = 0x0000_0000;
    const MB_ICONINFORMATION: u32 = 0x0000_0040;

    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: Hwnd, text: Lpcwstr, caption: Lpcwstr, typ: u32) -> i32;
    }

    let title = "Notarium - Start Screen"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let summary = format!(
        "Projeto: {}\nCompositor: {}\nBPM: {}\nInstrumentos: {}\nTema noturno ativo (accent #{:02x}{:02x}{:02x})\n\nBuild .exe OK. Interface nativa carregada.",
        project.metadata.title,
        project.metadata.composer,
        project.metadata.bpm,
        project.instrument_ids.len(),
        theme.accent.r,
        theme.accent.g,
        theme.accent.b
    );

    let text = summary
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let result = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        )
    };

    if result == 0 {
        Err(DesktopUiError::NativeDialogFailure)
    } else {
        Ok(())
    }
}
