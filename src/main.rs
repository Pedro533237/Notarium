#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Notarium atualmente é suportado apenas no Windows.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Notarium suporta apenas arquiteturas x64 (64-bit).");

mod audio;
mod music;
mod notation;

use egui::{self, ViewportCommand, ViewportId};
use egui_glium::EguiGlium;
use glium::backend::glutin::Display as GliumDisplay;
use glium::glutin;
use glium::winit;
use glium::Surface;
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use std::num::NonZeroU32;
use std::path::PathBuf;

use music::{
    Accidental, Articulation, Clef, DurationValue, DynamicMark, Instrument, KeySignature,
    NoteEvent, Ornament, PaperSize, Pitch, PitchClass, Score, ScoreSettings, TimeSignature,
};

fn main() {
    install_panic_hook();

    if let Err(error_message) = run_notarium() {
        let _ = std::fs::write("notarium.log", &error_message);
        show_startup_error(&error_message);
    }
}

fn run_notarium() -> Result<(), String> {
    let event_loop = winit::event_loop::EventLoop::builder()
        .build()
        .map_err(|err| format!("Falha ao criar event loop: {err}"))?;

    let window_attributes = winit::window::Window::default_attributes()
        .with_title("Notarium")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 800));

    let (window, display) = build_display_with_opengl2_fallback(&event_loop, window_attributes)?;

    let mut egui = EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);
    let mut app = NotariumApp::default();

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested
                    | winit::event::WindowEvent::Destroyed => {
                        app.playback.stop();
                        window_target.exit();
                    }
                    winit::event::WindowEvent::Resized(new_size) => {
                        display.resize((new_size.width, new_size.height));
                        window.request_redraw();
                    }
                    winit::event::WindowEvent::RedrawRequested => {
                        egui.run(&window, |ctx| {
                            app.update(ctx);
                        });

                        let mut target = display.draw();
                        target.clear_color(0.07, 0.07, 0.08, 1.0);
                        egui.paint(&display, &mut target);
                        if let Err(err) = target.finish() {
                            let message = format!("Falha ao finalizar frame OpenGL: {err}");
                            let _ = std::fs::write("notarium.log", &message);
                            show_startup_error(&message);
                            window_target.exit();
                        }
                    }
                    other => {
                        let response = egui.on_event(&window, &other);
                        if response.repaint {
                            window.request_redraw();
                        }
                    }
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .map_err(|err| format!("Falha no loop principal da janela: {err}"))
}

fn build_display_with_opengl2_fallback(
    event_loop: &winit::event_loop::EventLoop<()>,
    window_attributes: winit::window::WindowAttributes,
) -> Result<
    (
        winit::window::Window,
        GliumDisplay<glutin::surface::WindowSurface>,
    ),
    String,
> {
    use glium::glutin::config::ConfigTemplateBuilder;
    use glium::glutin::context::{ContextApi, ContextAttributesBuilder, GlProfile, Version};
    use glium::glutin::display::GetGlDisplay;
    use glium::glutin::prelude::*;
    use glium::glutin::surface::SurfaceAttributesBuilder;

    let template = ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

    let (window_opt, gl_config) = display_builder
        .build(event_loop, template, |mut configs| {
            configs
                .next()
                .expect("Nenhuma configuração OpenGL disponível")
        })
        .map_err(|err| format!("Falha ao selecionar configuração OpenGL: {err}"))?;

    let window = window_opt.ok_or_else(|| "Falha ao criar janela principal.".to_owned())?;

    let window_handle = window
        .window_handle()
        .map_err(|err| format!("Falha ao obter handle da janela: {err}"))?;

    let (w, h): (u32, u32) = window.inner_size().into();
    let width =
        NonZeroU32::new(w.max(1)).ok_or_else(|| "Largura inválida da janela.".to_owned())?;
    let height =
        NonZeroU32::new(h.max(1)).ok_or_else(|| "Altura inválida da janela.".to_owned())?;

    let surface_attributes = SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .build(window_handle.into(), width, height);

    let surface = unsafe {
        gl_config
            .display()
            .create_window_surface(&gl_config, &surface_attributes)
            .map_err(|err| format!("Falha ao criar superfície OpenGL: {err}"))?
    };

    let context_apis = [
        ContextApi::OpenGl(Some(Version::new(2, 1))),
        ContextApi::OpenGl(Some(Version::new(2, 0))),
        ContextApi::Gles(Some(Version::new(2, 0))),
        ContextApi::OpenGl(None),
    ];

    let mut errors = Vec::new();
    let mut current_context = None;

    for api in context_apis {
        let attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Compatibility)
            .with_context_api(api)
            .build(Some(window_handle.into()));

        let not_current = unsafe { gl_config.display().create_context(&gl_config, &attributes) };
        match not_current {
            Ok(ctx) => match ctx.make_current(&surface) {
                Ok(current) => {
                    current_context = Some(current);
                    break;
                }
                Err(err) => errors.push(format!("{api:?} (make_current): {err}")),
            },
            Err(err) => errors.push(format!("{api:?} (create_context): {err}")),
        }
    }

    let context = current_context.ok_or_else(|| {
        format!(
            "Falha ao criar janela/contexto OpenGL compatível com OpenGL 2.0. Tentativas: {}",
            errors.join(" | ")
        )
    })?;

    let display = GliumDisplay::new(context, surface)
        .map_err(|err| format!("Falha ao inicializar renderer glium: {err}"))?;

    Ok((window, display))
}

