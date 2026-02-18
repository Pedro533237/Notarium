#include "notation_engine.hpp"

#include <vector>

struct CppEngine {
    std::vector<CppNote> notes;
    float staff_width = 760.0f;
    float staff_height = 40.0f;

    void relayout() {
        if (notes.empty()) return;

        float total_beats = 0.0f;
        for (const auto& n : notes) {
            total_beats += (n.duration_beats > 0.0f ? n.duration_beats : 0.25f);
        }
        if (total_beats <= 0.0f) total_beats = 1.0f;

        float running = 0.0f;
        for (auto& n : notes) {
            const float beats = (n.duration_beats > 0.0f ? n.duration_beats : 0.25f);
            n.x = 12.0f + (running / total_beats) * (staff_width - 24.0f);
            running += beats;

            // E4 as center-line reference (MIDI 64)
            n.y = (staff_height * 0.5f) - ((static_cast<float>(n.midi - 64)) * (staff_height / 16.0f));
        }
    }
};

extern "C" {

CppEngine* cpp_engine_new() {
    return new CppEngine();
}

void cpp_engine_free(CppEngine* engine) {
    delete engine;
}

void cpp_engine_clear(CppEngine* engine) {
    if (!engine) return;
    engine->notes.clear();
}

void cpp_engine_add_note(CppEngine* engine, int32_t midi, float duration_beats) {
    if (!engine) return;
    CppNote note{};
    note.midi = midi;
    note.duration_beats = duration_beats;
    note.x = 0.0f;
    note.y = 0.0f;
    engine->notes.push_back(note);
    engine->relayout();
}

std::size_t cpp_engine_note_count(const CppEngine* engine) {
    if (!engine) return 0;
    return engine->notes.size();
}

CppNote cpp_engine_get_note(const CppEngine* engine, std::size_t index) {
    if (!engine || index >= engine->notes.size()) {
        return CppNote{0, 0.0f, 0.0f, 0.0f};
    }
    return engine->notes[index];
}

void cpp_engine_set_staff_geometry(CppEngine* engine, float width, float height) {
    if (!engine) return;
    engine->staff_width = width;
    engine->staff_height = height;
    engine->relayout();
}

} // extern "C"
