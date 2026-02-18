use crate::core::project::{KeySignature, ProjectError, TimeSignature};
use crate::instruments::registry::InstrumentRegistry;
use crate::ui::editor_screen::{EditorState, Score};
use crate::ui::home_screen::{notarium_documents_dir, HomeScreenState};
use crate::ui::new_score_screen::{NewScoreData, NewScoreUiState};
use crate::ui::theme::NotariumTheme;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    NewScoreStep1,
    NewScoreStep2,
    Editor,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_screen: AppScreen,
}

#[derive(Debug)]
pub struct NotariumUiApp {
    pub theme: NotariumTheme,
    pub state: AppState,
    pub home: HomeScreenState,
    pub new_score_data: NewScoreData,
    pub new_score_ui: NewScoreUiState,
    pub editor: Option<EditorState>,
}

#[derive(Debug)]
pub enum UiAppError {
    Project(ProjectError),
    Io(std::io::Error),
}

impl Display for UiAppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project(error) => write!(f, "erro de projeto: {error}"),
            Self::Io(error) => write!(f, "erro de IO: {error}"),
        }
    }
}

impl std::error::Error for UiAppError {}

impl From<ProjectError> for UiAppError {
    fn from(value: ProjectError) -> Self {
        Self::Project(value)
    }
}

impl From<std::io::Error> for UiAppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl NotariumUiApp {
    pub fn new(registry: &InstrumentRegistry) -> Result<Self, UiAppError> {
        Ok(Self {
            theme: NotariumTheme::noturno_vermelho(),
            state: AppState {
                current_screen: AppScreen::Home,
            },
            home: HomeScreenState::load_from_documents(),
            new_score_data: NewScoreData::default_with_time_signature()?,
            new_score_ui: NewScoreUiState::from_registry(registry),
            editor: None,
        })
    }

    pub fn go_to_new_score_step1(&mut self) {
        self.state.current_screen = AppScreen::NewScoreStep1;
    }

    pub fn go_to_new_score_step2(&mut self) {
        self.state.current_screen = AppScreen::NewScoreStep2;
    }

    pub fn back_to_step1(&mut self) {
        self.state.current_screen = AppScreen::NewScoreStep1;
    }

    pub fn back_to_home(&mut self) {
        self.home = HomeScreenState::load_from_documents();
        self.state.current_screen = AppScreen::Home;
    }

    pub fn open_file_from_home(&mut self, index: usize) {
        if let Some(score_file) = self.home.score_files.get(index) {
            let score = Score {
                title: score_file.name.clone(),
                instruments: self.new_score_data.instruments.clone(),
                tempo_bpm: self.new_score_data.tempo_bpm,
                key_signature: self.new_score_data.key_signature,
                time_signature: self.new_score_data.time_signature,
            };
            self.editor = Some(EditorState::new(score));
            self.state.current_screen = AppScreen::Editor;
        }
    }

    pub fn conclude_new_score(&mut self) -> Result<(), UiAppError> {
        if self.new_score_data.instruments.is_empty() {
            if let Some(default_instrument) = self.new_score_ui.available.first().cloned() {
                self.new_score_data.instruments.push(default_instrument);
            }
        }

        let score = Score {
            title: self.new_score_data.title.clone(),
            instruments: self.new_score_data.instruments.clone(),
            tempo_bpm: self.new_score_data.tempo_bpm,
            key_signature: self.new_score_data.key_signature,
            time_signature: self.new_score_data.time_signature,
        };

        save_score_file(&score)?;
        self.editor = Some(EditorState::new(score));
        self.state.current_screen = AppScreen::Editor;
        Ok(())
    }

    pub fn set_music_settings(
        &mut self,
        title: String,
        bpm: u32,
        key_signature: KeySignature,
        time_signature: TimeSignature,
    ) {
        self.new_score_data.title = title;
        self.new_score_data.tempo_bpm = bpm.clamp(20, 300);
        self.new_score_data.key_signature = key_signature;
        self.new_score_data.time_signature = time_signature;
    }

    pub fn set_default_music_settings(&mut self) -> Result<(), UiAppError> {
        self.set_music_settings(
            "Nova Partitura".to_owned(),
            120,
            KeySignature::C,
            TimeSignature::new(4, 4)?,
        );
        Ok(())
    }
}

fn save_score_file(score: &Score) -> Result<(), UiAppError> {
    let dir = notarium_documents_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    let filename = sanitize_filename(&score.title);
    let path = dir.join(format!("{filename}.ntr"));

    let mut data = String::new();
    data.push_str("NOTARIUM_SCORE_V1\n");
    data.push_str(&format!("title={}\n", score.title));
    data.push_str(&format!("tempo_bpm={}\n", score.tempo_bpm));
    data.push_str(&format!("instrument_count={}\n", score.instruments.len()));
    fs::write(path, data)?;

    Ok(())
}

fn sanitize_filename(value: &str) -> String {
    let mut output = value
        .chars()
        .filter(|ch| ch.is_alphanumeric() || *ch == '-' || *ch == '_')
        .collect::<String>();
    if output.is_empty() {
        output = "partitura".to_owned();
    }
    output
}
