#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Notarium atualmente é suportado apenas no Windows.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Notarium suporta apenas arquiteturas x64 (64-bit).");

mod audio;
mod editor;
mod input;
mod music;
mod render;

use editor::{EditMode, SelectionState};
use egui::{self, ViewportId};
use egui_glium::EguiGlium;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::winit;
use glium::Surface;
use input::keyboard::{apply_pitch_step, collect_actions, KeyboardAction};
use input::mouse::primary_click_position;
use music::{
    Clef, DurationValue, Instrument, KeySignature, Note, NoteEvent, NoteId, PaperSize, Pitch,
    PitchClass, Score, ScoreSettings, StaffSystem, TimeSignature,
};
use render::{GlProfile, Renderer};
use std::path::PathBuf;

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
    selected_clef: Clef,
    bpm: f32,
    screen: AppScreen,
    playback: audio::PlaybackController,
    is_paused: bool,
    orchestral_order: Vec<Instrument>,
    zoom_percent: f32,
    file_path_input: String,
    start_message: String,
    recent_scores: Vec<PathBuf>,
    renderer: Renderer,
    staff_system: StaffSystem,
    visual_notes: Vec<Note>,
    selection: SelectionState,
    edit_mode: EditMode,
    next_note_id: NoteId,
    snap_grid: f32,
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
            selected_pitch: PitchClass::C,
            selected_octave: 4,
            selected_duration: DurationValue::Quarter,
            selected_instrument: Instrument::Violin,
            selected_clef: Clef::Treble,
            bpm: 110.0,
            screen: AppScreen::Start,
            playback: audio::PlaybackController::new(),
            is_paused: false,
            orchestral_order: vec![
                Instrument::Flute,
                Instrument::Clarinet,
                Instrument::Trumpet,
                Instrument::Horn,
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
            renderer: Renderer::new(),
            staff_system: StaffSystem::with_standard_pair(
                settings.key_signature,
                settings.time_signature,
            ),
            visual_notes: Vec::new(),
            selection: SelectionState::default(),
            edit_mode: EditMode::None,
            next_note_id: 1,
            snap_grid: 0.25,
            settings,
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
        self.staff_system = StaffSystem::with_standard_pair(
            self.settings.key_signature,
            self.settings.time_signature,
        );
        self.score.notes.clear();
        self.visual_notes.clear();
        self.selection = SelectionState::default();
        self.edit_mode = EditMode::None;
        self.screen = AppScreen::Editor;
        self.start_message = "Nova partitura criada.".to_owned();
    }

    fn add_note(&mut self) {
        let mut evt = NoteEvent::new(
            Pitch {
                class: self.selected_pitch,
                octave: self.selected_octave,
            },
            self.selected_duration,
            self.selected_instrument,
        );
        evt.dotted = matches!(
            self.selected_duration,
            DurationValue::Eighth | DurationValue::Sixteenth
        );

        self.score.notes.push(evt.clone());
        let staff_pos = pitch_to_staff_position(evt.pitch);
        let note = Note::new(self.next_note_id, evt.pitch, evt.duration, staff_pos);
        self.next_note_id += 1;
        self.visual_notes.push(note);
        self.staff_system
            .ensure_layout_for_beats(self.score.total_beats());
    }

    fn apply_keyboard_editing(&mut self, ctx: &egui::Context) {
        let Some(selected) = self.selection.selected_note else {
            return;
        };

        for action in collect_actions(ctx) {
            match action {
                KeyboardAction::Delete => {
                    self.delete_note(selected);
                    break;
                }
                KeyboardAction::Duration(duration) => {
                    if let Some(idx) = self.find_note_index(selected) {
                        if let Some(note) = self.visual_notes.get_mut(idx) {
                            note.duration = duration;
                        }
                        if let Some(evt) = self.score.notes.get_mut(idx) {
                            evt.duration = duration;
                        }
                    }
                }
                KeyboardAction::PitchStep(step) => {
                    if let Some(idx) = self.find_note_index(selected) {
                        let mut updated_pitch = None;
                        if let Some(note) = self.visual_notes.get_mut(idx) {
                            apply_pitch_step(note, step);
                            note.staff_position = pitch_to_staff_position(note.pitch);
                            updated_pitch = Some(note.pitch);
                        }
                        if let (Some(evt), Some(pitch)) =
                            (self.score.notes.get_mut(idx), updated_pitch)
                        {
                            evt.pitch = pitch;
                        }
                    }
                }
            }
        }
    }

    fn find_note_index(&self, id: NoteId) -> Option<usize> {
        self.visual_notes.iter().position(|note| note.id == id)
    }

    fn delete_note(&mut self, id: NoteId) {
        if let Some(index) = self.visual_notes.iter().position(|n| n.id == id) {
            self.visual_notes.remove(index);
            if index < self.score.notes.len() {
                self.score.notes.remove(index);
            }
            self.selection.set_selected(None, &mut self.visual_notes);
            self.selection.focus = false;
            self.edit_mode = EditMode::None;
        }
    }

    fn save_ntr(&mut self) {
        let path = sanitized_ntr_path(&self.file_path_input);
        let payload = serialize_ntr(&self.settings, &self.score, self.bpm);

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
                    self.staff_system = StaffSystem::with_standard_pair(
                        self.settings.key_signature,
                        self.settings.time_signature,
                    );
                    self.rebuild_visual_notes();
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

    fn rebuild_visual_notes(&mut self) {
        self.visual_notes.clear();
        for evt in &self.score.notes {
            let note = Note::new(
                self.next_note_id,
                evt.pitch,
                evt.duration,
                pitch_to_staff_position(evt.pitch),
            );
            self.next_note_id += 1;
            self.visual_notes.push(note);
        }
        self.staff_system
            .ensure_layout_for_beats(self.score.total_beats());
    }

    fn render_start_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Notarium");
            ui.label("Hub moderno de partituras: crie, abra e gerencie arquivos .ntr");

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

                    if ui.button("Criar partitura").clicked() {
                        self.create_new_score_from_start();
                    }
                });

                columns[1].group(|ui| {
                    ui.heading("Abrir Partitura (.ntr)");
                    ui.separator();
                    ui.text_edit_singleline(&mut self.file_path_input);
                    if ui.button("Abrir arquivo").clicked() {
                        self.open_ntr_from_path(sanitized_ntr_path(&self.file_path_input));
                    }
                    ui.separator();
                    ui.label("Recentes:");
                    for path in self.recent_scores.clone() {
                        let label = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("arquivo.ntr");
                        if ui.button(label).clicked() {
                            self.open_ntr_from_path(path);
                        }
                    }
                });
            });

            ui.label(&self.start_message);
        });
    }

    fn render_editor(&mut self, ctx: &egui::Context) {
        self.apply_keyboard_editing(ctx);

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.heading(self.settings.title.as_str());
                ui.separator();
                ui.label(format!(
                    "Compasso: {}",
                    self.settings.time_signature.label()
                ));
                ui.separator();
                ui.label(format!(
                    "Tonalidade: {}",
                    self.settings.key_signature.label()
                ));
                ui.separator();
                ui.label(format!("OpenGL alvo: {}", self.renderer.gl_profile.label()));
                ui.separator();
                if ui.button("💾 Salvar .ntr").clicked() {
                    self.save_ntr();
                }
                if ui.button("← Voltar").clicked() {
                    self.screen = AppScreen::Start;
                }
            });
        });

        egui::SidePanel::left("controls")
            .resizable(false)
            .min_width(280.0)
            .show(ctx, |ui| {
                ui.heading("Entrada de Notas");
                ui.separator();

                egui::ComboBox::from_label("Perfil GL")
                    .selected_text(self.renderer.gl_profile.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.renderer.gl_profile, GlProfile::OpenGl21, GlProfile::OpenGl21.label());
                        ui.selectable_value(&mut self.renderer.gl_profile, GlProfile::OpenGl20, GlProfile::OpenGl20.label());
                        ui.selectable_value(&mut self.renderer.gl_profile, GlProfile::OpenGlEs20, GlProfile::OpenGlEs20.label());
                    });

                egui::ComboBox::from_label("Instrumento")
                    .selected_text(self.selected_instrument.label())
                    .show_ui(ui, |ui| {
                        for instrument in Instrument::ALL {
                            ui.selectable_value(&mut self.selected_instrument, instrument, instrument.label());
                        }
                    });

                egui::ComboBox::from_label("Clave")
                    .selected_text(self.selected_clef.label())
                    .show_ui(ui, |ui| {
                        for clef in Clef::ALL {
                            ui.selectable_value(&mut self.selected_clef, clef, clef.label());
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
                            ui.selectable_value(&mut self.selected_duration, duration, duration.label());
                        }
                    });

                ui.add(egui::Slider::new(&mut self.bpm, 40.0..=220.0).text("BPM"));
                ui.add(egui::Slider::new(&mut self.zoom_percent, 40.0..=200.0).text("Zoom"));
                ui.add(egui::Slider::new(&mut self.snap_grid, 0.125..=1.0).text("Snap rítmico"));

                if ui.button("Adicionar Nota").clicked() {
                    self.add_note();
                }

                if ui.button("Limpar Partitura").clicked() {
                    self.score.notes.clear();
                    self.visual_notes.clear();
                    self.selection = SelectionState::default();
                    self.selection.focus = false;
                    self.edit_mode = EditMode::None;
                }

                ui.horizontal_wrapped(|ui| {
                    if ui.button("▶ Play").clicked() {
                        self.playback.play(self.score.clone(), self.bpm);
                        self.is_paused = false;
                    }
                    if ui.button("⏸ Pausar").clicked() {
                        self.playback.pause();
                        self.is_paused = true;
                    }
                    if ui.button("⏵ Retomar").clicked() {
                        self.playback.resume();
                        self.is_paused = false;
                    }
                    if ui.button("⏹ Parar").clicked() {
                        self.playback.stop();
                        self.is_paused = false;
                    }
                    if ui.button("⏮ Reiniciar").clicked() {
                        self.playback.rewind();
                        self.is_paused = false;
                    }
                });

                ui.separator();
                ui.label("Edição de nota: clique para selecionar, 1..7 muda duração, ↑/↓ move pitch, Delete remove.");
                ui.label(format!("Notas inseridas: {}", self.score.notes.len()));
                ui.label(format!("Modo: {:?}", self.edit_mode));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualização Orquestral");
            ui.separator();

            egui::ScrollArea::both().show(ui, |ui| {
                let (response, overlays) = self.renderer.draw_orchestral_page(
                    ui,
                    &self.score,
                    &self.visual_notes,
                    &self.staff_system,
                    &self.orchestral_order,
                    "Movement II (excerpt) - Page 1",
                    self.zoom_percent,
                    &self.selection,
                );

                if let Some(cursor) = primary_click_position(&response) {
                    let hit = self.selection.hit_test(cursor, &overlays);
                    self.selection.set_selected(hit, &mut self.visual_notes);
                    self.selection.focus = hit.is_some();
                    self.edit_mode = hit.map_or(EditMode::None, EditMode::NoteSelected);
                }
            });
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(format!("Compassos: {:.1} • unidade:{:?}", self.score.total_measures(self.settings.time_signature), self.settings.time_signature.beat_unit()));
                ui.separator();
                ui.label(format!("Seleção: {:?} • foco:{}", self.selection.selected_note, self.selection.focus));
                ui.separator();
                ui.label(format!("Zoom: {:.1}%", self.zoom_percent));
                ui.separator();
                ui.label("Render pipeline: batching lógico + cache de glyphs + camadas Staff/Notes/SelectionOverlay");
            });
        });
    }
}