#[cfg(target_os = "windows")]
fn show_startup_error(message: &str) {
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Lpcwstr = *const u16;

    const MB_ICONERROR: u32 = 0x0000_0010;
    const MB_OK: u32 = 0x0000_0000;

    #[link(name = "user32")]
    extern "system" {
        fn MessageBoxW(hwnd: Hwnd, text: Lpcwstr, caption: Lpcwstr, typ: u32) -> i32;
    }

    let title = "Notarium - Falha ao iniciar"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let full_text = format!(
        "Não foi possível iniciar o Notarium.\n\nDetalhes:\n{}\nLog salvo em notarium.log",
        message
    );

    let text = full_text
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    unsafe {
        let _ = MessageBoxW(
            std::ptr::null_mut(),
            text.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn show_startup_error(message: &str) {
    eprintln!("{}", message);
}

fn install_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let message = format!("Pânico fatal ao iniciar/rodar o Notarium:\n{panic_info}");
        let _ = std::fs::write("notarium.log", &message);
        show_startup_error(&message);
    }));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppScreen {
    Start,
    Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiTab {
    File,
    Home,
    NoteInput,
    Notations,
    Play,
    Layout,
    Appearance,
    Parts,
    Review,
    View,
}

struct NotariumApp {
    score: Score,
    settings: ScoreSettings,
    start_title: String,
    start_composer: String,
    start_key_signature: KeySignature,
    start_time_signature: TimeSignature,
    start_paper_size: PaperSize,
    selected_duration: DurationValue,
    selected_instrument: Instrument,
    bpm: f32,
    screen: AppScreen,
    active_tab: UiTab,
    playback: audio::PlaybackController,
    is_paused: bool,
    orchestral_order: Vec<Instrument>,
    zoom_percent: f32,
    file_path_input: String,
    start_message: String,
    recent_scores: Vec<PathBuf>,
    start_home_tab: StartHomeTab,
    score_view_mode: ScoreViewMode,
    selected_clef: Clef,
    selected_dynamic: DynamicMark,
    selected_articulation: Articulation,
    selected_ornament: Ornament,
    selected_tool: notation::KeyboardTool,
    selected_accidental: Accidental,
    keyboard_open: bool,
    keyboard_page: KeyboardPage,
    playback_config: audio::PlaybackConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartHomeTab {
    Recent,
    Online,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScoreViewMode {
    SinglePage,
    FacingPages,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeyboardPage {
    One,
    Two,
    Three,
    Four,
    All,
}

impl Default for NotariumApp {
    fn default() -> Self {
        let settings = ScoreSettings::default();

        Self {
            score: Score::default(),
            start_title: settings.title.clone(),
            start_composer: settings.composer.clone(),
            start_key_signature: settings.key_signature,
            start_time_signature: settings.time_signature,
            start_paper_size: settings.paper_size,
            settings,
            selected_duration: DurationValue::Quarter,
            selected_instrument: Instrument::Violin,
            bpm: 110.0,
            screen: AppScreen::Start,
            active_tab: UiTab::Home,
            playback: audio::create_playback_controller(),
            is_paused: false,
            orchestral_order: vec![
                Instrument::Flute,
                Instrument::Clarinet,
                Instrument::Horn,
                Instrument::Trumpet,
                Instrument::Violin,
                Instrument::Viola,
                Instrument::Cello,
                Instrument::Timpani,
                Instrument::Piano,
            ],
            zoom_percent: 62.5,
            file_path_input: "notarium_score.ntr".to_owned(),
            start_message: "Pronto para criar ou abrir partitura.".to_owned(),
            recent_scores: find_recent_ntr_files(),
            start_home_tab: StartHomeTab::Recent,
            score_view_mode: ScoreViewMode::FacingPages,
            selected_clef: Clef::Treble,
            selected_dynamic: DynamicMark::Mf,
            selected_articulation: Articulation::None,
            selected_ornament: Ornament::None,
            selected_tool: notation::KeyboardTool::None,
            selected_accidental: Accidental::Natural,
            keyboard_open: true,
            keyboard_page: KeyboardPage::All,
            playback_config: audio::PlaybackConfig::default(),
        }
    }
}

impl NotariumApp {
    fn update(&mut self, ctx: &egui::Context) {
        match self.screen {
            AppScreen::Start => self.render_start_screen(ctx),
            AppScreen::Editor => self.render_editor(ctx),
        }
    }

    fn create_new_score_from_start(&mut self) {
        self.settings = ScoreSettings {
            title: self.start_title.trim().to_owned(),
            composer: self.start_composer.trim().to_owned(),
            key_signature: self.start_key_signature,
            time_signature: self.start_time_signature,
            paper_size: self.start_paper_size,
        };
        self.score.notes.clear();
        self.screen = AppScreen::Editor;
        self.start_message = "Nova partitura criada.".to_owned();
    }

    fn save_ntr(&mut self) {
        let path = sanitized_ntr_path(&self.file_path_input);
        let payload = serialize_ntr(
            &self.settings,
            &self.score,
            self.bpm,
            self.start_key_signature,
            self.start_time_signature,
            self.start_paper_size,
        );

        match std::fs::write(&path, payload) {
            Ok(()) => {
                self.start_message = format!("Partitura salva em {}", path.display());
                self.recent_scores = find_recent_ntr_files();
            }
            Err(err) => {
                self.start_message = format!("Falha ao salvar .ntr: {err}");
            }
        }
    }

    fn open_ntr_from_path(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(contents) => match deserialize_ntr(&contents) {
                Ok((settings, score, bpm)) => {
                    self.settings = settings.clone();
                    self.score = score;
                    self.bpm = bpm;
                    self.start_title = settings.title;
                    self.start_composer = settings.composer;
                    self.start_key_signature = settings.key_signature;
                    self.start_time_signature = settings.time_signature;
                    self.start_paper_size = settings.paper_size;
                    self.file_path_input = path.to_string_lossy().to_string();
                    self.screen = AppScreen::Editor;
                    self.start_message = format!("Arquivo carregado: {}", path.display());
                    self.recent_scores = find_recent_ntr_files();
                }
                Err(err) => {
                    self.start_message = format!("Falha ao ler .ntr: {err}");
                }
            },
            Err(err) => {
                self.start_message = format!("Não foi possível abrir arquivo: {err}");
            }
        }
    }

    fn open_ntr_from_input(&mut self) {
        let path = sanitized_ntr_path(&self.file_path_input);
        self.open_ntr_from_path(path);
    }

    fn render_start_screen(&mut self, ctx: &egui::Context) {
        let bg = egui::Color32::from_rgb(33, 35, 41);
        let panel = egui::Color32::from_rgb(42, 45, 53);
        let accent = egui::Color32::from_rgb(0, 170, 255);
        let mut visuals = egui::Visuals::dark();
        visuals.window_fill = panel;
        visuals.panel_fill = bg;
        visuals.widgets.active.bg_fill = accent;
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(64, 67, 76);
        visuals.extreme_bg_color = egui::Color32::from_rgb(31, 33, 39);
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("start_top_nav")
            .exact_height(44.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (label, selected) in [
                        ("Home", true),
                        ("Score", false),
                        ("Publish", false),
                        ("Learn", false),
                    ] {
                        let text = if selected {
                            egui::RichText::new(label)
                                .strong()
                                .color(egui::Color32::from_rgb(215, 235, 255))
                        } else {
                            egui::RichText::new(label).color(egui::Color32::from_rgb(176, 185, 202))
                        };
                        let _ = ui.add_sized([86.0, 28.0], egui::Button::new(text));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add_sized([86.0, 28.0], egui::Button::new("Sair"))
                            .clicked()
                        {
                            self.playback.stop();
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                });
            });

        egui::SidePanel::left("start_sidebar")
            .resizable(false)
            .exact_width(230.0)
            .show(ctx, |ui| {
                ui.add_space(12.0);
                ui.label(
                    egui::RichText::new("🎵 Notarium Team")
                        .size(22.0)
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new("Composição • Arranjo • Produção")
                        .color(egui::Color32::from_rgb(164, 173, 188)),
                );
                ui.add_space(18.0);

                for item in ["Scores", "Plugins", "Muse Sounds", "Learn", "Cloud"] {
                    let _ = ui.add_sized([200.0, 34.0], egui::Button::new(item));
                    ui.add_space(4.0);
                }

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Atalhos")
                        .strong()
                        .color(egui::Color32::WHITE),
                );
                ui.label("Ctrl+N  Nova partitura");
                ui.label("Ctrl+O  Abrir .ntr");
                ui.label("Space   Play/Pause");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading(
                egui::RichText::new("Scores")
                    .size(44.0)
                    .strong()
                    .color(egui::Color32::WHITE),
            );

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.start_home_tab, StartHomeTab::Recent, "New & recent");
                ui.selectable_value(&mut self.start_home_tab, StartHomeTab::Online, "My online scores");
            });

            ui.add_space(8.0);
            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Nova Partitura");
                    ui.label("Configure os metadados e abra o editor com visual estilo Sibelius.");
                    ui.separator();
                    ui.label("Nome da partitura");
                    ui.text_edit_singleline(&mut self.start_title);
                    ui.label("Nome do compositor");
                    ui.text_edit_singleline(&mut self.start_composer);
                    egui::ComboBox::from_label("Tonalidade")
                        .selected_text(self.start_key_signature.label())
                        .show_ui(ui, |ui| {
                            for key in KeySignature::ALL {
                                ui.selectable_value(&mut self.start_key_signature, key, key.label());
                            }
                        });
                    egui::ComboBox::from_label("Fórmula de compasso")
                        .selected_text(self.start_time_signature.label())
                        .show_ui(ui, |ui| {
                            for time in TimeSignature::ALL {
                                ui.selectable_value(&mut self.start_time_signature, time, time.label());
                            }
                        });
                    egui::ComboBox::from_label("Tamanho do papel")
                        .selected_text(self.start_paper_size.label())
                        .show_ui(ui, |ui| {
                            for size in PaperSize::ALL {
                                ui.selectable_value(&mut self.start_paper_size, size, size.label());
                            }
                        });
                    ui.add(egui::Slider::new(&mut self.bpm, 40.0..=220.0).text("BPM inicial"));

                    if ui
                        .add_sized([230.0, 34.0], egui::Button::new("✨ Criar e Abrir Editor"))
                        .clicked()
                    {
                        self.create_new_score_from_start();
                    }
                });

                columns[1].group(|ui| {
                    ui.heading("Partituras .ntr");
                    ui.label("Abra e salve seus projetos locais.");
                    ui.separator();
                    ui.label("Caminho do arquivo (.ntr)");
                    ui.text_edit_singleline(&mut self.file_path_input);
                    ui.horizontal(|ui| {
                        if ui.button("📂 Abrir .ntr").clicked() {
                            self.open_ntr_from_input();
                        }
                        if ui.button("💾 Salvar .ntr").clicked() {
                            self.save_ntr();
                        }
                        if ui.button("🔄 Atualizar").clicked() {
                            self.recent_scores = find_recent_ntr_files();
                        }
                    });

                    ui.separator();
                    egui::ScrollArea::vertical().max_height(220.0).show(ui, |ui| {
                        if self.recent_scores.is_empty() {
                            ui.label("Nenhum arquivo .ntr encontrado no diretório atual.");
                        }
                        let recent = self.recent_scores.clone();
                        for path in recent {
                            let label = path
                                .file_name()
                                .and_then(|f| f.to_str())
                                .unwrap_or("arquivo.ntr");
                            if ui.add_sized([290.0, 26.0], egui::Button::new(format!("🎼 {label}"))).clicked() {
                                self.open_ntr_from_path(path);
                            }
                        }
                    });
                });
            });

            ui.add_space(8.0);
            match self.start_home_tab {
                StartHomeTab::Recent => {
                    ui.label(egui::RichText::new("New & recent").strong().size(20.0));
                    ui.horizontal_wrapped(|ui| {
                        for preview in [
                            "Concerto em Ré - Strings",
                            "Suite de Câmara",
                            "Piano Lead Sheet",
                        ] {
                            egui::Frame::group(ui.style())
                                .fill(egui::Color32::from_rgb(50, 53, 61))
                                .show(ui, |ui| {
                                    ui.set_min_size(egui::vec2(180.0, 136.0));
                                    ui.vertical_centered(|ui| {
                                        ui.label(egui::RichText::new("♪ ♫ ♬").size(30.0));
                                        ui.add_space(6.0);
                                        ui.label(preview);
                                    });
                                });
                        }
                    });
                }
                StartHomeTab::Online => {
                    ui.label(egui::RichText::new("My online scores").strong().size(20.0));
                    ui.label("Integração cloud preparada para sincronização futura de projetos Notarium.");
                }
            }

            ui.add_space(6.0);
            ui.label(egui::RichText::new(&self.start_message).color(egui::Color32::from_rgb(139, 231, 184)));
        });
    }

    fn render_editor(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (tab, label) in [
                    (UiTab::File, "File"),
                    (UiTab::Home, "Home"),
                    (UiTab::NoteInput, "Notas"),
                    (UiTab::Notations, "Símbolos"),
                    (UiTab::Play, "Play"),
                    (UiTab::Layout, "Layout"),
                    (UiTab::Appearance, "Appearance"),
                    (UiTab::Parts, "Parts"),
                    (UiTab::Review, "Review"),
                    (UiTab::View, "View"),
                ] {
                    ui.selectable_value(&mut self.active_tab, tab, label);
                }
            });

            ui.separator();
            ui.horizontal_wrapped(|ui| {
                ui.heading(self.settings.title.as_str());
                ui.separator();
                ui.label(format!("Compositor: {}", self.settings.composer));
                ui.separator();
                ui.label(format!(
                    "Tonalidade: {}",
                    self.settings.key_signature.label()
                ));
                ui.separator();
                ui.label(format!(
                    "Compasso: {}",
                    self.settings.time_signature.label()
                ));
                ui.separator();
                ui.label(format!("Papel: {}", self.settings.paper_size.label()));
                ui.separator();
                ui.label(format!(
                    "Compassos: {:.1}",
                    self.score.total_measures(self.settings.time_signature)
                ));
                ui.separator();
                if ui.button("💾 Salvar .ntr").clicked() {
                    self.save_ntr();
                }
                if ui.button("📂 Abrir .ntr").clicked() {
                    self.open_ntr_from_input();
                }
                if ui.button("← Voltar para Início").clicked() {
                    self.screen = AppScreen::Start;
                }
                if ui.button("✖ Sair").clicked() {
                    self.playback.stop();
                    ctx.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });

        egui::SidePanel::left("controls")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.heading("Entrada de Notas");
                ui.label("Clique diretamente na pauta para inserir na posição desejada.");
                ui.separator();

                egui::ComboBox::from_label("Instrumento")
                    .selected_text(self.selected_instrument.label())
                    .show_ui(ui, |ui| {
                        for instrument in Instrument::ALL {
                            ui.selectable_value(
                                &mut self.selected_instrument,
                                instrument,
                                instrument.label(),
                            );
                        }
                    });

                egui::ComboBox::from_label("Clave")
                    .selected_text(self.selected_clef.label())
                    .show_ui(ui, |ui| {
                        for clef in Clef::ALL {
                            ui.selectable_value(&mut self.selected_clef, clef, clef.label());
                        }
                    });

                egui::ComboBox::from_label("Duração")
                    .selected_text(self.selected_duration.label())
                    .show_ui(ui, |ui| {
                        for duration in DurationValue::ALL {
                            ui.selectable_value(
                                &mut self.selected_duration,
                                duration,
                                duration.label(),
                            );
                        }
                    });

                ui.horizontal(|ui| {
                    ui.label("Acidente:");
                    for accidental in Accidental::ALL {
                        ui.selectable_value(
                            &mut self.selected_accidental,
                            accidental,
                            accidental.label(),
                        );
                    }
                });

                egui::ComboBox::from_label("Dinâmica")
                    .selected_text(self.selected_dynamic.label())
                    .show_ui(ui, |ui| {
                        for dynamic in DynamicMark::ALL {
                            ui.selectable_value(
                                &mut self.selected_dynamic,
                                dynamic,
                                dynamic.label(),
                            );
                        }
                    });

                egui::ComboBox::from_label("Articulação")
                    .selected_text(self.selected_articulation.label())
                    .show_ui(ui, |ui| {
                        for articulation in Articulation::ALL {
                            ui.selectable_value(
                                &mut self.selected_articulation,
                                articulation,
                                articulation.label(),
                            );
                        }
                    });

                egui::ComboBox::from_label("Ornamento")
                    .selected_text(self.selected_ornament.label())
                    .show_ui(ui, |ui| {
                        for ornament in Ornament::ALL {
                            ui.selectable_value(
                                &mut self.selected_ornament,
                                ornament,
                                ornament.label(),
                            );
                        }
                    });

                ui.horizontal(|ui| {
                    ui.label("Modo de visualização:");
                    ui.selectable_value(
                        &mut self.score_view_mode,
                        ScoreViewMode::SinglePage,
                        "Página única",
                    );
                    ui.selectable_value(
                        &mut self.score_view_mode,
                        ScoreViewMode::FacingPages,
                        "Duas páginas",
                    );
                });

                if ui.button("Adicionar nota (manual)").clicked() {
                    self.inserir_nota(
                        0,
                        self.score.notes.len(),
                        Pitch {
                            class: PitchClass::C,
                            octave: 4,
                        },
                        self.selected_duration,
                        self.selected_instrument,
                    );
                }

                if ui.button("Limpar Partitura").clicked() {
                    self.score.notes.clear();
                }

                ui.separator();
                ui.collapsing("🎛 Play > Mixer e reprodução", |ui| {
                    egui::ComboBox::from_label("Engine")
                        .selected_text(match self.playback_config.engine {
                            audio::PlaybackEngine::Notarium => "Notarium Engine",
                            audio::PlaybackEngine::Vst => "VST Host",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.playback_config.engine,
                                audio::PlaybackEngine::Notarium,
                                "Notarium Engine",
                            );
                            ui.selectable_value(
                                &mut self.playback_config.engine,
                                audio::PlaybackEngine::Vst,
                                "VST Host",
                            );
                        });

                    ui.checkbox(
                        &mut self.playback_config.noteperformer_profile,
                        "Perfil NotePerformer",
                    );

                    if self.playback_config.engine == audio::PlaybackEngine::Vst {
                        ui.label("Roteamento VST (host/plugin)");
                        ui.label("Host:");
                        ui.text_edit_singleline(&mut self.playback_config.vst_host);
                        ui.label("Plugin VST:");
                        ui.text_edit_singleline(&mut self.playback_config.vst_plugin);
                    }

                    ui.add(egui::Slider::new(&mut self.bpm, 40.0..=220.0).text("BPM"));
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("⏮").clicked() {
                            self.playback.rewind();
                            self.is_paused = false;
                        }
                        if ui.button("▶ Play").clicked() {
                            self.playback.play(
                                self.score.clone(),
                                self.bpm,
                                self.playback_config.clone(),
                            );
                            self.is_paused = false;
                        }
                        if ui
                            .button(if self.is_paused {
                                "⏵ Retomar"
                            } else {
                                "⏸ Pausar"
                            })
                            .clicked()
                        {
                            if self.is_paused {
                                self.playback.resume();
                            } else {
                                self.playback.pause();
                            }
                            self.is_paused = !self.is_paused;
                        }
                        if ui.button("⏹").clicked() {
                            self.playback.stop();
                            self.is_paused = false;
                        }
                    });
                });

                ui.separator();
                self.render_notation_catalog(ui);
                ui.separator();
                ui.label(format!("Notas inseridas: {}", self.score.notes.len()));
            });

        self.render_teclado_window(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Página da Partitura");
            ui.label("Clique em qualquer pauta para inserir nota na altura/posição exata.");
            ui.separator();

            let score_view_mode = self.score_view_mode;
            let zoom_percent = self.zoom_percent;
            let score = self.score.clone();
            let orchestral_order = self.orchestral_order.clone();
            let selected_tool = self.selected_tool;
            let mut placements = Vec::new();

            egui::ScrollArea::both().show(ui, |ui| match score_view_mode {
                ScoreViewMode::SinglePage => {
                    let placement = notation::draw_orchestral_page(
                        ui,
                        &score,
                        &orchestral_order,
                        "Movement II (excerpt) - Page 1",
                        zoom_percent,
                        selected_tool,
                    );
                    placements.push(placement);
                }
                ScoreViewMode::FacingPages => {
                    ui.horizontal_top(|ui| {
                        let placement = notation::draw_orchestral_page(
                            ui,
                            &score,
                            &orchestral_order,
                            "Movement II (excerpt) - Page 1",
                            zoom_percent,
                            selected_tool,
                        );
                        placements.push(placement);

                        ui.add_space(24.0);

                        let placement = notation::draw_orchestral_page(
                            ui,
                            &score,
                            &orchestral_order,
                            "Movement II (excerpt) - Page 2",
                            zoom_percent,
                            selected_tool,
                        );
                        placements.push(placement);
                    });
                }
            });

            for place in placements.into_iter().flatten() {
                self.inserir_nota(
                    0,
                    place.insert_index,
                    place.pitch,
                    self.selected_duration,
                    place.instrument,
                );
                self.selected_tool = notation::KeyboardTool::None;
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(match self.score_view_mode {
                    ScoreViewMode::SinglePage => "Page 1",
                    ScoreViewMode::FacingPages => "Page 1-2",
                });
                ui.separator();
                ui.label(format!("Bars: {}", self.score.notes.len().max(1)));
                ui.separator();
                ui.label(format!("Engine: {:?}", self.playback_config.engine));
                ui.separator();
                ui.label(format!("Zoom: {:.1}%", self.zoom_percent));
            });
        });
    }

    fn inserir_nota(
        &mut self,
        _compasso: usize,
        insert_index: usize,
        pitch: Pitch,
        duracao: DurationValue,
        instrument: Instrument,
    ) {
        let mut index = 0usize;
        for (global_idx, note) in self.score.notes.iter().enumerate() {
            if note.instrument == instrument {
                if index >= insert_index {
                    self.score.notes.insert(
                        global_idx,
                        NoteEvent {
                            pitch,
                            accidental: self.selected_accidental,
                            duration: duracao,
                            instrument,
                        },
                    );
                    return;
                }
                index += 1;
            }
        }

        self.score.notes.push(NoteEvent {
            pitch,
            accidental: self.selected_accidental,
            duration: duracao,
            instrument,
        });
    }

    fn render_teclado_window(&mut self, ctx: &egui::Context) {
        let mut keyboard_open = self.keyboard_open;

        egui::Window::new("Teclado")
            .default_width(220.0)
            .open(&mut keyboard_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Ferramentas de inserção");
                ui.add_space(4.0);

                egui::Grid::new("teclado_grid")
                    .num_columns(4)
                    .spacing([6.0, 6.0])
                    .show(ui, |ui| {
                        self.tool_button(ui, "𝅝", DurationValue::Whole);
                        self.tool_button(ui, "𝅗𝅥", DurationValue::Half);
                        self.tool_button(ui, "♩", DurationValue::Quarter);
                        self.tool_button(ui, "♪", DurationValue::Eighth);
                    });

                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.label("Acidentes:");
                    for accidental in Accidental::ALL {
                        if ui
                            .selectable_label(
                                self.selected_accidental == accidental,
                                accidental.label(),
                            )
                            .clicked()
                        {
                            self.selected_accidental = accidental;
                        }
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    self.keyboard_tab_button(ui, KeyboardPage::One, "1");
                    self.keyboard_tab_button(ui, KeyboardPage::Two, "2");
                    self.keyboard_tab_button(ui, KeyboardPage::Three, "3");
                    self.keyboard_tab_button(ui, KeyboardPage::Four, "4");
                    self.keyboard_tab_button(ui, KeyboardPage::All, "All");
                });
            });

        self.keyboard_open = keyboard_open;
    }

    fn tool_button(&mut self, ui: &mut egui::Ui, icon: &str, duration: DurationValue) {
        let selected = self.selected_duration == duration
            && self.selected_tool == notation::KeyboardTool::Insert;
        if ui
            .add_sized([32.0, 32.0], egui::Button::new(icon).selected(selected))
            .clicked()
        {
            self.selected_duration = duration;
            self.selected_tool = notation::KeyboardTool::Insert;
        }
    }

    fn keyboard_tab_button(&mut self, ui: &mut egui::Ui, tab: KeyboardPage, label: &str) {
        if ui
            .selectable_label(self.keyboard_page == tab, label)
            .clicked()
        {
            self.keyboard_page = tab;
        }
    }

    fn render_notation_catalog(&mut self, ui: &mut egui::Ui) {
        ui.collapsing("📚 Catálogo de notação (fundação)", |ui| {
            ui.label("Pentagrama: pauta, linhas suplementares, sistemas e claves históricas.");
            ui.label("Ritmo: semibreve → semifusa + valores irregulares.");
            ui.label("Altura: acidentes, armadura, modos e escalas.");
            ui.label("Compasso: simples, compostos, alternados, C e C cortado.");
            ui.label("Dinâmica: ppp..fff, cresc/dim, sfz/rfz.");
            ui.label("Articulação: staccato, tenuto, marcato, fermata etc.");
            ui.label("Ornamentos: trinado, mordente, grupeto, appoggiatura, tremolo.");
            ui.label("Estrutura: ritornello, D.C., D.S., coda, segno, fine, voltas.");
            ui.label("Playback pro: humanização, velocity, CC MIDI, VST/NotePerformer.");
            ui.label("Engraving pro: layout automático, partes, MusicXML, PDF/MIDI/WAV.");
        });
    }
}

