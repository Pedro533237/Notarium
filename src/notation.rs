use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

use crate::music::{DurationValue, NoteEvent, PitchClass, Score};

pub fn draw_score(ui: &mut egui::Ui, score: &Score) {
    let desired_size = Vec2::new(ui.available_width().max(600.0), 280.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
    let painter = ui.painter_at(rect);

    draw_staff(&painter, rect);

    let start_x = rect.left() + 50.0;
    let note_spacing = 36.0;

    for (idx, note) in score.notes.iter().enumerate() {
        let x = start_x + idx as f32 * note_spacing;
        let y = note_y(rect, note);

        draw_notehead(&painter, Pos2::new(x, y), note.duration);

        if needs_stem(note.duration) {
            painter.line_segment(
                [Pos2::new(x + 7.0, y), Pos2::new(x + 7.0, y - 35.0)],
                Stroke::new(1.5, Color32::BLACK),
            );
        }

        if note.pitch.class == PitchClass::C && note.pitch.octave >= 5 {
            painter.line_segment(
                [Pos2::new(x - 12.0, y), Pos2::new(x + 12.0, y)],
                Stroke::new(1.2, Color32::BLACK),
            );
        }
    }
}

fn draw_staff(painter: &egui::Painter, rect: Rect) {
    let top = rect.center().y - 50.0;

    for i in 0..5 {
        let y = top + i as f32 * 20.0;
        painter.line_segment(
            [
                Pos2::new(rect.left() + 20.0, y),
                Pos2::new(rect.right() - 20.0, y),
            ],
            Stroke::new(1.0, Color32::BLACK),
        );
    }
}

fn draw_notehead(painter: &egui::Painter, center: Pos2, duration: DurationValue) {
    let radius = Vec2::new(8.0, 6.0);
    let fill = if duration == DurationValue::Whole || duration == DurationValue::Half {
        Color32::WHITE
    } else {
        Color32::BLACK
    };

    painter.circle_filled(center, radius.x, fill);
    painter.circle_stroke(center, radius.x, Stroke::new(1.3, Color32::BLACK));
}

fn note_y(rect: Rect, note: &NoteEvent) -> f32 {
    let staff_top = rect.center().y - 50.0;
    let midi = (note.pitch.octave as i32 + 1) * 12 + note.pitch.class.semitone_offset();
    let e4_midi = 64;
    let half_steps = midi - e4_midi;
    staff_top + 80.0 - half_steps as f32 * 5.0
}

fn needs_stem(duration: DurationValue) -> bool {
    matches!(
        duration,
        DurationValue::Half | DurationValue::Quarter | DurationValue::Eighth
    )
}
