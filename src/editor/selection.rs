use egui::{Color32, Pos2, Rect, Stroke};

use crate::music::{Note, NoteId};

#[derive(Debug, Clone)]
pub struct SelectionOverlay {
    pub note_id: NoteId,
    pub rect: Rect,
}

#[derive(Debug, Clone, Default)]
pub struct SelectionState {
    pub selected_note: Option<NoteId>,
    pub focus: bool,
}

impl SelectionState {
    pub fn set_selected(&mut self, note_id: Option<NoteId>, notes: &mut [Note]) {
        self.selected_note = note_id;

        for note in notes {
            let selected = Some(note.id) == note_id;
            note.selected = selected;
            note.opacity = if selected { 0.5 } else { 1.0 };
        }
    }

    pub fn hit_test(&self, cursor: Pos2, overlays: &[SelectionOverlay]) -> Option<NoteId> {
        overlays
            .iter()
            .find(|overlay| overlay.rect.expand(2.0).contains(cursor))
            .map(|overlay| overlay.note_id)
    }

    pub fn draw_overlay(painter: &egui::Painter, overlay: &SelectionOverlay) {
        painter.rect_stroke(
            overlay.rect.expand(2.0),
            2.0,
            Stroke::new(1.5, Color32::from_rgb(75, 140, 255)),
            egui::StrokeKind::Outside,
        );
        painter.rect_filled(
            overlay.rect.expand(1.0),
            2.0,
            Color32::from_rgba_unmultiplied(90, 140, 255, 24),
        );
    }
}
