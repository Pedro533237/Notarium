use crate::music::Pitch;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CppNote {
    pub midi: i32,
    pub duration_beats: f32,
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
struct CppEngineOpaque {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn cpp_engine_new() -> *mut CppEngineOpaque;
    fn cpp_engine_free(engine: *mut CppEngineOpaque);

    fn cpp_engine_clear(engine: *mut CppEngineOpaque);
    fn cpp_engine_add_note(engine: *mut CppEngineOpaque, midi: i32, duration_beats: f32);

    fn cpp_engine_note_count(engine: *const CppEngineOpaque) -> usize;
    fn cpp_engine_get_note(engine: *const CppEngineOpaque, index: usize) -> CppNote;

    fn cpp_engine_set_staff_geometry(engine: *mut CppEngineOpaque, width: f32, height: f32);
}

pub struct CppNotationEngine {
    ptr: *mut CppEngineOpaque,
}

impl CppNotationEngine {
    pub fn new() -> Self {
        let ptr = unsafe { cpp_engine_new() };
        Self { ptr }
    }

    pub fn clear(&mut self) {
        unsafe { cpp_engine_clear(self.ptr) }
    }

    pub fn add_note(&mut self, pitch: Pitch, duration_beats: f32) {
        unsafe { cpp_engine_add_note(self.ptr, pitch.midi_number(), duration_beats) }
    }

    pub fn set_staff_geometry(&mut self, width: f32, height: f32) {
        unsafe { cpp_engine_set_staff_geometry(self.ptr, width, height) }
    }

    pub fn note_count(&self) -> usize {
        unsafe { cpp_engine_note_count(self.ptr) }
    }

    pub fn note_at(&self, index: usize) -> Option<CppNote> {
        if index >= self.note_count() {
            return None;
        }
        Some(unsafe { cpp_engine_get_note(self.ptr, index) })
    }
}

impl Drop for CppNotationEngine {
    fn drop(&mut self) {
        unsafe { cpp_engine_free(self.ptr) }
    }
}
