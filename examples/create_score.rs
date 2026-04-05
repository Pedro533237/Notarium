use notarium_core::{Accidental, Note, NoteDuration, Pitch, PitchClass, Score};

fn main() {
    let mut score = Score::default();

    score.insert_note(
        0,
        0,
        Note {
            pitch: Pitch {
                class: PitchClass::C,
                accidental: Accidental::Natural,
                octave: 4,
            },
            duration: NoteDuration::Quarter,
            beat_offset: 0.0,
            voice: 0,
        },
    );

    println!("Partitura pronta: {}", score.title);
}