fn find_recent_ntr_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ntr") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn sanitized_ntr_path(input: &str) -> PathBuf {
    let trimmed = input.trim();
    let base = if trimmed.is_empty() {
        "notarium_score.ntr"
    } else {
        trimmed
    };

    if base.ends_with(".ntr") {
        PathBuf::from(base)
    } else {
        PathBuf::from(format!("{base}.ntr"))
    }
}

fn serialize_ntr(
    settings: &ScoreSettings,
    score: &Score,
    bpm: f32,
    key_signature: KeySignature,
    time_signature: TimeSignature,
    paper_size: PaperSize,
) -> String {
    let mut out = String::new();
    out.push_str("NTR2\n");
    out.push_str(&format!("title={}\n", settings.title.replace('\n', " ")));
    out.push_str(&format!(
        "composer={}\n",
        settings.composer.replace('\n', " ")
    ));
    out.push_str(&format!("bpm={}\n", bpm));
    out.push_str(&format!("key={:?}\n", key_signature));
    out.push_str(&format!("time={:?}\n", time_signature));
    out.push_str(&format!("paper={:?}\n", paper_size));
    out.push_str("notes:\n");
    for note in &score.notes {
        out.push_str(&format!(
            "{},{:?},{:?},{},{:?}\n",
            note.pitch.octave,
            note.pitch.class,
            note.accidental,
            note.duration.beats(),
            note.instrument
        ));
    }
    out
}

