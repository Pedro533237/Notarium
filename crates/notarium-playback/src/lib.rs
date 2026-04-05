//! Playback simples baseado em CPAL.

use std::f32::consts::PI;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use notarium_core::{Instrument, Note, Score};

#[derive(Default)]
struct EngineState {
    playing: bool,
    bpm: f32,
    t: f32,
    notes: Vec<Scheduled>,
}

#[derive(Clone)]
struct Scheduled {
    start: f32,
    end: f32,
    freq: f32,
    instrument: Instrument,
}

pub struct PlaybackEngine {
    state: Arc<Mutex<EngineState>>,
    _stream: cpal::Stream,
}

impl PlaybackEngine {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("Sem dispositivo de áudio")?;
        let config = device.default_output_config().map_err(|e| e.to_string())?;
        let sample_rate = config.sample_rate().0 as f32;

        let state = Arc::new(Mutex::new(EngineState {
            bpm: 120.0,
            ..Default::default()
        }));
        let st = state.clone();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                build_stream::<f32>(&device, &config.into(), st, sample_rate)?
            }
            cpal::SampleFormat::I16 => {
                build_stream::<i16>(&device, &config.into(), st, sample_rate)?
            }
            cpal::SampleFormat::U16 => {
                build_stream::<u16>(&device, &config.into(), st, sample_rate)?
            }
            _ => return Err("Formato de áudio não suportado".to_owned()),
        };
        stream.play().map_err(|e| e.to_string())?;

        Ok(Self {
            state,
            _stream: stream,
        })
    }

    pub fn play(&self, score: &Score) {
        let time = 0.0;
        let mut sched = vec![];
        let beat_sec = 60.0 / score.bpm as f32;

        for staff in &score.staves {
            for measure in &staff.measures {
                for note in &measure.notes {
                    sched.push(to_scheduled(
                        note,
                        measure.number as f32,
                        beat_sec,
                        staff.instrument,
                    ));
                }
            }
        }

        sched.sort_by(|a, b| a.start.total_cmp(&b.start));

        if let Ok(mut st) = self.state.lock() {
            st.bpm = score.bpm as f32;
            st.notes = sched;
            st.t = time;
            st.playing = true;
        }
    }

    pub fn stop(&self) {
        if let Ok(mut st) = self.state.lock() {
            st.playing = false;
            st.t = 0.0;
        }
    }
}

fn to_scheduled(
    note: &Note,
    measure_number: f32,
    beat_sec: f32,
    instrument: Instrument,
) -> Scheduled {
    let measure_start_beat = (measure_number - 1.0) * 4.0;
    let start = (measure_start_beat + note.beat_offset) * beat_sec;
    let end = start + note.duration.beats() * beat_sec;
    Scheduled {
        start,
        end,
        freq: note.pitch.frequency_hz(),
        instrument,
    }
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    state: Arc<Mutex<EngineState>>,
    sample_rate: f32,
) -> Result<cpal::Stream, String>
where
    T: cpal::Sample + cpal::FromSample<f32> + cpal::SizedSample,
{
    let channels = config.channels as usize;
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _| write_data(data, channels, sample_rate, &state),
            move |err| eprintln!("Erro de áudio: {err}"),
            None,
        )
        .map_err(|e| e.to_string())
}

fn write_data<T: cpal::Sample + cpal::FromSample<f32>>(
    output: &mut [T],
    channels: usize,
    sample_rate: f32,
    state: &Arc<Mutex<EngineState>>,
) {
    let mut st = match state.lock() {
        Ok(v) => v,
        Err(_) => return,
    };

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;
        if st.playing {
            for n in &st.notes {
                if st.t >= n.start && st.t <= n.end {
                    let phase = 2.0 * PI * n.freq * st.t;
                    sample += wave(phase, n.instrument) * 0.15;
                }
            }

            let beat_sec = 60.0 / st.bpm.max(1.0);
            let in_beat = (st.t % beat_sec) / beat_sec;
            if in_beat < 0.03 {
                sample += 0.3 * (2.0 * PI * 1500.0 * st.t).sin();
            }
            st.t += 1.0 / sample_rate;
        }

        for out in frame.iter_mut() {
            *out = T::from_sample(sample);
        }
    }
}

fn wave(phase: f32, instrument: Instrument) -> f32 {
    match instrument {
        Instrument::Piano => (phase.sin() + 0.5 * (phase * 2.0).sin()) / 1.5,
        Instrument::Violin | Instrument::Viola => (phase.sin() + 0.3 * (phase * 3.0).sin()) / 1.3,
        Instrument::Cello => (phase.sin() + 0.25 * (phase * 0.5).sin()) / 1.25,
        Instrument::Flute => phase.sin(),
        Instrument::Clarinet => (phase.sin() + 0.4 * (phase * 3.0).sin()) / 1.4,
        Instrument::Trumpet => (phase.sin() + 0.35 * (phase * 2.0).sin()) / 1.35,
    }
}
