use egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

use crate::music::{DurationValue, Instrument, NoteEvent, PitchClass, Score};

pub fn draw_orchestral_page(
    ui: &mut egui::Ui,
    score: &Score,
    instruments: &[Instrument],
    page_label: &str,
    zoom_percent: f32,
) {
    let zoom = (zoom_percent / 100.0).clamp(0.5, 2.0);
    let desired_size = Vec2::new((860.0 * zoom).max(ui.available_width()), 1180.0 * zoom);
    let (rect, _) = ui.allocate_exact_size(desired_size, egui::Sense::hover());
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
        page_label,
        FontId::proportional(14.0 * zoom),
        Color32::DARK_GRAY,
    );

    let mut y = rect.top() + 120.0 * zoom;
    let left_name_x = rect.left() + 18.0 * zoom;
    let staff_left = rect.left() + 95.0 * zoom;
    let staff_right = rect.right() - 24.0 * zoom;

    for (idx, instrument) in instruments.iter().enumerate() {
        let staff_rect = Rect::from_min_max(
            Pos2::new(staff_left, y),
            Pos2::new(staff_right, y + 54.0 * zoom),
        );

        draw_staff(&painter, staff_rect);

        painter.text(
            Pos2::new(left_name_x, y + 24.0 * zoom),
            Align2::LEFT_CENTER,
            instrument_short_name(*instrument),
            FontId::proportional(14.0 * zoom),
            Color32::BLACK,
        );

        draw_measure_lines(&painter, staff_rect, 6);
        draw_notes_for_staff(&painter, staff_rect, score, idx, *instrument);

        y += 78.0 * zoom;
        if y > rect.bottom() - 90.0 * zoom {
            break;
        }
    }
}

fn draw_staff(painter: &egui::Painter, rect: Rect) {
    let spacing = rect.height() / 4.0;
    for i in 0..5 {
        let y = rect.top() + i as f32 * spacing;
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(1.0, Color32::from_gray(55)),
        );
    }
}

fn draw_measure_lines(painter: &egui::Painter, rect: Rect, count: usize) {
    let width = rect.width() / count as f32;
    for i in 0..=count {
        let x = rect.left() + i as f32 * width;
        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(0.8, Color32::from_gray(120)),
        );
    }
}

fn draw_notes_for_staff(
    painter: &egui::Painter,
    rect: Rect,
    score: &Score,
    staff_index: usize,
    instrument: Instrument,
) {
    if score.notes.is_empty() {
        return;
    }

    let note_count = score.notes.len().min(16);
    let spacing = (rect.width() - 18.0) / note_count as f32;

    for i in 0..note_count {
        let note = &score.notes[i];

        if i % 4 == 0 && note.instrument != instrument {
            continue;
        }

        let x = rect.left() + 12.0 + i as f32 * spacing;
        let y = note_y(rect, note, staff_index);

        draw_notehead(painter, Pos2::new(x, y), note.duration);

        if needs_stem(note.duration) {
            painter.line_segment(
                [Pos2::new(x + 5.5, y), Pos2::new(x + 5.5, y - 25.0)],
                Stroke::new(1.2, Color32::BLACK),
            );
        }

        if note.pitch.class == PitchClass::C && note.pitch.octave >= 5 {
            painter.line_segment(
                [Pos2::new(x - 10.0, y), Pos2::new(x + 10.0, y)],
                Stroke::new(1.0, Color32::BLACK),
            );
        }
    }
}

fn draw_notehead(painter: &egui::Painter, center: Pos2, duration: DurationValue) {
    let fill = if duration == DurationValue::Whole || duration == DurationValue::Half {
        Color32::WHITE
    } else {
        Color32::BLACK
    };

    painter.circle_filled(center, 5.4, fill);
    painter.circle_stroke(center, 5.4, Stroke::new(1.2, Color32::BLACK));
}

fn note_y(rect: Rect, note: &NoteEvent, staff_index: usize) -> f32 {
    let midi = (note.pitch.octave as i32 + 1) * 12 + note.pitch.class.semitone_offset();
    let e4_midi = 64;
    let half_steps = midi - e4_midi;
    let base = rect.center().y + (staff_index % 3) as f32 * 1.0;
    base + 14.0 - half_steps as f32 * 2.5
}

fn needs_stem(duration: DurationValue) -> bool {
    matches!(
        duration,
        DurationValue::Half | DurationValue::Quarter | DurationValue::Eighth
    )
}

fn instrument_short_name(instrument: Instrument) -> &'static str {
    match instrument {
        Instrument::Violin => "Vln.",
        Instrument::Viola => "Vla.",
        Instrument::Cello => "Vcl.",
        Instrument::Flute => "Fl.",
        Instrument::Clarinet => "Cl.",
        Instrument::Trumpet => "Tpt.",
        Instrument::Horn => "Hn.",
        Instrument::Timpani => "Tmp.",
        Instrument::Piano => "Pno.",
    }
}