fn deserialize_ntr(contents: &str) -> Result<(ScoreSettings, Score, f32), String> {
    let mut lines = contents.lines();
    let Some(header) = lines.next() else {
        return Err("arquivo vazio".to_owned());
    };
    if header.trim() != "NTR1" && header.trim() != "NTR2" {
        return Err("formato .ntr inválido".to_owned());
    }

    let is_v2 = header.trim() == "NTR2";

    let mut title = "Nova Partitura".to_owned();
    let mut composer = "Compositor".to_owned();
    let mut bpm = 110.0_f32;
    let mut key = KeySignature::C;
    let mut time = TimeSignature::FourFour;
    let mut paper = PaperSize::A4;
    let mut notes = Vec::new();
    let mut in_notes = false;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if line == "notes:" {
            in_notes = true;
            continue;
        }

        if !in_notes {
            if let Some(rest) = line.strip_prefix("title=") {
                title = rest.to_owned();
            } else if let Some(rest) = line.strip_prefix("composer=") {
                composer = rest.to_owned();
            } else if let Some(rest) = line.strip_prefix("bpm=") {
                bpm = rest.parse::<f32>().unwrap_or(110.0);
            } else if let Some(rest) = line.strip_prefix("key=") {
                key = parse_key(rest).unwrap_or(KeySignature::C);
            } else if let Some(rest) = line.strip_prefix("time=") {
                time = parse_time(rest).unwrap_or(TimeSignature::FourFour);
            } else if let Some(rest) = line.strip_prefix("paper=") {
                paper = parse_paper(rest).unwrap_or(PaperSize::A4);
            }
        } else {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 4 {
                continue;
            }
            let octave = parts[0].parse::<i8>().unwrap_or(4);
            let class = parse_pitch(parts[1]).unwrap_or(PitchClass::C);
            let (accidental, beats_index, instrument_index) = if is_v2 && parts.len() >= 5 {
                (
                    parse_accidental(parts[2]).unwrap_or(Accidental::Natural),
                    3,
                    4,
                )
            } else {
                (Accidental::Natural, 2, 3)
            };
            let beats = parts[beats_index].parse::<f32>().unwrap_or(1.0);
            let duration = parse_duration_from_beats(beats);
            let instrument = parse_instrument(parts[instrument_index]).unwrap_or(Instrument::Piano);
            notes.push(NoteEvent {
                pitch: Pitch { class, octave },
                accidental,
                duration,
                instrument,
            });
        }
    }

    Ok((
        ScoreSettings {
            title,
            composer,
            key_signature: key,
            time_signature: time,
            paper_size: paper,
        },
        Score { notes },
        bpm,
    ))
}

