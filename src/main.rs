#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Notarium atualmente é suportado apenas no Windows.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Notarium suporta apenas arquiteturas x64 (64-bit).");

mod audio;
mod music;
mod notation;

use egui::{self, ViewportId};
use egui_glium::EguiGlium;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::winit;
use glium::Surface;
use music::{
    DurationValue, Instrument, KeySignature, NoteEvent, PaperSize, Pitch, PitchClass, Score,
    ScoreSettings, TimeSignature,
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

    let (window, display) = SimpleWindowBuilder::new()
        .set_window_builder(window_attributes)
        .build(&event_loop);

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
    selected_pitch: PitchClass,
    selected_octave: i8,
    selected_duration: DurationValue,
    selected_instrument: Instrument,
    bpm: f32,
    screen: AppScreen,
    active_tab: UiTab,
    playback: audio::PlaybackController,
    is_paused: bool,
    orchestral_order: Vec<Instrument>,
    zoom_percent: f32,
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
            selected_pitch: PitchClass::C,
            selected_octave: 4,
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

    fn render_start_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(24.0);
                ui.heading("Notarium");
                ui.label("Crie uma nova partitura antes de abrir o editor.");
                ui.add_space(18.0);
            });

            ui.group(|ui| {
                ui.set_width(520.0);
                ui.heading("Início / Configurações da partitura");
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
                    .add_sized(
                        [200.0, 36.0],
                        egui::Button::new("Nova Partitura (abrir editor)"),
                    )
                    .clicked()
                {
                    self.settings = ScoreSettings {
                        title: self.start_title.trim().to_owned(),
                        composer: self.start_composer.trim().to_owned(),
                        key_signature: self.start_key_signature,
                        time_signature: self.start_time_signature,
                        paper_size: self.start_paper_size,
                    };
                    self.score.notes.clear();
                    self.screen = AppScreen::Editor;
                }
            });
        });
    }

    fn render_editor(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (tab, label) in [
                    (UiTab::File, "File"),
                    (UiTab::Home, "Home"),
                    (UiTab::NoteInput, "Note Input"),
                    (UiTab::Notations, "Notations"),
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
                ui.group(|ui| {
                    ui.label("Clipboard");
                    ui.horizontal(|ui| {
                        let _ = ui.button("Paste");
                        let _ = ui.button("Copy");
                        let _ = ui.button("Cut");
                    });
                });

                ui.group(|ui| {
                    ui.label("Instruments");
                    ui.horizontal(|ui| {
                        let _ = ui.button("Add");
                        let _ = ui.button("Remove");
                        let _ = ui.button("Transpose");
                    });
                });

                ui.group(|ui| {
                    ui.label("Bars / View");
                    ui.horizontal(|ui| {
                        let _ = ui.button("Split");
                        let _ = ui.button("Join");
                        ui.add(
                            egui::Slider::new(&mut self.zoom_percent, 40.0..=140.0).text("Zoom"),
                        );
                    });
                });

                ui.group(|ui| {
                    ui.label("Playback");
                    ui.horizontal_wrapped(|ui| {
                        if ui.button("⏮ Retroceder").clicked() {
                            self.playback.rewind();
                            self.is_paused = false;
                        }

                        if ui.button("▶ Play").clicked() {
                            self.playback.play(self.score.clone(), self.bpm);
                            self.is_paused = false;
                        }

                        let pause_label = if self.is_paused {
                            "⏵ Retomar"
                        } else {
                            "⏸ Pausar"
                        };
                        if ui.button(pause_label).clicked() {
                            if self.is_paused {
                                self.playback.resume();
                            } else {
                                self.playback.pause();
                            }
                            self.is_paused = !self.is_paused;
                        }

                        if ui.button("⏹ Parar").clicked() {
                            self.playback.stop();
                            self.is_paused = false;
                        }
                    });
                });
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
                if ui.button("← Voltar para Início").clicked() {
                    self.screen = AppScreen::Start;
                }
            });
        });

        egui::SidePanel::left("controls")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                ui.heading("Entrada de Notas");
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

                egui::ComboBox::from_label("Altura")
                    .selected_text(self.selected_pitch.label())
                    .show_ui(ui, |ui| {
                        for pitch in PitchClass::ALL {
                            ui.selectable_value(&mut self.selected_pitch, pitch, pitch.label());
                        }
                    });

                ui.add(egui::Slider::new(&mut self.selected_octave, 1..=7).text("Oitava"));

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

                ui.add(egui::Slider::new(&mut self.bpm, 40.0..=220.0).text("BPM"));

                if ui.button("Adicionar Nota").clicked() {
                    self.score.notes.push(NoteEvent {
                        pitch: Pitch {
                            class: self.selected_pitch,
                            octave: self.selected_octave,
                        },
                        duration: self.selected_duration,
                        instrument: self.selected_instrument,
                    });
                }

                if ui.button("Limpar Partitura").clicked() {
                    self.score.notes.clear();
                }

                if ui.button("Play (síntese)").clicked() {
                    self.playback.play(self.score.clone(), self.bpm);
                    self.is_paused = false;
                }

                ui.separator();
                ui.label(format!("Notas inseridas: {}", self.score.notes.len()));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualização Orquestral");
            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    notation::draw_orchestral_page(
                        ui,
                        &self.score,
                        &self.orchestral_order,
                        "Movement II (excerpt) - Page 1",
                        self.zoom_percent,
                    );

                    ui.add_space(24.0);

                    notation::draw_orchestral_page(
                        ui,
                        &self.score,
                        &self.orchestral_order,
                        "Movement II (excerpt) - Page 2",
                        self.zoom_percent,
                    );
                });
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Page 1 of 2");
                ui.separator();
                ui.label(format!("Bars: {}", self.score.notes.len().max(1)));
                ui.separator();
                ui.label("No Selection");
                ui.separator();
                ui.label("Transposing Score");
                ui.separator();
                ui.label(format!("Zoom: {:.1}%", self.zoom_percent));
            });
        });
    }
}
