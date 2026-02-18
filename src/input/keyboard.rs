use egui::Key;

use crate::music::{Accidental, Note, NoteDuration, PitchClass};

pub enum KeyboardAction {
    Delete,
    Duration(NoteDuration),
    PitchStep(i8),
    InsertPitch(PitchClass),
    SetAccidental(Accidental),
}

pub fn collect_actions(ctx: &egui::Context) -> Vec<KeyboardAction> {
    let mut actions = Vec::new();

    if ctx.input(|i| i.key_pressed(Key::Delete)) {
        actions.push(KeyboardAction::Delete);
    }
    if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
        actions.push(KeyboardAction::PitchStep(1));
    }
    if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
        actions.push(KeyboardAction::PitchStep(-1));
    }

    for (key, duration) in [
        (Key::Num1, NoteDuration::Whole),
        (Key::Num2, NoteDuration::Half),
        (Key::Num3, NoteDuration::Quarter),
        (Key::Num4, NoteDuration::Eighth),
        (Key::Num5, NoteDuration::Sixteenth),
        (Key::Num6, NoteDuration::ThirtySecond),
        (Key::Num7, NoteDuration::SixtyFourth),
    ] {
        if ctx.input(|i| i.key_pressed(key)) {
            actions.push(KeyboardAction::Duration(duration));
        }
    }

    for (key, pitch) in [
        (Key::A, PitchClass::C),
        (Key::S, PitchClass::D),
        (Key::D, PitchClass::E),
        (Key::F, PitchClass::F),
        (Key::G, PitchClass::G),
        (Key::H, PitchClass::A),
        (Key::J, PitchClass::B),
    ] {
        if ctx.input(|i| i.key_pressed(key)) {
            actions.push(KeyboardAction::InsertPitch(pitch));
        }
    }

    if ctx.input(|i| i.key_pressed(Key::Q)) {
        actions.push(KeyboardAction::SetAccidental(Accidental::Sharp));
    }
    if ctx.input(|i| i.key_pressed(Key::W)) {
        actions.push(KeyboardAction::SetAccidental(Accidental::Flat));
    }
    if ctx.input(|i| i.key_pressed(Key::E)) {
        actions.push(KeyboardAction::SetAccidental(Accidental::Natural));
    }
    if ctx.input(|i| i.key_pressed(Key::R)) {
        actions.push(KeyboardAction::SetAccidental(Accidental::None));
    }

    actions
}

pub fn apply_pitch_step(note: &mut Note, step: i8) {
    let classes = PitchClass::ALL;
    let mut idx = classes
        .iter()
        .position(|pc| *pc == note.pitch.class)
        .unwrap_or(0) as i32;
    idx += step as i32;

    while idx < 0 {
        note.pitch.octave -= 1;
        idx += classes.len() as i32;
    }
    while idx >= classes.len() as i32 {
        note.pitch.octave += 1;
        idx -= classes.len() as i32;
    }

    note.pitch.class = classes[idx as usize];
}
