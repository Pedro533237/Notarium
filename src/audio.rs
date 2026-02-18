use std::f32::consts::PI;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

use crate::music::{Instrument, NoteEvent, Score};

const SAMPLE_RATE: u32 = 44_100;

enum PlaybackCommand {
    Play {
        score: Score,
        bpm: f32,
        config: PlaybackConfig,
    },
    Pause,
    Resume,
    Stop,
    Rewind,
}

#[derive(Clone)]
pub struct PlaybackController {
    tx: Sender<PlaybackCommand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackEngine {
    Notarium,
    Vst,
}

#[derive(Debug, Clone)]
pub struct PlaybackConfig {
    pub engine: PlaybackEngine,
    pub vst_host: String,
    pub vst_plugin: String,
    pub noteperformer_profile: bool,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            engine: PlaybackEngine::Notarium,
            vst_host: "Builtin Host".to_owned(),
            vst_plugin: "".to_owned(),
            noteperformer_profile: false,
        }
    }
}

impl PlaybackController {
    pub fn play(&self, score: Score, bpm: f32, config: PlaybackConfig) {
        let _ = self.tx.send(PlaybackCommand::Play { score, bpm, config });
    }

    pub fn pause(&self) {
        let _ = self.tx.send(PlaybackCommand::Pause);
    }

    pub fn resume(&self) {
        let _ = self.tx.send(PlaybackCommand::Resume);
    }

    pub fn stop(&self) {
        let _ = self.tx.send(PlaybackCommand::Stop);
    }

    pub fn rewind(&self) {
        let _ = self.tx.send(PlaybackCommand::Rewind);
    }
}

pub fn create_playback_controller() -> PlaybackController {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || playback_thread(rx));
    PlaybackController { tx }
}

fn playback_thread(rx: Receiver<PlaybackCommand>) {
    let Ok((_stream, handle)) = OutputStream::try_default() else {
        return;
    };

    let mut sink: Option<Sink> = None;
    let mut last_score: Option<Score> = None;
    let mut last_bpm = 110.0;
    let mut last_config = PlaybackConfig::default();

    while let Ok(cmd) = rx.recv() {
        match cmd {
            PlaybackCommand::Play { score, bpm, config } => {
                last_score = Some(score);
                last_bpm = bpm;
                last_config = config;
                sink = create_sink_with_score(&handle, last_score.as_ref(), last_bpm, &last_config);
            }
            PlaybackCommand::Pause => {
                if let Some(current) = &sink {
                    current.pause();
                }
            }
            PlaybackCommand::Resume => {
                if let Some(current) = &sink {
                    current.play();
                }
            }
            PlaybackCommand::Stop => {
                if let Some(current) = sink.take() {
                    current.stop();
                }
            }
            PlaybackCommand::Rewind => {
                if let Some(current) = sink.take() {
                    current.stop();
                }
                sink = create_sink_with_score(&handle, last_score.as_ref(), last_bpm, &last_config);
            }
        }
    }
}

fn create_sink_with_score(
    handle: &rodio::OutputStreamHandle,
    score: Option<&Score>,
    bpm: f32,
    config: &PlaybackConfig,
) -> Option<Sink> {
    let score = score?;

    if config.engine == PlaybackEngine::Vst {
        return None;
    }

    let Ok(sink) = Sink::try_new(handle) else {
        return None;
    };

    for note in &score.notes {
        let beat_duration_s = 60.0 / bpm.max(20.0);
        let duration_s = note.duration.beats() * beat_duration_s;
        let samples = synthesize_note(note, duration_s);
        let buffer = SamplesBuffer::new(1, SAMPLE_RATE, samples);
        sink.append(buffer);
    }

    Some(sink)
}

fn synthesize_note(note: &NoteEvent, duration_s: f32) -> Vec<f32> {
    let frequency = note.pitch.frequency_hz_with_accidental(note.accidental);
    let sample_count = (duration_s * SAMPLE_RATE as f32).max(1.0) as usize;
    let mut out = Vec::with_capacity(sample_count);

    for index in 0..sample_count {
        let t = index as f32 / SAMPLE_RATE as f32;
        let phase = 2.0 * PI * frequency * t;
        let harmonic = harmonic_mix(phase, note.instrument);
        let env = envelope(t, duration_s);
        out.push(harmonic * env * 0.35);
    }

    out
}

fn harmonic_mix(phase: f32, instrument: Instrument) -> f32 {
    match instrument {
        Instrument::Violin | Instrument::Viola => {
            (phase.sin() + 0.35 * (2.0 * phase).sin() + 0.2 * (3.0 * phase).sin()) / 1.55
        }
        Instrument::Cello | Instrument::Horn => {
            (phase.sin() + 0.45 * (0.5 * phase).sin() + 0.2 * (2.0 * phase).sin()) / 1.65
        }
        Instrument::Flute => (phase.sin() + 0.1 * (2.0 * phase).sin()) / 1.1,
        Instrument::Clarinet => {
            (phase.sin() + 0.55 * (3.0 * phase).sin() + 0.3 * (5.0 * phase).sin()) / 1.85
        }
        Instrument::Trumpet => {
            (phase.sin() + 0.4 * (2.0 * phase).sin() + 0.25 * (4.0 * phase).sin()) / 1.65
        }
        Instrument::Timpani => ((phase.sin() * 0.8) + (0.27 * (1.6 * phase).sin())) / 1.07,
        Instrument::Piano => {
            (phase.sin() + 0.5 * (2.0 * phase).sin() + 0.3 * (4.0 * phase).sin()) / 1.8
        }
    }
}

fn envelope(t: f32, total: f32) -> f32 {
    let attack = 0.02_f32.min(total * 0.25);
    let release_start = (total - 0.08).max(total * 0.7);

    if t < attack {
        (t / attack).clamp(0.0, 1.0)
    } else if t > release_start {
        ((total - t) / (total - release_start)).clamp(0.0, 1.0)
    } else {
        0.9
    }
}