fn parse_pitch(raw: &str) -> Option<PitchClass> {
    match raw {
        "C" => Some(PitchClass::C),
        "D" => Some(PitchClass::D),
        "E" => Some(PitchClass::E),
        "F" => Some(PitchClass::F),
        "G" => Some(PitchClass::G),
        "A" => Some(PitchClass::A),
        "B" => Some(PitchClass::B),
        _ => None,
    }
}

fn parse_accidental(raw: &str) -> Option<Accidental> {
    match raw {
        "Natural" => Some(Accidental::Natural),
        "Sharp" => Some(Accidental::Sharp),
        "Flat" => Some(Accidental::Flat),
        _ => None,
    }
}

fn parse_duration_from_beats(beats: f32) -> DurationValue {
    if (beats - 4.0).abs() < 0.1 {
        DurationValue::Whole
    } else if (beats - 2.0).abs() < 0.1 {
        DurationValue::Half
    } else if (beats - 1.0).abs() < 0.1 {
        DurationValue::Quarter
    } else if (beats - 0.5).abs() < 0.1 {
        DurationValue::Eighth
    } else if (beats - 0.25).abs() < 0.05 {
        DurationValue::Sixteenth
    } else if (beats - 0.125).abs() < 0.03 {
        DurationValue::ThirtySecond
    } else if (beats - 0.0625).abs() < 0.02 {
        DurationValue::SixtyFourth
    } else {
        DurationValue::Quarter
    }
}

