mod audio;
mod music;
mod notation;

use eframe::egui;
use music::{DurationValue, Instrument, NoteEvent, Pitch, PitchClass, Score};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Notarium",
        options,
        Box::new(|_cc| Box::<NotariumApp>::default()),
    )
}

struct NotariumApp {
    score: Score,
    selected_pitch: PitchClass,
    selected_octave: i8,
    selected_duration: DurationValue,
    selected_instrument: Instrument,
    bpm: f32,
}

impl Default for NotariumApp {
    fn default() -> Self {
        Self {
            score: Score::default(),
            selected_pitch: PitchClass::C,
            selected_octave: 4,
            selected_duration: DurationValue::Quarter,
            selected_instrument: Instrument::Violin,
            bpm: 110.0,
        }
    }
}

impl eframe::App for NotariumApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Notarium — Editor de Partituras");
                ui.separator();
                ui.label(format!(
                    "Compassos (4/4): {:.1}",
                    self.score.total_beats() / 4.0
                ));
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
                        ui.selectable_value(
                            &mut self.selected_duration,
                            DurationValue::Whole,
                            DurationValue::Whole.label(),
                        );
                        ui.selectable_value(
                            &mut self.selected_duration,
                            DurationValue::Half,
                            DurationValue::Half.label(),
                        );
                        ui.selectable_value(
                            &mut self.selected_duration,
                            DurationValue::Quarter,
                            DurationValue::Quarter.label(),
                        );
                        ui.selectable_value(
                            &mut self.selected_duration,
                            DurationValue::Eighth,
                            DurationValue::Eighth.label(),
                        );
                    });

                ui.add(egui::Slider::new(&mut self.bpm, 40.0..=220.0).text("BPM"));

                if ui.button("Adicionar nota").clicked() {
                    self.score.notes.push(NoteEvent {
                        pitch: Pitch {
                            class: self.selected_pitch,
                            octave: self.selected_octave,
                        },
                        duration: self.selected_duration,
                        instrument: self.selected_instrument,
                    });
                }

                if ui.button("Playback").clicked() {
                    audio::play_score(self.score.clone(), self.bpm);
                }

                if ui.button("Limpar partitura").clicked() {
                    self.score.notes.clear();
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualização da Partitura");
            ui.label(
                "Protótipo inspirado em fluxo de trabalho de software de notação profissional.",
            );
            notation::draw_score(ui, &self.score);
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, note) in self.score.notes.iter().enumerate() {
                    ui.label(format!(
                        "{}: {} • {} • {}",
                        index + 1,
                        note.pitch.label(),
                        note.duration.label(),
                        note.instrument.label()
                    ));
                }
            });
        });
    }
}
