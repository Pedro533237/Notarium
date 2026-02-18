use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke};

use crate::music::{Staff, StaffSystem};

pub fn draw_staff_system(
    painter: &egui::Painter,
    rect: Rect,
    system: &StaffSystem,
    zoom: f32,
) -> Vec<Rect> {
    let mut out = Vec::with_capacity(system.staffs.len());
    let staff_height = 40.0 * zoom;
    let vertical_gap = 42.0 * zoom;
    let mut y = rect.top() + 120.0 * zoom;

    for staff in &system.staffs {
        let staff_rect = Rect::from_min_max(
            Pos2::new(rect.left() + 95.0 * zoom, y),
            Pos2::new(rect.right() - 24.0 * zoom, y + staff_height),
        );
        draw_staff(painter, staff_rect, staff);
        draw_staff_headers(painter, staff_rect, staff, zoom);

        out.push(staff_rect);
        y += staff_height + vertical_gap;
        if y > rect.bottom() - 80.0 * zoom {
            break;
        }
    }

    out
}

fn draw_staff(painter: &egui::Painter, rect: Rect, staff: &Staff) {
    let spacing =
        (rect.height() / (staff.line_count as f32 - 1.0)) * staff.line_spacing_ssu.max(0.75);
    for line in 0..staff.line_count {
        let y = rect.top() + line as f32 * spacing;
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(1.0, Color32::from_gray(55)),
        );
    }

    if !staff.measures.is_empty() {
        let total_width_ssu: f32 = staff.measures.iter().map(|m| m.width_ssu.max(0.1)).sum();
        for measure in &staff.measures {
            let progress = if total_width_ssu > 0.0 {
                (measure.start_beat / total_width_ssu).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let x = rect.left() + rect.width() * progress;
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                Stroke::new(0.8, Color32::from_gray(110)),
            );
        }

        painter.line_segment(
            [
                Pos2::new(rect.right(), rect.top()),
                Pos2::new(rect.right(), rect.bottom()),
            ],
            Stroke::new(0.9, Color32::from_gray(90)),
        );
    }
}

fn draw_staff_headers(painter: &egui::Painter, rect: Rect, staff: &Staff, zoom: f32) {
    painter.text(
        Pos2::new(rect.left() - 34.0 * zoom, rect.center().y),
        Align2::CENTER_CENTER,
        staff.clef.symbol(),
        FontId::proportional(23.0 * zoom),
        Color32::BLACK,
    );

    let ks = staff.key_signature.accidental_count();
    if ks != 0 {
        let acc = if ks > 0 { "♯" } else { "♭" };
        let count = ks.unsigned_abs() as usize;
        painter.text(
            Pos2::new(rect.left() - 8.0 * zoom, rect.center().y),
            Align2::CENTER_CENTER,
            acc.repeat(count),
            FontId::proportional(14.0 * zoom),
            Color32::from_gray(40),
        );
    }

    painter.text(
        Pos2::new(rect.left() + 25.0 * zoom, rect.center().y),
        Align2::CENTER_CENTER,
        staff.time_signature.label(),
        FontId::proportional(12.0 * zoom),
        Color32::from_gray(40),
    );

    painter.text(
        Pos2::new(rect.left() - 68.0 * zoom, rect.center().y),
        Align2::CENTER_CENTER,
        format!("S{}", staff.index + 1),
        FontId::proportional(10.0 * zoom),
        Color32::from_gray(70),
    );

    if let Some(last) = staff.measures.last() {
        painter.text(
            Pos2::new(rect.right() - 4.0 * zoom, rect.top() - 10.0 * zoom),
            Align2::RIGHT_CENTER,
            format!("m{}", last.index + 1),
            FontId::proportional(9.0 * zoom),
            Color32::from_gray(100),
        );
    }
}
