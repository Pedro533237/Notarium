# Notarium

Notarium é uma base de arquitetura em Rust para editor profissional de notação musical com foco em **compatibilidade com PCs antigos (2010)** e **pipeline CPU-only**.

## Diretrizes implementadas

- Renderização e fluxo principal orientados a fallback de software (sem dependência de OpenGL moderno/Vulkan).
- Tema padrão **Noturno** com paleta vermelho escuro e estrutura pronta para tema claro.
- Tela inicial profissional com metadados completos do projeto, template e seleção de instrumentos.
- Banco de instrumentos expansível por JSON externo (sem recompilar).
- Núcleo de playback com `ExpressionEngine` (dinâmica, articulação e humanização).
- Arquitetura em módulos:

```
/core
/ui
/audio
/plugins
/instruments
/playback
/engraving
/export
/theme
```

## Executar

```bash
cargo run
```

## Validar

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
