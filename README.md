# Notarium

Notarium é um editor de partituras em Rust com foco em Linux e Windows.

## O que já está implementado

- Tela de **Início** para criar nova partitura.
- Configuração inicial de partitura: nome, compositor, tonalidade, fórmula de compasso e tamanho de papel.
- Interface desktop com `egui` para edição.
- Inserção de notas (altura, oitava, duração e instrumento).
- Renderização básica de pauta e cabeças de nota.
- Playback com síntese digital em tempo real e presets de instrumentos orquestrais.
- Pipeline de CI em GitHub Actions para validar build, testes e gerar binários.

## Windows portable via GitHub Actions

O workflow gera o artefato **`notarium-windows-portable`** com um arquivo `notarium-windows-portable.zip`.

Passos:
1. Abra a aba **Actions** no GitHub.
2. Entre em uma execução de workflow com status verde.
3. Baixe o artefato `notarium-windows-portable`.
4. Extraia o `.zip`.
5. Rode `notarium.exe` (sem instalador, estilo portable).

## Limites atuais

Este repositório é um **MVP técnico**. Ainda não cobre 100% da notação completa de ferramentas como Sibelius/MuseScore (articulações avançadas, layout editorial completo, VST, MusicXML completo, etc.).

## Como executar

```bash
cargo run
```

## Como testar

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```
