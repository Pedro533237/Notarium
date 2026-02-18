# Stack musical recomendado para o Notarium

Este documento traduz o direcionamento do produto para um caminho técnico realista no ecossistema Rust.

## Backend de música e teoria

- `rust-music-theory`: base para operações de notas, intervalos, acordes e escalas.
- `music_note`: conversões MIDI e utilitários rápidos de nota (útil para import/export e debug).
- `tunes`: playback/síntese em tempo real e padrões musicais para pré-escuta.
- `rust-music` (paveyry): camada de estrutura para obras grandes com múltiplos instrumentos e exportação MIDI.

## Fluxo sugerido

1. **Modelo interno (`music.rs`)**
   - continuar sendo a fonte de verdade do editor.
2. **Camada de teoria**
   - adaptar eventos `NoteEvent` para tipos da `rust-music-theory`.
3. **Camada de playback**
   - usar `tunes` + `cpal` para baixa latência em reprodução.
4. **Interop e export**
   - usar `music_note` e `rust-music` para MIDI/arranjos mais complexos.

## Renderização de partitura

- Curto prazo: evolução da renderização atual em `notation.rs`.
- Médio prazo: gerar SVG com pipeline dedicado para preview e export.
- Longo prazo: engine de layout com colisão, espaçamento rítmico e tipografia de edição profissional.

## Diretriz visual

A UI deve seguir estética profissional estilo Sibelius/MuseScore:

- dark theme sofisticado;
- navegação rica (top tabs + sidebar);
- cartões visuais para projetos recentes;
- feedback claro para ações de criar/abrir/salvar.
