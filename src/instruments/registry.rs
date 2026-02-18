use crate::core::project::Uuid;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentFamily {
    Orchestral,
    Baroque,
    Medieval,
    Ethnic,
    Electronic,
    Voice,
    Percussion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clef {
    Treble,
    Bass,
    Alto,
    Tenor,
    Percussion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pitch {
    pub midi: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    pub id: Uuid,
    pub name: String,
    pub family: InstrumentFamily,
    pub range_low: Pitch,
    pub range_high: Pitch,
    pub transposition: i8,
    pub clef: Clef,
    pub midi_program: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentRegistryError {
    Parse(String),
    Validation(String),
}

impl Display for InstrumentRegistryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(msg) => write!(f, "falha ao ler banco de instrumentos JSON: {msg}"),
            Self::Validation(msg) => write!(f, "instrumento inválido: {msg}"),
        }
    }
}

impl std::error::Error for InstrumentRegistryError {}

#[derive(Debug, Clone)]
pub struct InstrumentRegistry {
    by_id: HashMap<Uuid, Instrument>,
    by_name: HashMap<String, Uuid>,
}

impl InstrumentRegistry {
    pub fn load_from_json(json: &str) -> Result<Self, InstrumentRegistryError> {
        let mut by_id = HashMap::new();
        let mut by_name = HashMap::new();

        for object in split_json_objects(json)? {
            let instrument = parse_instrument(object)?;
            if instrument.range_low.midi > instrument.range_high.midi {
                return Err(InstrumentRegistryError::Validation(format!(
                    "range inválido para {}",
                    instrument.name
                )));
            }
            by_name.insert(instrument.name.to_lowercase(), instrument.id.clone());
            by_id.insert(instrument.id.clone(), instrument);
        }

        Ok(Self { by_id, by_name })
    }

    pub fn load_embedded() -> Result<Self, InstrumentRegistryError> {
        Self::load_from_json(include_str!(
            "../../assets/instruments/default_instruments.json"
        ))
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Instrument> {
        self.by_name
            .get(&name.to_lowercase())
            .and_then(|id| self.by_id.get(id))
    }

    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }
}

fn split_json_objects(input: &str) -> Result<Vec<&str>, InstrumentRegistryError> {
    let trimmed = input.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(InstrumentRegistryError::Parse(
            "array JSON esperado".to_owned(),
        ));
    }

    let mut objs = Vec::new();
    let mut depth = 0usize;
    let mut start = None;
    for (i, ch) in trimmed.char_indices() {
        match ch {
            '{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    if let Some(s) = start {
                        objs.push(&trimmed[s..=i]);
                    }
                    start = None;
                }
            }
            _ => {}
        }
    }
    Ok(objs)
}

fn parse_instrument(obj: &str) -> Result<Instrument, InstrumentRegistryError> {
    Ok(Instrument {
        id: json_string(obj, "id")?,
        name: json_string(obj, "name")?,
        family: parse_family(&json_string(obj, "family")?)?,
        range_low: Pitch {
            midi: json_number(obj, "range_low")? as u8,
        },
        range_high: Pitch {
            midi: json_number(obj, "range_high")? as u8,
        },
        transposition: json_i8(obj, "transposition")?,
        clef: parse_clef(&json_string(obj, "clef")?)?,
        midi_program: json_number(obj, "midi_program")? as u8,
    })
}

fn parse_family(value: &str) -> Result<InstrumentFamily, InstrumentRegistryError> {
    match value {
        "Orchestral" => Ok(InstrumentFamily::Orchestral),
        "Baroque" => Ok(InstrumentFamily::Baroque),
        "Medieval" => Ok(InstrumentFamily::Medieval),
        "Ethnic" => Ok(InstrumentFamily::Ethnic),
        "Electronic" => Ok(InstrumentFamily::Electronic),
        "Voice" => Ok(InstrumentFamily::Voice),
        "Percussion" => Ok(InstrumentFamily::Percussion),
        other => Err(InstrumentRegistryError::Parse(format!(
            "família desconhecida: {other}"
        ))),
    }
}

fn parse_clef(value: &str) -> Result<Clef, InstrumentRegistryError> {
    match value {
        "Treble" => Ok(Clef::Treble),
        "Bass" => Ok(Clef::Bass),
        "Alto" => Ok(Clef::Alto),
        "Tenor" => Ok(Clef::Tenor),
        "Percussion" => Ok(Clef::Percussion),
        other => Err(InstrumentRegistryError::Parse(format!(
            "clave desconhecida: {other}"
        ))),
    }
}

fn json_string(obj: &str, key: &str) -> Result<String, InstrumentRegistryError> {
    let pattern = format!("\"{key}\":");
    let start = obj
        .find(&pattern)
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("campo ausente: {key}")))?;
    let tail = &obj[start + pattern.len()..];
    let first = tail
        .find('"')
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("valor string inválido: {key}")))?;
    let rest = &tail[first + 1..];
    let end = rest
        .find('"')
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("valor string inválido: {key}")))?;
    Ok(rest[..end].to_owned())
}

fn json_number(obj: &str, key: &str) -> Result<i32, InstrumentRegistryError> {
    if key == "range_low" || key == "range_high" {
        let section = find_section(obj, key)?;
        return json_number(section, "midi");
    }
    let pattern = format!("\"{key}\":");
    let start = obj
        .find(&pattern)
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("campo ausente: {key}")))?;
    let tail = obj[start + pattern.len()..].trim_start();
    let mut end = 0usize;
    for (i, ch) in tail.char_indices() {
        if !(ch.is_ascii_digit() || ch == '-') {
            break;
        }
        end = i + 1;
    }
    tail[..end]
        .parse::<i32>()
        .map_err(|_| InstrumentRegistryError::Parse(format!("número inválido: {key}")))
}

fn json_i8(obj: &str, key: &str) -> Result<i8, InstrumentRegistryError> {
    let value = json_number(obj, key)?;
    i8::try_from(value).map_err(|_| InstrumentRegistryError::Parse(format!("i8 inválido: {key}")))
}

fn find_section<'a>(obj: &'a str, key: &str) -> Result<&'a str, InstrumentRegistryError> {
    let pattern = format!("\"{key}\":");
    let start = obj
        .find(&pattern)
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("campo ausente: {key}")))?;
    let tail = &obj[start + pattern.len()..];
    let open = tail
        .find('{')
        .ok_or_else(|| InstrumentRegistryError::Parse(format!("objeto inválido: {key}")))?;
    let mut depth = 0usize;
    for (i, ch) in tail[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&tail[open..=open + i]);
                }
            }
            _ => {}
        }
    }
    Err(InstrumentRegistryError::Parse(format!(
        "objeto não fechado: {key}"
    )))
}

#[cfg(test)]
mod tests {
    use super::InstrumentRegistry;

    #[test]
    fn loads_embedded_json() {
        let registry = InstrumentRegistry::load_embedded().expect("embedded JSON should be valid");
        assert!(!registry.is_empty());
        assert!(registry.find_by_name("Violin I").is_some());
        assert!(registry.find_by_name("Soprano Voice").is_some());
        assert!(registry.find_by_name("Taiko").is_some());
    }
}
