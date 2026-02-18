use crate::music::NoteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditMode {
    #[default]
    None,
    NoteSelected(NoteId),
}
