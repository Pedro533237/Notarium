use crate::core::project::NotariumProject;
use crate::theme::notarium_theme::NotariumTheme;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DesktopUiError {
    NativeWindowFailure(&'static str),
}

impl Display for DesktopUiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NativeWindowFailure(msg) => {
                write!(f, "falha ao abrir interface nativa do Windows: {msg}")
            }
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
        show_windows_start_window(project, theme)
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!(
            "Notarium {} | {} instrumentos | accent #{:02x}{:02x}{:02x}",
            project.metadata.title,
            project.instrument_ids.len(),
            theme.accent.r,
            theme.accent.g,
            theme.accent.b
        );
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn show_windows_start_window(
    project: &NotariumProject,
    theme: NotariumTheme,
) -> Result<(), DesktopUiError> {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Hinstance = *mut c_void;
    type Hcursor = *mut c_void;
    type Hbrush = *mut c_void;
    type Hicon = *mut c_void;
    type Hmenu = *mut c_void;
    type Lpcwstr = *const u16;
    type Wparam = usize;
    type Lparam = isize;
    type Lresult = isize;
    type Uint = u32;

    #[repr(C)]
    struct WndClassW {
        style: Uint,
        lpfn_wnd_proc: extern "system" fn(Hwnd, Uint, Wparam, Lparam) -> Lresult,
        cb_cls_extra: i32,
        cb_wnd_extra: i32,
        h_instance: Hinstance,
        h_icon: Hicon,
        h_cursor: Hcursor,
        hbr_background: Hbrush,
        lpsz_menu_name: Lpcwstr,
        lpsz_class_name: Lpcwstr,
    }

    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: Uint,
        w_param: Wparam,
        l_param: Lparam,
        time: u32,
        pt_x: i32,
        pt_y: i32,
    }

    const WM_DESTROY: Uint = 0x0002;
    const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
    const WS_VISIBLE: u32 = 0x10000000;
    const WS_CHILD: u32 = 0x40000000;
    const WS_BORDER: u32 = 0x00800000;
    const BS_PUSHBUTTON: u32 = 0x00000000;
    const CW_USEDEFAULT: i32 = i32::MIN;
    const CS_HREDRAW: Uint = 0x0002;
    const CS_VREDRAW: Uint = 0x0001;
    const IDC_ARROW: usize = 32512;
    const COLOR_WINDOW: isize = 5;
    const MF_STRING: u32 = 0x0000;

    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassW(lp_wnd_class: *const WndClassW) -> u16;
        fn CreateWindowExW(
            dw_ex_style: u32,
            lp_class_name: Lpcwstr,
            lp_window_name: Lpcwstr,
            dw_style: u32,
            x: i32,
            y: i32,
            n_width: i32,
            n_height: i32,
            h_wnd_parent: Hwnd,
            h_menu: Hmenu,
            h_instance: Hinstance,
            lp_param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: Uint, wparam: Wparam, lparam: Lparam) -> Lresult;
        fn DispatchMessageW(msg: *const Msg) -> Lresult;
        fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: Uint, max: Uint) -> i32;
        fn PostQuitMessage(code: i32);
        fn TranslateMessage(msg: *const Msg) -> i32;
        fn ShowWindow(hwnd: Hwnd, cmd_show: i32) -> i32;
        fn UpdateWindow(hwnd: Hwnd) -> i32;
        fn LoadCursorW(h_instance: Hinstance, cursor_name: Lpcwstr) -> Hcursor;
        fn GetModuleHandleW(module_name: Lpcwstr) -> Hinstance;
        fn SetWindowTextW(hwnd: Hwnd, text: Lpcwstr) -> i32;
        fn CreateMenu() -> Hmenu;
        fn AppendMenuW(
            h_menu: Hmenu,
            u_flags: u32,
            u_id_new_item: usize,
            lp_new_item: Lpcwstr,
        ) -> i32;
        fn SetMenu(h_wnd: Hwnd, h_menu: Hmenu) -> i32;
        fn DrawMenuBar(h_wnd: Hwnd) -> i32;
    }

    extern "system" fn wnd_proc(hwnd: Hwnd, msg: Uint, wparam: Wparam, lparam: Lparam) -> Lresult {
        match msg {
            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                }
                0
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }

    let class_name = wide("NotariumMainWindow");
    let title = wide("Notarium - Partitura");

    let instance = unsafe { GetModuleHandleW(std::ptr::null()) };
    if instance.is_null() {
        return Err(DesktopUiError::NativeWindowFailure("GetModuleHandleW"));
    }

    let cursor = unsafe { LoadCursorW(std::ptr::null_mut(), IDC_ARROW as Lpcwstr) };

    let window_class = WndClassW {
        style: CS_HREDRAW | CS_VREDRAW,
        lpfn_wnd_proc: wnd_proc,
        cb_cls_extra: 0,
        cb_wnd_extra: 0,
        h_instance: instance,
        h_icon: std::ptr::null_mut(),
        h_cursor: cursor,
        hbr_background: (COLOR_WINDOW + 1) as Hbrush,
        lpsz_menu_name: std::ptr::null(),
        lpsz_class_name: class_name.as_ptr(),
    };

    if unsafe { RegisterClassW(&window_class) } == 0 {
        return Err(DesktopUiError::NativeWindowFailure("RegisterClassW"));
    }

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1320,
            860,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null_mut(),
        )
    };

    if hwnd.is_null() {
        return Err(DesktopUiError::NativeWindowFailure("CreateWindowExW"));
    }

    setup_top_menu(hwnd)?;
    setup_ribbon(hwnd, instance)?;
    setup_score_canvas(hwnd, instance, project, theme)?;

    unsafe {
        let _ = ShowWindow(hwnd, 1);
        let _ = UpdateWindow(hwnd);
    }

    let mut msg = Msg {
        hwnd: std::ptr::null_mut(),
        message: 0,
        w_param: 0,
        l_param: 0,
        time: 0,
        pt_x: 0,
        pt_y: 0,
    };

    loop {
        let code = unsafe { GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) };
        if code == -1 {
            return Err(DesktopUiError::NativeWindowFailure("GetMessageW"));
        }
        if code == 0 {
            break;
        }
        unsafe {
            let _ = TranslateMessage(&msg);
            let _ = DispatchMessageW(&msg);
        }
    }

    fn setup_top_menu(hwnd: Hwnd) -> Result<(), DesktopUiError> {
        #[link(name = "user32")]
        extern "system" {
            fn CreateMenu() -> Hmenu;
            fn AppendMenuW(
                h_menu: Hmenu,
                u_flags: u32,
                u_id_new_item: usize,
                lp_new_item: Lpcwstr,
            ) -> i32;
            fn SetMenu(h_wnd: Hwnd, h_menu: Hmenu) -> i32;
            fn DrawMenuBar(h_wnd: Hwnd) -> i32;
        }

        let menu = unsafe { CreateMenu() };
        if menu.is_null() {
            return Err(DesktopUiError::NativeWindowFailure("CreateMenu"));
        }

        let tabs = [
            "File",
            "Home",
            "Note Input",
            "Notations",
            "Play",
            "Layout",
            "Appearance",
            "Parts",
            "Review",
            "View",
        ];

        for (i, tab) in tabs.iter().enumerate() {
            let label = wide(tab);
            if unsafe { AppendMenuW(menu, MF_STRING, 1000 + i, label.as_ptr()) } == 0 {
                return Err(DesktopUiError::NativeWindowFailure("AppendMenuW"));
            }
        }

        if unsafe { SetMenu(hwnd, menu) } == 0 {
            return Err(DesktopUiError::NativeWindowFailure("SetMenu"));
        }

        unsafe {
            let _ = DrawMenuBar(hwnd);
        }
        Ok(())
    }

    fn setup_ribbon(hwnd: Hwnd, instance: Hinstance) -> Result<(), DesktopUiError> {
        let ribbon_labels = [
            "Nova Partitura",
            "Abrir Projeto",
            "Salvar",
            "Adicionar Instrumento",
            "Transposing Score",
            "Play",
            "Stop",
            "Layout",
            "Parts",
            "Plugins",
        ];

        let button_class = wide("BUTTON");
        let mut x = 10;
        for label in ribbon_labels {
            let text = wide(label);
            let handle = unsafe {
                CreateWindowExW(
                    0,
                    button_class.as_ptr(),
                    text.as_ptr(),
                    WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                    x,
                    42,
                    122,
                    42,
                    hwnd,
                    std::ptr::null_mut(),
                    instance,
                    std::ptr::null_mut(),
                )
            };

            if handle.is_null() {
                return Err(DesktopUiError::NativeWindowFailure(
                    "CreateWindowExW BUTTON",
                ));
            }
            x += 128;
        }

        Ok(())
    }

    fn setup_score_canvas(
        hwnd: Hwnd,
        instance: Hinstance,
        project: &NotariumProject,
        theme: NotariumTheme,
    ) -> Result<(), DesktopUiError> {
        let static_class = wide("STATIC");
        let title = wide(&format!(
            "Partitura: {}   |   Compositor: {}   |   BPM: {}   |   Instrumentos: {}",
            project.metadata.title,
            project.metadata.composer,
            project.metadata.bpm,
            project.instrument_ids.len()
        ));

        let title_handle = unsafe {
            CreateWindowExW(
                0,
                static_class.as_ptr(),
                title.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                12,
                96,
                1260,
                24,
                hwnd,
                std::ptr::null_mut(),
                instance,
                std::ptr::null_mut(),
            )
        };
        if title_handle.is_null() {
            return Err(DesktopUiError::NativeWindowFailure(
                "CreateWindowExW STATIC header",
            ));
        }

        let paper = wide(&format!(
            "Área de Partitura (CPU render placeholder) | Tema Noturno #{:02x}{:02x}{:02x}",
            theme.background.r, theme.background.g, theme.background.b
        ));

        let canvas = unsafe {
            CreateWindowExW(
                0,
                static_class.as_ptr(),
                paper.as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                12,
                124,
                1260,
                680,
                hwnd,
                std::ptr::null_mut(),
                instance,
                std::ptr::null_mut(),
            )
        };

        if canvas.is_null() {
            return Err(DesktopUiError::NativeWindowFailure(
                "CreateWindowExW STATIC canvas",
            ));
        }

        let caption = wide("Notarium Ribbon UI - estilo Sibelius/MuseScore (base)");
        unsafe {
            let _ = SetWindowTextW(hwnd, caption.as_ptr());
        }

        Ok(())
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
