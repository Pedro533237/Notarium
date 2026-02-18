use egui::{Align2, Color32, FontId, Pos2, Sense, Stroke, Vec2};

use crate::editor::{SelectionOverlay, SelectionState};
use crate::music::{Instrument, Note, Score, StaffSystem};

use super::{glyph_cache::GlyphCache, note_renderer, staff_renderer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlProfile {
    OpenGl20,
    OpenGl21,
    OpenGlEs20,
    Default,
}

impl GlProfile {
    pub fn label(self) -> &'static str {
        match self {
            Self::OpenGl20 => "OpenGL 2.0",
            Self::OpenGl21 => "OpenGL 2.1",
            Self::OpenGlEs20 => "OpenGL ES 2.0",
            Self::Default => "OpenGL padrão do driver",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderLayer {
    Staff,
    Notes,
    SelectionOverlay,
}

pub struct Renderer {
    glyph_cache: GlyphCache,
    pub gl_profile: GlProfile,
}

impl Renderer {
    pub fn new(gl_profile: GlProfile) -> Self {
        let mut glyph_cache = GlyphCache::default();
        glyph_cache.warmup_music_symbols();

        Self {
            glyph_cache,
            gl_profile,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_orchestral_page(
        &mut self,
        ui: &mut egui::Ui,
        score: &Score,
        notes: &[Note],
        staff_system: &StaffSystem,
        instruments: &[Instrument],
        page_label: &str,
        zoom_percent: f32,
        selection: &SelectionState,
    ) -> (egui::Response, Vec<SelectionOverlay>) {
        let zoom = (zoom_percent / 100.0).clamp(0.45, 2.0);
        let desired_size = Vec2::new((860.0 * zoom).max(ui.available_width()), 1180.0 * zoom);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        let painter = ui.painter_at(rect);

        painter.rect_filled(rect, 6.0, Color32::from_rgb(250, 248, 242));
        painter.rect_stroke(
            rect,
            6.0,
            Stroke::new(1.0, Color32::from_gray(180)),
            egui::StrokeKind::Outside,
        );

        painter.text(
            Pos2::new(rect.center().x, rect.top() + 36.0 * zoom),
            Align2::CENTER_CENTER,
            "Notarium - Transposing Score",
            FontId::proportional(18.0 * zoom),
            Color32::BLACK,
        );
        painter.text(
            Pos2::new(rect.center().x, rect.top() + 64.0 * zoom),
            Align2::CENTER_CENTER,
            format!(
                "{page_label} • {} • glyphs:{} • atlas:{}",
                self.gl_profile.label(),
                self.glyph_cache.entry_count(),
                self.glyph_cache.atlas_checksum()
            ),
            FontId::proportional(13.0 * zoom),
            Color32::DARK_GRAY,
        );

        let mut overlays = Vec::new();
        let mut staff_rects = Vec::new();

        for layer in [
            RenderLayer::Staff,
            RenderLayer::Notes,
            RenderLayer::SelectionOverlay,
        ] {
            match layer {
                RenderLayer::Staff => {
                    staff_rects =
                        staff_renderer::draw_staff_system(&painter, rect, staff_system, zoom);
                }
                RenderLayer::Notes => {
                    for (staff_index, staff_rect) in staff_rects.iter().copied().enumerate() {
                        if let Some(staff) = staff_system.staffs.get(staff_index) {
                            let staff_notes =
                                filtered_notes(notes, score, staff_index, instruments);
                            let mut rendered = note_renderer::draw_notes(
                                &painter,
                                staff_rect,
                                staff,
                                &staff_notes,
                            );
                            overlays.append(&mut rendered);
                        }
                    }
                }
                RenderLayer::SelectionOverlay => {
                    for overlay in &overlays {
                        if Some(overlay.note_id) == selection.selected_note {
                            SelectionState::draw_overlay(&painter, overlay);
                        }
                    }
                }
            }
        }

        (response, overlays)
    }
}

fn filtered_notes(
    notes: &[Note],
    score: &Score,
    staff_index: usize,
    instruments: &[Instrument],
) -> Vec<Note> {
    let instrument = instruments.get(staff_index % instruments.len().max(1));

    score
        .notes
        .iter()
        .zip(notes.iter())
        .filter(|(evt, _)| instrument.is_none_or(|ins| *ins == evt.instrument))
        .map(|(_, note)| note.clone())
        .collect()
}
