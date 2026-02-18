use crate::ui::app::{NotariumUiApp, UiAppError};

pub fn run_desktop_app(app: &mut NotariumUiApp) -> Result<(), UiAppError> {
    #[cfg(target_os = "windows")]
    {
        render_windows_shell(app)
    }

    #[cfg(not(target_os = "windows"))]
    {
        render_console_preview(app);
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn render_console_preview(app: &NotariumUiApp) {
    println!("NOTARIUM");
    println!(
        "Tema accent: #{:02x}{:02x}{:02x}",
        app.theme.accent.r, app.theme.accent.g, app.theme.accent.b
    );
    println!("Partituras encontradas: {}", app.home.score_files.len());
    if app.home.score_files.is_empty() {
        println!("{}", app.home.empty_message);
    }
}

#[cfg(target_os = "windows")]
fn render_windows_shell(app: &mut NotariumUiApp) -> Result<(), UiAppError> {
    use crate::ui::app::AppScreen;
    use crate::ui::components::format_system_time;
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Hinstance = *mut c_void;
    type Hmenu = *mut c_void;
    type Lpcwstr = *const u16;

    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: Hwnd, text: Lpcwstr, caption: Lpcwstr, typ: u32) -> i32;
        fn CreateMenu() -> Hmenu;
        fn AppendMenuW(
            h_menu: Hmenu,
            u_flags: u32,
            u_id_new_item: usize,
            lp_new_item: Lpcwstr,
        ) -> i32;
        fn GetModuleHandleW(module_name: Lpcwstr) -> Hinstance;
    }

    let _ = unsafe { GetModuleHandleW(std::ptr::null()) };
    let menu = unsafe { CreateMenu() };
    if !menu.is_null() {
        let labels = ["File", "Editar", "Visualizar", "Reprodução"];
        for (id, label) in labels.iter().enumerate() {
            let text = wide(label);
            let _ = unsafe { AppendMenuW(menu, 0x0000, 2000 + id, text.as_ptr()) };
        }
    }

    let mut body = String::from("NOTARIUM\n\n➕ Nova Partitura\n\nPartituras:\n");
    if app.home.score_files.is_empty() {
        body.push_str("Nenhuma partitura encontrada.\n");
    } else {
        for file in &app.home.score_files {
            body.push_str(&format!(
                "- {} ({})\n",
                file.name,
                format_system_time(file.last_modified)
            ));
        }
    }

    body.push_str("\nFluxo UI:\n");
    body.push_str("Home -> NewScoreStep1 -> NewScoreStep2 -> Editor\n");
    body.push_str(&format!("Tela atual: {:?}\n", app.state.current_screen));

    if app.state.current_screen == AppScreen::Home {
        app.go_to_new_score_step1();
        app.go_to_new_score_step2();
        app.set_default_music_settings()?;
        app.conclude_new_score()?;
    }

    let title = wide("Notarium UI (CPU)");
    let text = wide(&body);
    let _ = unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            0x0000_0000,
        )
    };
    Ok(())
}

#[cfg(target_os = "windows")]
fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
