# Notarium

Notarium é um editor de notação musical minimalista em Rust (edition 2021), inspirado em Sibelius/MuseScore, com arquitetura modular pronta para expansão.

## Arquitetura

O projeto foi dividido em crates internas:

- `notarium-core`: teoria musical + modelo de partitura + operações de edição.
- `notarium-render`: layout/render vetorial de partitura com `egui` shapes.
- `notarium-playback`: playback básico com `cpal` + metrônomo.
- `notarium-io`: salvar/carregar `.notarium` (JSON) e exportar `.musicxml`.
- `notarium` (app): janela principal (`eframe`) e integração de UI.

Mais detalhes em [`docs/architecture.md`](docs/architecture.md).

## Funcionalidades MVP

- Inserção de notas por mouse (caneta).
- Entrada por teclado do PC (A–G, Shift = sustenido, Alt = bemol, setas para oitava).
- Ferramentas: seleção, borracha e caneta.
- Zoom + rolagem da partitura.
- Menu File: New / Open / Save / Export MusicXML.
- Painel lateral de instrumentos.
- Barra inferior de transporte: Play / Stop / BPM / Compasso.
- Renderização de pentagrama vazio (ou com notas) já na abertura.

## Executar

```bash
cargo run
```

## Checks

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Exemplo

Veja `examples/create_score.rs`.
