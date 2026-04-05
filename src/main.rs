use eframe::egui;
use notarium_core::{
    Accidental, Instrument, KeySignature, Note, NoteDuration, Pitch, PitchClass, Score,
};
use notarium_io::{export_musicxml, load_notarium, save_notarium};
use notarium_playback::PlaybackEngine;
use notarium_render::{draw_score, hit_test, Tool};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Notarium",
        options,
        Box::new(|_cc| Ok(Box::new(NotariumApp::default()))),
    )
}

struct NotariumApp {
    score: Score,
    zoom: f32,
    tool: Tool,
    selected_duration: NoteDuration,
    selected_pitch: PitchClass,
    selected_octave: i8,
    selected_accidental: Accidental,
    selected_instrument: Instrument,
    file_path: String,
    status: String,
    playback: Option<PlaybackEngine>,
}

impl Default for NotariumApp {
    fn default() -> Self {
        Self {
            score: Score::default(),
            zoom: 1.0,
            tool: Tool::Pen,
            selected_duration: NoteDuration::Quarter,
            selected_pitch: PitchClass::C,
            selected_octave: 4,
            selected_accidental: Accidental::Natural,
            selected_instrument: Instrument::Piano,
            file_path: "partitura.notarium".to_owned(),
            status: "Pronto".to_owned(),
            playback: PlaybackEngine::new().ok(),
        }
    }
}

impl eframe::App for NotariumApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard(ctx);

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.score = Score::default();
                        self.status = "Nova partitura criada".to_owned();
                        ui.close();
                    }
                    if ui.button("Open").clicked() {
                        match load_notarium(&self.file_path) {
                            Ok(s) => {
                                self.score = s;
                                self.status = "Arquivo aberto".to_owned();
                            }
                            Err(err) => self.status = format!("Erro ao abrir: {err}"),
                        }
                        ui.close();
                    }
                    if ui.button("Save").clicked() {
                        match save_notarium(&self.file_path, &self.score) {
                            Ok(_) => self.status = "Arquivo salvo".to_owned(),
                            Err(err) => self.status = format!("Erro ao salvar: {err}"),
                        }
                        ui.close();
                    }
                    if ui.button("Export MusicXML").clicked() {
                        let target = self.file_path.replace(".notarium", ".musicxml");
                        match export_musicxml(&target, &self.score) {
                            Ok(_) => self.status = format!("Exportado: {target}"),
                            Err(err) => self.status = format!("Erro no export: {err}"),
                        }
                        ui.close();
                    }
                });
                ui.separator();
                ui.label("Arquivo:");
                ui.text_edit_singleline(&mut self.file_path);
            });
        });

        egui::SidePanel::left("instruments").show(ctx, |ui| {
            ui.heading("Instrumentos");
            for inst in Instrument::ALL {
                if ui
                    .selectable_label(self.selected_instrument == inst, inst.label())
                    .clicked()
                {
                    self.selected_instrument = inst;
                    if let Some(first) = self.score.staves.first_mut() {
                        first.instrument = inst;
                        first.name = inst.label().to_owned();
                    }
                }
            }

            ui.separator();
            ui.label("Ferramentas");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tool, Tool::Select, "Seleção");
                ui.selectable_value(&mut self.tool, Tool::Eraser, "Borracha");
                ui.selectable_value(&mut self.tool, Tool::Pen, "Caneta");
            });

            ui.separator();
            egui::ComboBox::from_label("Duração")
                .selected_text(self.selected_duration.label())
                .show_ui(ui, |ui| {
                    for dur in NoteDuration::ALL {
                        ui.selectable_value(&mut self.selected_duration, dur, dur.label());
                    }
                });

            egui::ComboBox::from_label("Armadura")
                .selected_text(self.score.key_signature.label())
                .show_ui(ui, |ui| {
                    for key in KeySignature::ALL {
                        ui.selectable_value(&mut self.score.key_signature, key, key.label());
                    }
                });

            ui.add(egui::Slider::new(&mut self.zoom, 0.5..=2.0).text("Zoom"));
        });

        egui::TopBottomPanel::bottom("transport").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Play").clicked() {
                    if let Some(playback) = &self.playback {
                        playback.play(&self.score);
                    }
                }
                if ui.button("Stop").clicked() {
                    if let Some(playback) = &self.playback {
                        playback.stop();
                    }
                }
                ui.add(egui::Slider::new(&mut self.score.bpm, 40..=220).text("BPM"));
                ui.separator();
                ui.label("Compasso:");
                ui.add(
                    egui::DragValue::new(&mut self.score.time_signature.beats_per_measure)
                        .range(1..=12),
                );
                ui.label("/");
                ui.add(
                    egui::DragValue::new(&mut self.score.time_signature.beat_unit).range(1..=16),
                );
                ui.separator();
                ui.label(&self.status);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                let (resp, measure_rects) = draw_score(ui, &self.score, self.zoom);
                if resp.clicked() {
                    if let Some(pos) = resp.interact_pointer_pos() {
                        if let Some(hit) = hit_test(
                            &measure_rects,
                            &self.score,
                            pos,
                            self.score.time_signature.beat_unit,
                        ) {
                            match self.tool {
                                Tool::Pen => {
                                    self.score.insert_note(
                                        hit.staff_idx,
                                        hit.measure_idx,
                                        Note {
                                            pitch: Pitch {
                                                class: self.selected_pitch,
                                                accidental: self.selected_accidental,
                                                octave: self.selected_octave,
                                            },
                                            duration: self.selected_duration,
                                            beat_offset: hit.beat,
                                            voice: 0,
                                        },
                                    );
                                }
                                Tool::Eraser => {
                                    if let Some(n_idx) = hit.note_idx {
                                        self.score.delete_note(
                                            hit.staff_idx,
                                            hit.measure_idx,
                                            n_idx,
                                        );
                                    }
                                }
                                Tool::Select => {
                                    self.selected_pitch = hit.pitch_class;
                                    self.selected_octave = hit.octave;
                                }
                            }
                        }
                    }
                }
            });
        });
    }
}

impl NotariumApp {
    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            let mut set_note = |pc| self.selected_pitch = pc;
            if i.key_pressed(egui::Key::A) {
                set_note(PitchClass::A);
            }
            if i.key_pressed(egui::Key::B) {
                set_note(PitchClass::B);
            }
            if i.key_pressed(egui::Key::C) {
                set_note(PitchClass::C);
            }
            if i.key_pressed(egui::Key::D) {
                set_note(PitchClass::D);
            }
            if i.key_pressed(egui::Key::E) {
                set_note(PitchClass::E);
            }
            if i.key_pressed(egui::Key::F) {
                set_note(PitchClass::F);
            }
            if i.key_pressed(egui::Key::G) {
                set_note(PitchClass::G);
            }

            if i.key_pressed(egui::Key::ArrowUp) {
                self.selected_octave = (self.selected_octave + 1).clamp(0, 8);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.selected_octave = (self.selected_octave - 1).clamp(0, 8);
            }

            if i.modifiers.shift {
                self.selected_accidental = Accidental::Sharp;
            } else if i.modifiers.alt {
                self.selected_accidental = Accidental::Flat;
            } else {
                self.selected_accidental = Accidental::Natural;
            }
        });
    }
}
