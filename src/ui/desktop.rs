use crate::ui::app::{NotariumUiApp, UiAppError};

#[cfg(target_os = "windows")]
use crate::ui::app::AppScreen;

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
    println!("UI desktop avançada está disponível no build Windows.");
    println!("Partituras encontradas: {}", app.home.score_files.len());
}

#[cfg(target_os = "windows")]
fn render_windows_shell(app: &mut NotariumUiApp) -> Result<(), UiAppError> {
    use crate::ui::components::format_system_time;
    use std::ffi::c_void;

    type Bool = i32;
    type Hbrush = *mut c_void;
    type Hcursor = *mut c_void;
    type Hdc = *mut c_void;
    type Hinstance = *mut c_void;
    type Hicon = *mut c_void;
    type Hwnd = *mut c_void;
    type Lparam = isize;
    type Lpcwstr = *const u16;
    type Lresult = isize;
    type Uint = u32;
    type Wparam = usize;

    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: Uint,
        w_param: Wparam,
        l_param: Lparam,
        time: u32,
        pt: Point,
        l_private: u32,
    }

    #[repr(C)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[repr(C)]
    struct PaintStruct {
        hdc: Hdc,
        f_erase: Bool,
        rc_paint: Rect,
        f_restore: Bool,
        f_inc_update: Bool,
        rgb_reserved: [u8; 32],
    }

    #[repr(C)]
    struct WndClassW {
        style: Uint,
        lpfn_wnd_proc: Option<extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult>,
        cb_cls_extra: i32,
        cb_wnd_extra: i32,
        h_instance: Hinstance,
        h_icon: Hicon,
        h_cursor: Hcursor,
        hbr_background: Hbrush,
        lpsz_menu_name: Lpcwstr,
        lpsz_class_name: Lpcwstr,
    }

    const WM_DESTROY: Uint = 0x0002;
    const WM_PAINT: Uint = 0x000F;
    const WM_LBUTTONDOWN: Uint = 0x0201;
    const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
    const WS_VISIBLE: u32 = 0x10000000;
    const SW_SHOW: i32 = 5;
    const IDC_ARROW: Lpcwstr = 32512usize as Lpcwstr;
    const COLOR_WINDOW: usize = 5;
    const GWLP_USERDATA: i32 = -21;

    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassW(wnd_class: *const WndClassW) -> u16;
        fn CreateWindowExW(
            ex_style: u32,
            class_name: Lpcwstr,
            window_name: Lpcwstr,
            style: u32,
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            parent: Hwnd,
            menu: *mut c_void,
            instance: Hinstance,
            param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: Uint, w_param: Wparam, l_param: Lparam) -> Lresult;
        fn ShowWindow(hwnd: Hwnd, cmd_show: i32) -> Bool;
        fn UpdateWindow(hwnd: Hwnd) -> Bool;
        fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: Uint, max: Uint) -> Bool;
        fn TranslateMessage(msg: *const Msg) -> Bool;
        fn DispatchMessageW(msg: *const Msg) -> Lresult;
        fn PostQuitMessage(exit_code: i32);
        fn LoadCursorW(instance: Hinstance, cursor_name: Lpcwstr) -> Hcursor;
        fn GetModuleHandleW(name: Lpcwstr) -> Hinstance;
        fn SetWindowLongPtrW(hwnd: Hwnd, index: i32, value: isize) -> isize;
        fn GetWindowLongPtrW(hwnd: Hwnd, index: i32) -> isize;
        fn InvalidateRect(hwnd: Hwnd, rect: *const Rect, erase: Bool) -> Bool;
        fn BeginPaint(hwnd: Hwnd, ps: *mut PaintStruct) -> Hdc;
        fn EndPaint(hwnd: Hwnd, ps: *const PaintStruct) -> Bool;
    }

    #[link(name = "gdi32")]
    extern "system" {
        fn CreateSolidBrush(color: u32) -> Hbrush;
        fn FillRect(hdc: Hdc, rect: *const Rect, brush: Hbrush) -> i32;
        fn DeleteObject(obj: *mut c_void) -> i32;
        fn SetTextColor(hdc: Hdc, color: u32) -> u32;
        fn SetBkMode(hdc: Hdc, mode: i32) -> i32;
        fn TextOutW(hdc: Hdc, x: i32, y: i32, text: Lpcwstr, len: i32) -> Bool;
        fn MoveToEx(hdc: Hdc, x: i32, y: i32, prev: *mut Point) -> Bool;
        fn LineTo(hdc: Hdc, x: i32, y: i32) -> Bool;
    }

    struct WindowModel {
        app: NotariumUiApp,
    }

    extern "system" fn wnd_proc(
        hwnd: Hwnd,
        msg: Uint,
        w_param: Wparam,
        l_param: Lparam,
    ) -> Lresult {
        match msg {
            WM_LBUTTONDOWN => {
                let x = (l_param as u32 & 0xFFFF) as i32;
                let y = ((l_param as u32 >> 16) & 0xFFFF) as i32;
                let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowModel;
                if !ptr.is_null() {
                    let model = unsafe { &mut *ptr };
                    if model.app.state.current_screen == AppScreen::Home {
                        if (1080..=1320).contains(&x) && (92..=140).contains(&y) {
                            model.app.go_to_new_score_step1();
                            let _ = unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
                        }
                    } else if model.app.state.current_screen == AppScreen::NewScoreStep1 {
                        if (1140..=1320).contains(&x) && (760..=810).contains(&y) {
                            model.app.go_to_new_score_step2();
                            let _ = unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
                        }
                    } else if model.app.state.current_screen == AppScreen::NewScoreStep2
                        && (1080..=1320).contains(&x)
                        && (760..=810).contains(&y)
                    {
                        if let Err(_error) = model.app.conclude_new_score() {
                            model.app.back_to_home();
                        }
                        let _ = unsafe { InvalidateRect(hwnd, std::ptr::null(), 1) };
                    }
                }
                0
            }
            WM_PAINT => {
                let mut paint = PaintStruct {
                    hdc: std::ptr::null_mut(),
                    f_erase: 0,
                    rc_paint: Rect {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                    },
                    f_restore: 0,
                    f_inc_update: 0,
                    rgb_reserved: [0; 32],
                };

                let hdc = unsafe { BeginPaint(hwnd, &mut paint) };
                let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowModel;
                if ptr.is_null() {
                    let _ = unsafe { EndPaint(hwnd, &paint) };
                    return 0;
                }

                let model = unsafe { &mut *ptr };
                let theme = model.app.theme;

                let background = Rect {
                    left: 0,
                    top: 0,
                    right: 1366,
                    bottom: 900,
                };
                fill_rect(
                    hdc,
                    &background,
                    rgb(theme.background.r, theme.background.g, theme.background.b),
                );

                let top_bar = Rect {
                    left: 0,
                    top: 0,
                    right: 1366,
                    bottom: 72,
                };
                fill_rect(
                    hdc,
                    &top_bar,
                    rgb(theme.panel.r, theme.panel.g, theme.panel.b),
                );
                draw_text(
                    hdc,
                    18,
                    22,
                    "NOTARIUM",
                    rgb(theme.accent.r, theme.accent.g, theme.accent.b),
                );
                draw_text(
                    hdc,
                    240,
                    24,
                    "Arquivo   Editar   Notação   Reprodução   Layout   Plugins",
                    rgb(theme.text.r, theme.text.g, theme.text.b),
                );

                let ribbon = Rect {
                    left: 0,
                    top: 72,
                    right: 1366,
                    bottom: 160,
                };
                fill_rect(hdc, &ribbon, rgb(theme.panel.r.saturating_add(8), 0, 0));
                fill_rect(
                    hdc,
                    &Rect {
                        left: 1080,
                        top: 92,
                        right: 1320,
                        bottom: 140,
                    },
                    rgb(theme.accent.r, 0, 0),
                );
                draw_text(
                    hdc,
                    1122,
                    109,
                    "+ Nova Partitura",
                    rgb(theme.text.r, theme.text.g, theme.text.b),
                );

                match model.app.state.current_screen {
                    AppScreen::Home => {
                        draw_text(
                            hdc,
                            30,
                            184,
                            "Partituras recentes",
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                        if model.app.home.score_files.is_empty() {
                            draw_text(
                                hdc,
                                30,
                                220,
                                "Nenhuma partitura encontrada em /documents/notarium/",
                                rgb(theme.text.r, theme.text.g, theme.text.b),
                            );
                        } else {
                            for (index, file) in
                                model.app.home.score_files.iter().take(12).enumerate()
                            {
                                let top = 220 + (index as i32 * 48);
                                fill_rect(
                                    hdc,
                                    &Rect {
                                        left: 24,
                                        top,
                                        right: 1328,
                                        bottom: top + 38,
                                    },
                                    rgb(theme.button.r, theme.button.g, theme.button.b),
                                );
                                draw_text(
                                    hdc,
                                    36,
                                    top + 11,
                                    &file.name,
                                    rgb(theme.text.r, theme.text.g, theme.text.b),
                                );
                                let modified = format_system_time(file.last_modified);
                                draw_text(
                                    hdc,
                                    1030,
                                    top + 11,
                                    &modified,
                                    rgb(theme.text.r, theme.text.g, theme.text.b),
                                );
                            }
                        }
                    }
                    AppScreen::NewScoreStep1 => {
                        draw_text(
                            hdc,
                            30,
                            184,
                            "Etapa 1 - Instrumentos",
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                        fill_rect(
                            hdc,
                            &Rect {
                                left: 24,
                                top: 220,
                                right: 670,
                                bottom: 720,
                            },
                            rgb(theme.panel.r, theme.panel.g, theme.panel.b),
                        );
                        fill_rect(
                            hdc,
                            &Rect {
                                left: 694,
                                top: 220,
                                right: 1328,
                                bottom: 720,
                            },
                            rgb(theme.panel.r, theme.panel.g, theme.panel.b),
                        );
                        draw_text(
                            hdc,
                            40,
                            236,
                            "Disponíveis",
                            rgb(theme.accent.r, theme.accent.g, theme.accent.b),
                        );
                        draw_text(
                            hdc,
                            710,
                            236,
                            "Selecionados",
                            rgb(theme.accent.r, theme.accent.g, theme.accent.b),
                        );

                        for (index, instrument) in
                            model.app.new_score_ui.available.iter().take(10).enumerate()
                        {
                            draw_text(
                                hdc,
                                40,
                                272 + (index as i32 * 36),
                                &instrument.name,
                                rgb(theme.text.r, theme.text.g, theme.text.b),
                            );
                        }

                        for (index, instrument) in model
                            .app
                            .new_score_data
                            .instruments
                            .iter()
                            .take(10)
                            .enumerate()
                        {
                            draw_text(
                                hdc,
                                710,
                                272 + (index as i32 * 36),
                                &instrument.name,
                                rgb(theme.text.r, theme.text.g, theme.text.b),
                            );
                        }

                        fill_rect(
                            hdc,
                            &Rect {
                                left: 1140,
                                top: 760,
                                right: 1320,
                                bottom: 810,
                            },
                            rgb(theme.accent.r, 0, 0),
                        );
                        draw_text(
                            hdc,
                            1198,
                            780,
                            "Próximo",
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                    }
                    AppScreen::NewScoreStep2 => {
                        draw_text(
                            hdc,
                            30,
                            184,
                            "Etapa 2 - Configurações",
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                        draw_text(
                            hdc,
                            40,
                            240,
                            &format!("Título: {}", model.app.new_score_data.title),
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                        draw_text(
                            hdc,
                            40,
                            280,
                            &format!("BPM: {}", model.app.new_score_data.tempo_bpm),
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                        draw_text(
                            hdc,
                            40,
                            320,
                            &format!(
                                "Compasso: {}/{}",
                                model.app.new_score_data.time_signature.numerator,
                                model.app.new_score_data.time_signature.denominator
                            ),
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );

                        fill_rect(
                            hdc,
                            &Rect {
                                left: 1080,
                                top: 760,
                                right: 1320,
                                bottom: 810,
                            },
                            rgb(theme.accent.r, 0, 0),
                        );
                        draw_text(
                            hdc,
                            1140,
                            780,
                            "Concluir",
                            rgb(theme.text.r, theme.text.g, theme.text.b),
                        );
                    }
                    AppScreen::Editor => {
                        fill_rect(
                            hdc,
                            &Rect {
                                left: 0,
                                top: 160,
                                right: 280,
                                bottom: 900,
                            },
                            rgb(theme.panel.r, theme.panel.g, theme.panel.b),
                        );
                        draw_text(
                            hdc,
                            20,
                            186,
                            "Instrumentos",
                            rgb(theme.accent.r, theme.accent.g, theme.accent.b),
                        );
                        if let Some(editor) = &model.app.editor {
                            for (index, instrument) in
                                editor.score.instruments.iter().take(18).enumerate()
                            {
                                draw_text(
                                    hdc,
                                    20,
                                    218 + (index as i32 * 30),
                                    &instrument.name,
                                    rgb(theme.text.r, theme.text.g, theme.text.b),
                                );
                            }

                            fill_rect(
                                hdc,
                                &Rect {
                                    left: 300,
                                    top: 180,
                                    right: 1328,
                                    bottom: 840,
                                },
                                rgb(250, 250, 247),
                            );
                            draw_text(hdc, 322, 194, &editor.score.title, rgb(30, 30, 30));
                            for staff in 0..9 {
                                let base_y = 250 + (staff * 62);
                                for line in 0..5 {
                                    let y = base_y + (line * 10);
                                    draw_line(hdc, 360, y, 1270, y, rgb(120, 120, 120));
                                }
                            }
                        }
                    }
                }

                let _ = unsafe { EndPaint(hwnd, &paint) };
                0
            }
            WM_DESTROY => {
                let ptr = unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) } as *mut WindowModel;
                if !ptr.is_null() {
                    let _ = unsafe { Box::from_raw(ptr) };
                    let _ = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) };
                }
                unsafe { PostQuitMessage(0) };
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
        }
    }

    fn rgb(r: u8, g: u8, b: u8) -> u32 {
        (r as u32) | ((g as u32) << 8) | ((b as u32) << 16)
    }

    fn fill_rect(hdc: Hdc, rect: &Rect, color: u32) {
        let brush = unsafe { CreateSolidBrush(color) };
        if !brush.is_null() {
            let _ = unsafe { FillRect(hdc, rect, brush) };
            let _ = unsafe { DeleteObject(brush) };
        }
    }

    fn draw_text(hdc: Hdc, x: i32, y: i32, text: &str, color: u32) {
        let wide = text.encode_utf16().collect::<Vec<u16>>();
        let _ = unsafe { SetTextColor(hdc, color) };
        let _ = unsafe { SetBkMode(hdc, 1) };
        let _ = unsafe { TextOutW(hdc, x, y, wide.as_ptr(), wide.len() as i32) };
    }

    fn draw_line(hdc: Hdc, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let _ = unsafe { SetTextColor(hdc, color) };
        let _ = unsafe { MoveToEx(hdc, x1, y1, std::ptr::null_mut()) };
        let _ = unsafe { LineTo(hdc, x2, y2) };
    }

    let instance = unsafe { GetModuleHandleW(std::ptr::null()) };
    let class_name = wide("NotariumWindow");
    let title = wide("Notarium");

    let wnd_class = WndClassW {
        style: 0,
        lpfn_wnd_proc: Some(wnd_proc),
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance: instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: unsafe { LoadCursorW(std::ptr::null_mut(), IDC_ARROW) },
        hbr_background: (COLOR_WINDOW + 1) as Hbrush,
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_name.as_ptr(),
    };

    let class_result = unsafe { RegisterClassW(&wnd_class) };
    if class_result == 0 {
        return Err(UiAppError::Desktop(
            "falha ao registrar classe de janela".to_owned(),
        ));
    }

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            80,
            40,
            1366,
            860,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        )
    };

    if hwnd.is_null() {
        return Err(UiAppError::Desktop(
            "falha ao criar janela principal".to_owned(),
        ));
    }

    let model = Box::new(WindowModel {
        app: std::mem::replace(app, NotariumUiApp::new_empty()?),
    });
    let model_ptr = Box::into_raw(model);
    let _ = unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, model_ptr as isize) };

    let _ = unsafe { ShowWindow(hwnd, SW_SHOW) };
    let _ = unsafe { UpdateWindow(hwnd) };

    let mut msg = Msg {
        hwnd: std::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt: Point { x: 0, y: 0 },
        l_private: 0,
    };

    while unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) } > 0 {
        let _ = unsafe { TranslateMessage(&msg) };
        let _ = unsafe { DispatchMessageW(&msg) };
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
