#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

#[cfg(not(target_os = "windows"))]
compile_error!("Notarium atualmente é suportado apenas no Windows.");

#[cfg(not(target_pointer_width = "64"))]
compile_error!("Notarium suporta apenas arquiteturas x64 (64-bit).");

mod audio;
mod music;
mod notation;

use eframe::egui;
use music::{
    DurationValue, Instrument, KeySignature, NoteEvent, PaperSize, Pitch, PitchClass, Score,
    ScoreSettings, TimeSignature,
};

fn main() -> eframe::Result<()> {
    // Preferência por menor consumo; em Windows antigo pode cair em caminho de software.
    std::env::set_var("WGPU_POWER_PREF", "low");

    let mut startup_errors = Vec::new();

    if let Err(err) = run_notarium_guarded(eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    }) {
        startup_errors.push(format!("Falha ao iniciar Notarium (WGPU padrão): {err}"));
    } else {
        return Ok(());
    }

    #[cfg(target_os = "windows")]
    {
        // Tenta caminho mais estável para máquinas antigas no Windows (DX11 + fallback adapter).
        std::env::set_var("WGPU_BACKEND", "dx11");
        std::env::set_var("WGPU_FORCE_FALLBACK_ADAPTER", "1");

        if let Err(err) = run_notarium_guarded(eframe::NativeOptions {
            renderer: eframe::Renderer::Wgpu,
            ..Default::default()
        }) {
            startup_errors.push(format!("Falha WGPU(DX11/WARP fallback): {err}"));
        } else {
            return Ok(());
        }

        std::env::remove_var("WGPU_FORCE_FALLBACK_ADAPTER");
        std::env::remove_var("WGPU_BACKEND");

        // Fallback para Glow com aceleração preferida (evita exigir contexto ES específico).
        if let Err(err) = run_notarium_guarded(eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,
            ..Default::default()
        }) {
            startup_errors.push(format!("Falha Glow(Preferred): {err}"));
        } else {
            return Ok(());
        }

        // Último fallback: Glow com aceleração requerida.
        if let Err(err) = run_notarium_guarded(eframe::NativeOptions {
            renderer: eframe::Renderer::Glow,
            hardware_acceleration: eframe::HardwareAcceleration::Required,
            ..Default::default()
        }) {
            startup_errors.push(format!("Falha Glow(Required): {err}"));
        } else {
            return Ok(());
        }
    }

    let error_message = startup_errors.join("\n") + "\n";
    let _ = std::fs::write("notarium.log", &error_message);
    show_startup_error(&error_message);
    Ok(())
}

fn run_notarium_guarded(options: eframe::NativeOptions) -> Result<(), String> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_notarium(options))) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(format!("{err:?}")),
        Err(panic_payload) => {
            let message = if let Some(msg) = panic_payload.downcast_ref::<&str>() {
                (*msg).to_string()
            } else if let Some(msg) = panic_payload.downcast_ref::<String>() {
                msg.clone()
            } else {
                "panic sem mensagem detalhada".to_owned()
            };
            Err(format!("panic capturado: {message}"))
        }
    }
}

fn run_notarium(options: eframe::NativeOptions) -> eframe::Result<()> {
    eframe::run_native(
        "Notarium",
        options,
        Box::new(|_cc| Box::<NotariumApp>::default()),
    )
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
                    audio::play_score(self.score.clone(), self.bpm);
                }

                ui.separator();
                ui.label(format!("Notas inseridas: {}", self.score.notes.len()));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualização da Partitura");
            ui.separator();
            notation::draw_score(ui, &self.score);
        });
    }
}