fn pitch_to_staff_position(pitch: Pitch) -> f32 {
    let midi = pitch.midi_number();
    let e4_midi = 64;
    (midi - e4_midi) as f32 * 0.5
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

fn serialize_ntr(settings: &ScoreSettings, score: &Score, bpm: f32) -> String {
    let mut out = String::new();
    out.push_str("NTR2\n");
    out.push_str(&format!("title={}\n", settings.title.replace('\n', " ")));
    out.push_str(&format!(
        "composer={}\n",
        settings.composer.replace('\n', " ")
    ));
    out.push_str(&format!("bpm={}\n", bpm));
    out.push_str(&format!("key={:?}\n", settings.key_signature));
    out.push_str(&format!("time={:?}\n", settings.time_signature));
    out.push_str(&format!("paper={:?}\n", settings.paper_size));
    out.push_str("notes:\n");
    for note in &score.notes {
        out.push_str(&format!(
            "{},{:?},{},{:?},{},{},{},{}\n",
            note.pitch.octave,
            note.pitch.class,
            note.duration.beats(),
            note.instrument,
            note.dotted,
            note.tie_start,
            note.tie_end,
            note.velocity
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
            let beats = parts[2].parse::<f32>().unwrap_or(1.0);
            let duration = DurationValue::from_beats(beats);
            let instrument = parse_instrument(parts[3]).unwrap_or(Instrument::Piano);
            let mut evt = NoteEvent::new(Pitch { class, octave }, duration, instrument);
            if parts.len() >= 8 {
                evt.dotted = parts[4].parse::<bool>().unwrap_or(false);
                evt.tie_start = parts[5].parse::<bool>().unwrap_or(false);
                evt.tie_end = parts[6].parse::<bool>().unwrap_or(false);
                evt.velocity = parts[7].parse::<u8>().unwrap_or(100);
            }
            notes.push(evt);
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

fn parse_key(input: &str) -> Option<KeySignature> {
    Some(match input.trim() {
        "C" => KeySignature::C,
        "G" => KeySignature::G,
        "D" => KeySignature::D,
        "A" => KeySignature::A,
        "E" => KeySignature::E,
        "F" => KeySignature::F,
        "Bb" => KeySignature::Bb,
        "Eb" => KeySignature::Eb,
        "Ab" => KeySignature::Ab,
        _ => return None,
    })
}

fn parse_time(input: &str) -> Option<TimeSignature> {
    Some(match input.trim() {
        "FourFour" => TimeSignature::FourFour,
        "ThreeFour" => TimeSignature::ThreeFour,
        "TwoFour" => TimeSignature::TwoFour,
        "SixEight" => TimeSignature::SixEight,
        _ => return None,
    })
}

fn parse_paper(input: &str) -> Option<PaperSize> {
    Some(match input.trim() {
        "A4" => PaperSize::A4,
        "A3" => PaperSize::A3,
        "Letter" => PaperSize::Letter,
        _ => return None,
    })
}

fn parse_pitch(input: &str) -> Option<PitchClass> {
    Some(match input.trim() {
        "C" => PitchClass::C,
        "D" => PitchClass::D,
        "E" => PitchClass::E,
        "F" => PitchClass::F,
        "G" => PitchClass::G,
        "A" => PitchClass::A,
        "B" => PitchClass::B,
        _ => return None,
    })
}

fn parse_instrument(input: &str) -> Option<Instrument> {
    Some(match input.trim() {
        "Violin" => Instrument::Violin,
        "Viola" => Instrument::Viola,
        "Cello" => Instrument::Cello,
        "Flute" => Instrument::Flute,
        "Clarinet" => Instrument::Clarinet,
        "Trumpet" => Instrument::Trumpet,
        "Horn" => Instrument::Horn,
        "Timpani" => Instrument::Timpani,
        "Piano" => Instrument::Piano,
        _ => return None,
    })
}
