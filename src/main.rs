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
use std::path::PathBuf;

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
    file_path_input: String,
    start_message: String,
    recent_scores: Vec<PathBuf>,
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
            file_path_input: "notarium_score.ntr".to_owned(),
            start_message: "Pronto para criar ou abrir partitura.".to_owned(),
            recent_scores: find_recent_ntr_files(),
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().panel_fill = egui::Color32::from_rgb(23, 25, 30);

            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.heading(
                    egui::RichText::new("Notarium")
                        .size(34.0)
                        .color(egui::Color32::WHITE),
                );
                ui.label(
                    egui::RichText::new(
                        "Hub moderno de partituras: crie, abra e gerencie arquivos .ntr",
                    )
                    .color(egui::Color32::from_rgb(190, 196, 210)),
                );
                ui.add_space(10.0);
            });

            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.heading("Nova Partitura");
                    ui.separator();
                    ui.label("Nome da partitura");
                    ui.text_edit_singleline(&mut self.start_title);
                    ui.label("Nome do compositor");
                    ui.text_edit_singleline(&mut self.start_composer);

                    egui::ComboBox::from_label("Tonalidade")
                        .selected_text(self.start_key_signature.label())
                        .show_ui(ui, |ui| {
                            for key in KeySignature::ALL {
                                ui.selectable_value(
                                    &mut self.start_key_signature,
                                    key,
                                    key.label(),
                                );
                            }
                        });

                    egui::ComboBox::from_label("Fórmula de compasso")
                        .selected_text(self.start_time_signature.label())
                        .show_ui(ui, |ui| {
                            for time in TimeSignature::ALL {
                                ui.selectable_value(
                                    &mut self.start_time_signature,
                                    time,
                                    time.label(),
                                );
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

                    if ui.button("✨ Criar e Abrir Editor").clicked() {
                        self.create_new_score_from_start();
                    }
                });

                columns[1].group(|ui| {
                    ui.heading("Partituras .ntr");
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
                    });

                    if ui.button("🔄 Atualizar lista").clicked() {
                        self.recent_scores = find_recent_ntr_files();
                    }

                    ui.separator();
                    ui.label("Recentes");
                    egui::ScrollArea::vertical()
                        .max_height(320.0)
                        .show(ui, |ui| {
                            let recent = self.recent_scores.clone();
                            for path in recent {
                                let label = path
                                    .file_name()
                                    .and_then(|f| f.to_str())
                                    .unwrap_or("arquivo.ntr");
                                if ui.button(label).clicked() {
                                    self.open_ntr_from_path(path);
                                }
                            }
                        });
                });
            });

            ui.add_space(8.0);
            ui.label(egui::RichText::new(&self.start_message).color(egui::Color32::LIGHT_GREEN));
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
                if ui.button("💾 Salvar .ntr").clicked() {
                    self.save_ntr();
                }
                if ui.button("📂 Abrir .ntr").clicked() {
                    self.open_ntr_from_input();
                }
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
    out.push_str("NTR1\n");
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
            "{},{:?},{},{:?}\n",
            note.pitch.octave,
            note.pitch.class,
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
    if header.trim() != "NTR1" {
        return Err("formato .ntr inválido".to_owned());
    }

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
            if parts.len() != 4 {
                continue;
            }
            let octave = parts[0].parse::<i8>().unwrap_or(4);
            let class = parse_pitch(parts[1]).unwrap_or(PitchClass::C);
            let beats = parts[2].parse::<f32>().unwrap_or(1.0);
            let duration = parse_duration_from_beats(beats);
            let instrument = parse_instrument(parts[3]).unwrap_or(Instrument::Piano);
            notes.push(NoteEvent {
                pitch: Pitch { class, octave },
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

fn parse_duration_from_beats(beats: f32) -> DurationValue {
    if (beats - 4.0).abs() < 0.1 {
        DurationValue::Whole
    } else if (beats - 2.0).abs() < 0.1 {
        DurationValue::Half
    } else if (beats - 0.5).abs() < 0.1 {
        DurationValue::Eighth
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
