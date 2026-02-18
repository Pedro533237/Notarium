# Notarium

Notarium é um editor de partituras em Rust com foco em Windows x64, com arquitetura modular para evolução de notação profissional.

## Novidades estruturais (Staff System + Edição)

- Novo núcleo musical modular em `src/music/` (`staff`, `measure`, `clef`, `duration`, `note`).
- Sistema de pautas com 5 linhas, múltiplas pautas por sistema, clave (Sol/Fá/Dó), armadura de clave e fórmula de compasso.
- Barras de compasso automáticas por staff.
- Coordenadas relativas em **staff space units (SSU)** para separação de lógica musical e layout.
- `NoteDuration` expandido: semibreve, mínima, semínima, colcheia, semicolcheia, fusa e semifusa.
- `Note` com seleção visual (`selected`, `opacity`) e suporte a `velocity`, pontuação e ligadura.
- Seleção estilo MuseScore: click com hitbox por nota, highlight azul, opacidade reduzida e modo de edição único (`EditMode::NoteSelected`).
- Atalhos de edição:
  - `1..7` altera duração
  - `↑/↓` altera pitch
  - `Delete` remove nota selecionada
- Renderização em camadas (`Staff`, `Notes`, `SelectionOverlay`) com cache de glyphs e batching lógico.

## Compatibilidade OpenGL

O renderer foi preparado para perfis legados:

- OpenGL 2.1
- OpenGL 2.0
- OpenGL ES 2.0

Fallback automático de contexto na inicialização (ordem): **OpenGL 2.0 → OpenGL 2.1 → OpenGL ES 2.0 → padrão do driver**.

## Estrutura de código

```text
src/
 ├── music/
 │   ├── duration.rs
 │   ├── measure.rs
 │   ├── note.rs
 │   ├── staff.rs
 │   └── mod.rs
 ├── render/
 │   ├── renderer.rs
 │   ├── glyph_cache.rs
 │   ├── staff_renderer.rs
 │   ├── note_renderer.rs
 │   └── mod.rs
 ├── input/
 │   ├── mouse.rs
 │   ├── keyboard.rs
 │   └── mod.rs
 ├── editor/
 │   ├── selection.rs
 │   ├── edit_mode.rs
 │   └── mod.rs
 ├── audio.rs
 └── main.rs
```

## Como executar (Windows x64)

```bash
cargo run --release
```

## Build para Windows

Use o script:

```bat
scripts\build_windows.bat
```

Ele compila em release para `x86_64-pc-windows-msvc`.

## Checks recomendados

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo check --target x86_64-pc-windows-msvc
```
