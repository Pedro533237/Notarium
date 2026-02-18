use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct ScoreFile {
    pub name: String,
    pub path: PathBuf,
    pub last_modified: SystemTime,
}

#[derive(Debug, Clone)]
pub struct HomeScreenState {
    pub score_files: Vec<ScoreFile>,
    pub empty_message: String,
}

impl HomeScreenState {
    pub fn load_from_documents() -> Self {
        match scan_notarium_files() {
            Ok(files) if !files.is_empty() => Self {
                score_files: files,
                empty_message: String::new(),
            },
            Ok(_) => Self {
                score_files: Vec::new(),
                empty_message: "Nenhuma partitura encontrada.".to_owned(),
            },
            Err(_) => Self {
                score_files: Vec::new(),
                empty_message: "Nenhuma partitura encontrada.".to_owned(),
            },
        }
    }
}

pub fn notarium_documents_dir() -> PathBuf {
    PathBuf::from("/documents/notarium/")
}

fn scan_notarium_files() -> std::io::Result<Vec<ScoreFile>> {
    let dir = notarium_documents_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let item = entry?;
        let path = item.path();
        if path.extension().and_then(|value| value.to_str()) != Some("ntr") {
            continue;
        }

        let metadata = item.metadata()?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .map_or_else(|| "partitura.ntr".to_owned(), ToOwned::to_owned);

        files.push(ScoreFile {
            name,
            path,
            last_modified: modified,
        });
    }

    files.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(files)
}
