use egui::{Color32, Pos2, Rect, Stroke, Vec2};

use crate::editor::SelectionOverlay;
use crate::music::{Accidental, Note, NoteDuration, Staff};

pub fn draw_notes(
    painter: &egui::Painter,
    staff_rect: Rect,
    staff: &Staff,
    notes: &[Note],
) -> Vec<SelectionOverlay> {
    let mut overlays = Vec::new();
    if notes.is_empty() {
        return overlays;
    }

    let visible: Vec<&Note> = notes.iter().take(200).collect();
    let total_beats: f32 = visible
        .iter()
        .map(|n| n.duration.beats())
        .sum::<f32>()
        .max(1.0);
    let mut running_beats = 0.0;

    for note in visible {
        let x =
            staff_rect.left() + 12.0 + (running_beats / total_beats) * (staff_rect.width() - 24.0);
        running_beats += note.duration.beats();

        let y = y_for_staff_position(staff_rect, staff, note.staff_position);
        let center = Pos2::new(x, y);

        draw_notehead(painter, center, note.duration, note.opacity, note.velocity);
        draw_accidental(painter, note.pitch.accidental, center, note.opacity);

        if note.dotted {
            painter.circle_filled(
                Pos2::new(x + 10.0, y - 1.0),
                1.8,
                Color32::from_black_alpha((255.0 * note.opacity) as u8),
            );
        }

        if needs_stem(note.duration) {
            draw_stem_and_flags(
                painter,
                center,
                note.duration,
                note.opacity,
                note.stem_direction_up(),
            );
        }

        if note.tie_start {
            painter.line_segment(
                [Pos2::new(x + 7.0, y + 7.0), Pos2::new(x + 16.0, y + 8.5)],
                Stroke::new(1.0, Color32::from_gray(35)),
            );
        }

        if note.tie_end {
            painter.line_segment(
                [Pos2::new(x - 16.0, y + 8.5), Pos2::new(x - 7.0, y + 7.0)],
                Stroke::new(1.0, Color32::from_gray(35)),
            );
        }

        let rect = Rect::from_center_size(center, Vec2::new(18.0, 30.0));
        if note.selected {
            painter.rect_stroke(
                rect,
                2.0,
                Stroke::new(1.0, Color32::from_rgb(75, 140, 255)),
                egui::StrokeKind::Outside,
            );
        }

        overlays.push(SelectionOverlay {
            note_id: note.id,
            rect,
        });
    }

    draw_simple_beams(painter, &overlays, notes);
    overlays
}

fn draw_simple_beams(painter: &egui::Painter, overlays: &[SelectionOverlay], notes: &[Note]) {
    for window in overlays.windows(2).zip(notes.windows(2)) {
        let (overlay_pair, note_pair) = window;
        if note_pair[0].duration.beats() <= 0.5 && note_pair[1].duration.beats() <= 0.5 {
            let a = overlay_pair[0].rect.center_top() + egui::vec2(5.0, -22.0);
            let b = overlay_pair[1].rect.center_top() + egui::vec2(5.0, -22.0);
            painter.line_segment([a, b], Stroke::new(2.0, Color32::BLACK));
        }
    }
}

fn draw_accidental(painter: &egui::Painter, accidental: Accidental, center: Pos2, opacity: f32) {
    if accidental == Accidental::None {
        return;
    }
    let alpha = (255.0 * opacity).clamp(0.0, 255.0) as u8;
    painter.text(
        Pos2::new(center.x - 13.0, center.y),
        egui::Align2::CENTER_CENTER,
        accidental.symbol(),
        egui::FontId::proportional(14.0),
        Color32::from_rgba_unmultiplied(20, 20, 20, alpha),
    );
}

fn draw_notehead(
    painter: &egui::Painter,
    center: Pos2,
    duration: NoteDuration,
    opacity: f32,
    velocity: u8,
) {
    let velocity_alpha = (velocity as f32 / 127.0).clamp(0.45, 1.0);
    let alpha = (255.0 * opacity * velocity_alpha).clamp(0.0, 255.0) as u8;
    let fill = if matches!(duration, NoteDuration::Whole | NoteDuration::Half) {
        Color32::from_rgba_unmultiplied(255, 255, 255, alpha)
    } else {
        Color32::from_rgba_unmultiplied(20, 20, 20, alpha)
    };

    painter.circle_filled(center, 5.4, fill);
    painter.circle_stroke(
        center,
        5.4,
        Stroke::new(1.2, Color32::from_rgba_unmultiplied(0, 0, 0, alpha)),
    );
}

fn draw_stem_and_flags(
    painter: &egui::Painter,
    center: Pos2,
    duration: NoteDuration,
    opacity: f32,
    up: bool,
) {
    let alpha = (255.0 * opacity).clamp(0.0, 255.0) as u8;
    let stroke = Stroke::new(1.1, Color32::from_rgba_unmultiplied(0, 0, 0, alpha));
    let stem_x = if up { center.x + 5.5 } else { center.x - 5.5 };
    let stem_top = if up { center.y - 25.0 } else { center.y + 25.0 };

    painter.line_segment(
        [Pos2::new(stem_x, center.y), Pos2::new(stem_x, stem_top)],
        stroke,
    );

    for flag_idx in 0..duration.flag_count() {
        let offset = flag_idx as f32 * 4.0;
        let (start, end) = if up {
            (
                Pos2::new(stem_x, stem_top + offset),
                Pos2::new(stem_x + 6.0, stem_top + 4.0 + offset),
            )
        } else {
            (
                Pos2::new(stem_x, stem_top - offset),
                Pos2::new(stem_x - 6.0, stem_top - 4.0 - offset),
            )
        };
        painter.line_segment([start, end], stroke);
    }
}

fn y_for_staff_position(staff_rect: Rect, _staff: &Staff, staff_position: f32) -> f32 {
    let center = staff_rect.center().y;
    center - (staff_position * (staff_rect.height() / 8.0))
}

fn needs_stem(duration: NoteDuration) -> bool {
    !matches!(duration, NoteDuration::Whole)
}

trait StemDirectionExt {
    fn stem_direction_up(&self) -> bool;
}

impl StemDirectionExt for Note {
    fn stem_direction_up(&self) -> bool {
        matches!(self.stem_direction, crate::music::StemDirection::Up)
    }
}
