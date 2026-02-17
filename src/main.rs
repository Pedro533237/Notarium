#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod audio;
mod music;
mod notation;

use eframe::egui;
use music::{
    DurationValue, Instrument, KeySignature, NoteEvent, PaperSize, Pitch, PitchClass, Score,
    ScoreSettings, TimeSignature,
};

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.renderer = eframe::Renderer::Glow;

    eframe::run_native(
        "Notarium",
        options,
        Box::new(|_cc| Box::<NotariumApp>::default()),
    )
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
    bpm: f32,
    screen: AppScreen,
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
        }
    }
}

impl eframe::App for NotariumApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        match self.screen {
            AppScreen::Start => self.render_start_screen(ctx),
            AppScreen::Editor => self.render_editor(ctx),
        }
    }
}

impl NotariumApp {
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
            ui.horizontal(|ui| {
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
            });

            if ui.button("← Voltar para Início").clicked() {
                self.screen = AppScreen::Start;
            }
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
            ui.label("Editor aberto após configurar nova partitura na tela inicial.");
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
