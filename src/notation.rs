use egui::{self, Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};

use crate::music::{DurationValue, Instrument, NoteEvent, PitchClass, Score};

#[derive(Debug, Clone, Copy)]
pub struct NotePlacement {
    pub instrument: Instrument,
    pub insert_index: usize,
    pub pitch: crate::music::Pitch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardTool {
    None,
    Insert,
}

pub fn draw_orchestral_page(
    ui: &mut egui::Ui,
    score: &Score,
    instruments: &[Instrument],
    page_label: &str,
    zoom_percent: f32,
    selected_tool: KeyboardTool,
) -> Option<NotePlacement> {
    let zoom = (zoom_percent / 100.0).clamp(0.5, 2.0);
    let desired_size = Vec2::new((860.0 * zoom).max(ui.available_width()), 1180.0 * zoom);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    let painter = ui.painter_at(rect);
    let mut placement = None;

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

        if selected_tool == KeyboardTool::Insert
            && response.clicked()
            && response.interact_pointer_pos().is_some_and(|pos| {
                staff_rect
                    .expand2(Vec2::new(0.0, 24.0 * zoom))
                    .contains(pos)
            })
        {
            if let Some(pointer) = response.interact_pointer_pos() {
                let pitch = pitch_from_y(staff_rect, pointer.y);
                let insert_index = slot_from_x(staff_rect, pointer.x);
                placement = Some(NotePlacement {
                    instrument: *instrument,
                    insert_index,
                    pitch,
                });
            }
        }

        y += 78.0 * zoom;
        if y > rect.bottom() - 90.0 * zoom {
            break;
        }
    }

    placement
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

    let mut local_index = 0;
    let staff_notes = score.notes.iter().filter(|n| n.instrument == instrument);
    let note_count = staff_notes.clone().count().clamp(1, 24);
    let spacing = (rect.width() - 18.0) / note_count as f32;

    for note in staff_notes.take(24) {
        if note.instrument != instrument {
            continue;
        }

        let x = rect.left() + 12.0 + local_index as f32 * spacing;
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

        local_index += 1;
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
        DurationValue::Half
            | DurationValue::Quarter
            | DurationValue::Eighth
            | DurationValue::Sixteenth
            | DurationValue::ThirtySecond
            | DurationValue::SixtyFourth
    )
}

fn pitch_from_y(rect: Rect, y: f32) -> crate::music::Pitch {
    let mut best = (0_u8, 0.0_f32);
    for midi in 36..=96 {
        let approx_y = rect.center().y + 14.0 - (midi - 64) as f32 * 2.5;
        let dist = (approx_y - y).abs();
        if midi == 36 || dist < best.1 {
            best = (midi as u8, dist);
        }
    }

    midi_to_pitch(best.0)
}

fn slot_from_x(rect: Rect, x: f32) -> usize {
    let normalized = ((x - rect.left()) / rect.width()).clamp(0.0, 1.0);
    (normalized * 64.0).round() as usize
}

fn midi_to_pitch(midi: u8) -> crate::music::Pitch {
    let semitone = midi % 12;
    let octave = midi as i8 / 12 - 1;
    let class = match semitone {
        0 | 1 => PitchClass::C,
        2 | 3 => PitchClass::D,
        4 => PitchClass::E,
        5 | 6 => PitchClass::F,
        7 | 8 => PitchClass::G,
        9 | 10 => PitchClass::A,
        _ => PitchClass::B,
    };

    crate::music::Pitch { class, octave }
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
