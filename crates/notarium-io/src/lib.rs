//! Persistência (.notarium em JSON) e exportação MusicXML.

use std::{fs, path::Path};

use notarium_core::Score;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("falha de IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("falha de serialização JSON: {0}")]
    Serde(#[from] serde_json::Error),
}

pub fn save_notarium(path: impl AsRef<Path>, score: &Score) -> Result<(), IoError> {
    let payload = serde_json::to_string_pretty(score)?;
    fs::write(path, payload)?;
    Ok(())
}

pub fn load_notarium(path: impl AsRef<Path>) -> Result<Score, IoError> {
    let payload = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&payload)?)
}

pub fn export_musicxml(path: impl AsRef<Path>, score: &Score) -> Result<(), IoError> {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<score-partwise version=\"3.1\">\n");
    out.push_str("  <part-list>\n");

    for (i, staff) in score.staves.iter().enumerate() {
        out.push_str(&format!(
            "    <score-part id=\"P{}\"><part-name>{}</part-name></score-part>\n",
            i + 1,
            staff.name
        ));
    }
    out.push_str("  </part-list>\n");

    for (i, staff) in score.staves.iter().enumerate() {
        out.push_str(&format!("  <part id=\"P{}\">\n", i + 1));
        for measure in &staff.measures {
            out.push_str(&format!("    <measure number=\"{}\">\n", measure.number));
            for note in &measure.notes {
                out.push_str("      <note>\n");
                out.push_str("        <pitch>\n");
                out.push_str(&format!(
                    "          <step>{}</step>\n",
                    note.pitch.class.label()
                ));
                out.push_str(&format!(
                    "          <octave>{}</octave>\n",
                    note.pitch.octave
                ));
                out.push_str("        </pitch>\n");
                out.push_str(&format!(
                    "        <duration>{}</duration>\n",
                    (note.duration.beats() * 4.0) as i32
                ));
                out.push_str("      </note>\n");
            }
            out.push_str("    </measure>\n");
        }
        out.push_str("  </part>\n");
    }
    out.push_str("</score-partwise>\n");

    fs::write(path, out)?;
    Ok(())
}
