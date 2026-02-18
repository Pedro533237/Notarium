#pragma once

#include <cstddef>
#include <cstdint>

extern "C" {

struct CppNote {
    int32_t midi;
    float duration_beats;
    float x;
    float y;
};

struct CppEngine;

CppEngine* cpp_engine_new();
void cpp_engine_free(CppEngine* engine);

void cpp_engine_clear(CppEngine* engine);
void cpp_engine_add_note(CppEngine* engine, int32_t midi, float duration_beats);

std::size_t cpp_engine_note_count(const CppEngine* engine);
CppNote cpp_engine_get_note(const CppEngine* engine, std::size_t index);

void cpp_engine_set_staff_geometry(CppEngine* engine, float width, float height);

}
