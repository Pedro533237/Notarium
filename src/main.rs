#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use notarium::instruments::registry::InstrumentRegistry;
use notarium::ui::app::NotariumUiApp;
use notarium::ui::desktop::run_desktop_app;

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
    let registry = InstrumentRegistry::load_embedded()?;
    let mut app = NotariumUiApp::new(&registry)?;
    run_desktop_app(&mut app)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn show_error_dialog(message: &str) -> Result<(), String> {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Lpcwstr = *const u16;

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
            0x0000_0010,
        )
    };

    if result == 0 {
        Err("MessageBoxW retornou falha".to_owned())
    } else {
        Ok(())
    }
}
