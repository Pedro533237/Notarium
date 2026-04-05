//! Motor de renderização vetorial de partitura para egui.

use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use notarium_core::{Accidental, Note, NoteDuration, PitchClass, Score};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Select,
    Eraser,
    Pen,
}

#[derive(Debug, Clone)]
pub struct HitInfo {
    pub staff_idx: usize,
    pub measure_idx: usize,
    pub note_idx: Option<usize>,
    pub beat: f32,
    pub pitch_class: PitchClass,
    pub octave: i8,
}

pub fn draw_score(ui: &mut Ui, score: &Score, zoom: f32) -> (Response, Vec<Rect>) {
    let measure_count = score
        .staves
        .first()
        .map(|s| s.measures.len())
        .unwrap_or(1)
        .max(1);
    let width = 280.0 * measure_count as f32 * zoom;
    let height = 160.0 * score.staves.len() as f32 * zoom + 80.0;
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(width.max(ui.available_width()), height),
        Sense::click(),
    );
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 6.0, Color32::from_rgb(250, 248, 242));
    painter.text(
        Pos2::new(rect.left() + 16.0, rect.top() + 12.0),
        egui::Align2::LEFT_TOP,
        format!("{} — {}", score.title, score.composer),
        egui::FontId::proportional(20.0),
        Color32::BLACK,
    );

    let mut measure_rects = Vec::new();
    let staff_height = 70.0 * zoom;
    let measure_width = 260.0 * zoom;
    let mut y = rect.top() + 50.0;

    for staff in &score.staves {
        painter.text(
            Pos2::new(rect.left() + 10.0, y + 22.0),
            egui::Align2::LEFT_CENTER,
            &staff.name,
            egui::FontId::proportional(14.0),
            Color32::DARK_GRAY,
        );

        for line in 0..5 {
            let line_y = y + line as f32 * (staff_height / 4.0);
            painter.line_segment(
                [
                    Pos2::new(rect.left() + 90.0, line_y),
                    Pos2::new(rect.right() - 16.0, line_y),
                ],
                Stroke::new(1.0, Color32::BLACK),
            );
        }

        for m_idx in 0..measure_count {
            let x = rect.left() + 90.0 + m_idx as f32 * measure_width;
            let mr = Rect::from_min_size(Pos2::new(x, y), Vec2::new(measure_width, staff_height));
            measure_rects.push(mr);
            painter.line_segment(
                [Pos2::new(x, y), Pos2::new(x, y + staff_height)],
                Stroke::new(1.0, Color32::GRAY),
            );
            if let Some(measure) = staff.measures.get(m_idx) {
                for note in &measure.notes {
                    draw_note(&painter, mr, note);
                }
            }
        }

        let end_x = rect.left() + 90.0 + measure_count as f32 * measure_width;
        painter.line_segment(
            [Pos2::new(end_x, y), Pos2::new(end_x, y + staff_height)],
            Stroke::new(2.0, Color32::BLACK),
        );
        y += 120.0 * zoom;
    }

    (response, measure_rects)
}

fn draw_note(p: &egui::Painter, measure_rect: Rect, note: &Note) {
    let x = measure_rect.left() + 24.0 + note.beat_offset * 50.0;
    let y = pitch_to_y(measure_rect, note);
    let fill = if matches!(note.duration, NoteDuration::Whole | NoteDuration::Half) {
        Color32::WHITE
    } else {
        Color32::BLACK
    };
    p.circle_filled(Pos2::new(x, y), 6.0, fill);
    p.circle_stroke(Pos2::new(x, y), 6.0, Stroke::new(1.2, Color32::BLACK));

    if !matches!(note.duration, NoteDuration::Whole) {
        p.line_segment(
            [Pos2::new(x + 6.0, y), Pos2::new(x + 6.0, y - 28.0)],
            Stroke::new(1.2, Color32::BLACK),
        );
    }

    if !matches!(note.pitch.accidental, Accidental::Natural) {
        p.text(
            Pos2::new(x - 14.0, y),
            egui::Align2::CENTER_CENTER,
            note.pitch.accidental.label(),
            egui::FontId::proportional(16.0),
            Color32::BLACK,
        );
    }
}

fn pitch_to_y(rect: Rect, note: &Note) -> f32 {
    let midi = note.pitch.midi_number();
    let e4 = 64;
    rect.center().y + 14.0 - (midi - e4) as f32 * 2.5
}

pub fn hit_test(
    measure_rects: &[Rect],
    score: &Score,
    pos: Pos2,
    beat_unit: u8,
) -> Option<HitInfo> {
    if beat_unit == 0 {
        return None;
    }

    let measure_per_staff = score.staves.first()?.measures.len().max(1);

    for (idx, rect) in measure_rects.iter().enumerate() {
        if !rect.contains(pos) {
            continue;
        }

        let staff_idx = idx / measure_per_staff;
        let measure_idx = idx % measure_per_staff;
        let beat = ((pos.x - rect.left() - 24.0) / 50.0).max(0.0);
        let semis = ((rect.center().y + 14.0 - pos.y) / 2.5).round() as i32;
        let midi = 64 + semis;
        let octave = (midi / 12 - 1) as i8;
        let pc = match midi.rem_euclid(12) {
            0 | 1 => PitchClass::C,
            2 | 3 => PitchClass::D,
            4 => PitchClass::E,
            5 | 6 => PitchClass::F,
            7 | 8 => PitchClass::G,
            9 | 10 => PitchClass::A,
            _ => PitchClass::B,
        };

        let note_idx = score
            .staves
            .get(staff_idx)
            .and_then(|s| s.measures.get(measure_idx))
            .and_then(|m| {
                m.notes
                    .iter()
                    .enumerate()
                    .find(|(_, n)| (n.beat_offset - beat).abs() < 0.35)
                    .map(|(i, _)| i)
            });

        return Some(HitInfo {
            staff_idx,
            measure_idx,
            note_idx,
            beat,
            pitch_class: pc,
            octave,
        });
    }

    None
}
