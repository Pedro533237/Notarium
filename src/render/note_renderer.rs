use egui::{Color32, Pos2, Rect, Stroke, Vec2};

use crate::editor::SelectionOverlay;
use crate::music::{Note, NoteDuration, Staff};

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

    let note_count = notes.len().min(64);
    let spacing = (staff_rect.width() - 20.0) / note_count as f32;

    for (index, note) in notes.iter().take(note_count).enumerate() {
        let x = staff_rect.left() + 10.0 + index as f32 * spacing;
        let y = y_for_staff_position(staff_rect, staff, note.staff_position);
        let center = Pos2::new(x, y);

        draw_notehead(painter, center, note.duration, note.opacity);
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

        let rect = Rect::from_center_size(center, Vec2::new(16.0, 28.0));
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

    overlays
}

fn draw_notehead(painter: &egui::Painter, center: Pos2, duration: NoteDuration, opacity: f32) {
    let alpha = (255.0 * opacity).clamp(0.0, 255.0) as u8;
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
