use crate::music::NoteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditMode {
    None,
    NoteSelected(NoteId),
}

impl Default for EditMode {
    fn default() -> Self {
        Self::None
    }
}
