use crate::common::id::Id;
use crate::core::project::{KeySignature, ProjectMetadata, ScoreProject, TimeSignature};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectTemplate {
    Orchestra,
    StringQuartet,
    Band,
    Choir,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioConfigSelection {
    pub sample_rate: u32,
    pub buffer_size: u16,
    pub low_performance_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartScreenState {
    pub title: String,
    pub subtitle: String,
    pub composer: String,
    pub arranger: String,
    pub copyright: String,
    pub key_signature: KeySignature,
    pub time_signature: TimeSignature,
    pub bpm: u16,
    pub selected_instruments: Vec<Id>,
    pub template: ProjectTemplate,
    pub audio_config: AudioConfigSelection,
}

impl Default for StartScreenState {
    fn default() -> Self {
        Self {
            title: "Novo Projeto".to_string(),
            subtitle: String::new(),
            composer: String::new(),
            arranger: String::new(),
            copyright: String::new(),
            key_signature: KeySignature::C,
            time_signature: TimeSignature::common_time(),
            bpm: 120,
            selected_instruments: Vec::new(),
            template: ProjectTemplate::Empty,
            audio_config: AudioConfigSelection {
                sample_rate: 44_100,
                buffer_size: 256,
                low_performance_mode: true,
            },
        }
    }
}

impl StartScreenState {
    pub fn create_project(self) -> ScoreProject {
        let metadata = ProjectMetadata {
            title: self.title,
            subtitle: self.subtitle,
            composer: self.composer,
            arranger: self.arranger,
            copyright: self.copyright,
        };

        let mut project = ScoreProject::new(metadata);
        project.key_signature = self.key_signature;
        project.time_signature = self.time_signature;
        project.bpm = self.bpm;
        project.instrument_ids = self.selected_instruments;
        project
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_project_from_start_screen() {
        let state = StartScreenState {
            title: "Suite".to_string(),
            bpm: 96,
            ..Default::default()
        };
        let project = state.create_project();
        assert_eq!(project.metadata.title, "Suite");
        assert_eq!(project.bpm, 96);
    }
}