fn parse_instrument(raw: &str) -> Option<Instrument> {
    match raw {
        "Violin" => Some(Instrument::Violin),
        "Viola" => Some(Instrument::Viola),
        "Cello" => Some(Instrument::Cello),
        "Flute" => Some(Instrument::Flute),
        "Clarinet" => Some(Instrument::Clarinet),
        "Trumpet" => Some(Instrument::Trumpet),
        "Horn" => Some(Instrument::Horn),
        "Timpani" => Some(Instrument::Timpani),
        "Piano" => Some(Instrument::Piano),
        _ => None,
    }
}

fn parse_key(raw: &str) -> Option<KeySignature> {
    match raw {
        "C" => Some(KeySignature::C),
        "G" => Some(KeySignature::G),
        "D" => Some(KeySignature::D),
        "A" => Some(KeySignature::A),
        "E" => Some(KeySignature::E),
        "F" => Some(KeySignature::F),
        "Bb" => Some(KeySignature::Bb),
        "Eb" => Some(KeySignature::Eb),
        "Ab" => Some(KeySignature::Ab),
        _ => None,
    }
}

fn parse_time(raw: &str) -> Option<TimeSignature> {
    match raw {
        "FourFour" => Some(TimeSignature::FourFour),
        "ThreeFour" => Some(TimeSignature::ThreeFour),
        "TwoFour" => Some(TimeSignature::TwoFour),
        "SixEight" => Some(TimeSignature::SixEight),
        _ => None,
    }
}

fn parse_paper(raw: &str) -> Option<PaperSize> {
    match raw {
        "A4" => Some(PaperSize::A4),
        "A3" => Some(PaperSize::A3),
        "Letter" => Some(PaperSize::Letter),
        _ => None,
    }
}
