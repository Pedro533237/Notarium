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

## Gerar EXE (Windows)

```bash
cargo build --release
```

Saída esperada:

```text
target/release/notarium.exe
```

## Rodar EXE gerado (Windows)

```powershell
.\target\release\notarium.exe
```

O app abre uma interface nativa do Windows (sem depender só do prompt de comando).

## Validar no ambiente de desenvolvimento

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```


## Tamanho do executável

Em builds sem dependências pesadas de GUI/áudio, o `notarium.exe` pode ficar pequeno (ex.: ~273 KB) e isso é esperado.
O comportamento correto é: abrir janela nativa no Windows ao executar o `.exe`.
