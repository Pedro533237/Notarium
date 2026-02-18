use std::{collections::HashMap, fs, path::Path};

use crate::common::id::Id;
use crate::instruments::model::{Clef, Instrument, InstrumentFamily, Pitch};

#[derive(Debug)]
pub enum InstrumentRepositoryError {
    Io(std::io::Error),
    Parse(String),
}

impl From<std::io::Error> for InstrumentRepositoryError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Default)]
pub struct InstrumentRepository {
    instruments: HashMap<Id, Instrument>,
}

impl InstrumentRepository {
    pub fn with_builtin() -> Self {
        let mut repo = Self::default();
        repo.add_builtin_instruments();
        repo
    }

    pub fn insert(&mut self, instrument: Instrument) {
        self.instruments.insert(instrument.id, instrument);
    }

    pub fn get(&self, id: &Id) -> Option<&Instrument> {
        self.instruments.get(id)
    }

    pub fn all(&self) -> impl Iterator<Item = &Instrument> {
        self.instruments.values()
    }

    /// Carrega instrumentos de um JSON externo simples.
    ///
    /// Formato esperado:
    /// `[ { "id":"<hex>", "name":"...", "family":"Ethnic", "midi_program":77, ... } ]`
    pub fn load_json_file<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<usize, InstrumentRepositoryError> {
        let content = fs::read_to_string(path)?;
        self.load_json_str(&content)
    }

    pub fn load_json_str(&mut self, content: &str) -> Result<usize, InstrumentRepositoryError> {
        let mut loaded = 0usize;
        for obj in content.split('{').skip(1) {
            let maybe_obj = obj.split('}').next().unwrap_or_default();
            if maybe_obj.trim().is_empty() {
                continue;
            }

            let name = extract_string(maybe_obj, "name")
                .ok_or_else(|| InstrumentRepositoryError::Parse("missing name".to_string()))?;
            let family =
                parse_family(&extract_string(maybe_obj, "family").ok_or_else(|| {
                    InstrumentRepositoryError::Parse("missing family".to_string())
                })?)?;
            let midi_program = extract_u8(maybe_obj, "midi_program").ok_or_else(|| {
                InstrumentRepositoryError::Parse("missing midi_program".to_string())
            })?;

            let low = extract_u8(maybe_obj, "range_low").unwrap_or(36);
            let high = extract_u8(maybe_obj, "range_high").unwrap_or(96);
            let transposition = extract_i8(maybe_obj, "transposition").unwrap_or(0);
            let clef = parse_clef(
                &extract_string(maybe_obj, "clef").unwrap_or_else(|| "Treble".to_string()),
            )?;

            let id = extract_string(maybe_obj, "id")
                .and_then(|raw| Id::from_hex(raw.trim()))
                .unwrap_or_default();

            self.insert(Instrument {
                id,
                name,
                family,
                range_low: Pitch::new(low),
                range_high: Pitch::new(high),
                transposition,
                clef,
                midi_program,
            });
            loaded += 1;
        }
        Ok(loaded)
    }

    fn add_builtin_instruments(&mut self) {
        let builtin = [
            (
                "Violin",
                InstrumentFamily::Orchestral,
                55,
                103,
                0,
                Clef::Treble,
                40,
            ),
            (
                "Soprano",
                InstrumentFamily::VoiceSatb,
                60,
                84,
                0,
                Clef::Treble,
                53,
            ),
            (
                "Duduk",
                InstrumentFamily::Ethnic,
                50,
                79,
                -2,
                Clef::Treble,
                71,
            ),
            (
                "Lute",
                InstrumentFamily::Baroque,
                40,
                76,
                0,
                Clef::Treble,
                24,
            ),
            (
                "Frame Drum",
                InstrumentFamily::GlobalPercussion,
                35,
                81,
                0,
                Clef::Percussion,
                118,
            ),
            (
                "Synth Pad",
                InstrumentFamily::Electronic,
                36,
                96,
                0,
                Clef::Treble,
                88,
            ),
        ];

        for (name, family, low, high, transposition, clef, midi_program) in builtin {
            self.insert(Instrument {
                id: Id::new(),
                name: name.to_string(),
                family,
                range_low: Pitch::new(low),
                range_high: Pitch::new(high),
                transposition,
                clef,
                midi_program,
            });
        }
    }
}

fn parse_family(value: &str) -> Result<InstrumentFamily, InstrumentRepositoryError> {
    match value {
        "Orchestral" => Ok(InstrumentFamily::Orchestral),
        "Baroque" => Ok(InstrumentFamily::Baroque),
        "Medieval" => Ok(InstrumentFamily::Medieval),
        "Ethnic" => Ok(InstrumentFamily::Ethnic),
        "Electronic" => Ok(InstrumentFamily::Electronic),
        "VoiceSatb" => Ok(InstrumentFamily::VoiceSatb),
        "GlobalPercussion" => Ok(InstrumentFamily::GlobalPercussion),
        _ => Err(InstrumentRepositoryError::Parse(
            "invalid family".to_string(),
        )),
    }
}

fn parse_clef(value: &str) -> Result<Clef, InstrumentRepositoryError> {
    match value {
        "Treble" => Ok(Clef::Treble),
        "Bass" => Ok(Clef::Bass),
        "Alto" => Ok(Clef::Alto),
        "Tenor" => Ok(Clef::Tenor),
        "Percussion" => Ok(Clef::Percussion),
        _ => Err(InstrumentRepositoryError::Parse("invalid clef".to_string())),
    }
}

fn extract_string(input: &str, key: &str) -> Option<String> {
    let marker = format!("\"{}\"", key);
    let start = input.find(&marker)?;
    let after = &input[start + marker.len()..];
    let q1 = after.find('"')?;
    let after_q1 = &after[q1 + 1..];
    let q2 = after_q1.find('"')?;
    Some(after_q1[..q2].to_string())
}

fn extract_u8(input: &str, key: &str) -> Option<u8> {
    extract_int(input, key).and_then(|v| u8::try_from(v).ok())
}

fn extract_i8(input: &str, key: &str) -> Option<i8> {
    extract_int(input, key).and_then(|v| i8::try_from(v).ok())
}

fn extract_int(input: &str, key: &str) -> Option<i32> {
    let marker = format!("\"{}\"", key);
    let start = input.find(&marker)?;
    let after = &input[start + marker.len()..];
    let colon = after.find(':')?;
    let mut digits = String::new();
    for ch in after[colon + 1..].chars() {
        if ch.is_ascii_digit() || ch == '-' {
            digits.push(ch);
        } else if !digits.is_empty() {
            break;
        }
    }
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_instruments_from_external_json() {
        let json = r#"[
            {
              "id":"0000000000000000000000000000002a",
              "name":"Shakuhachi",
              "family":"Ethnic",
              "range_low": 50,
              "range_high": 86,
              "transposition": 0,
              "clef":"Treble",
              "midi_program": 77
            }
        ]"#;

        let mut repo = InstrumentRepository::default();
        let loaded = repo.load_json_str(json).expect("load json");
        assert_eq!(loaded, 1);
        assert_eq!(repo.all().count(), 1);
    }
}
