# Notarium

Notarium é um editor de partituras em Rust com foco em Windows x64.

## O que já está implementado

- Tela de **Início** para criar nova partitura.
- Configuração inicial de partitura: nome, compositor, tonalidade, fórmula de compasso e tamanho de papel.
- Interface desktop com `egui` + `glium` (OpenGL puro) para edição.
- Inserção de notas (altura, oitava, duração e instrumento).
- Renderização básica de pauta e cabeças de nota.
- Playback com síntese digital em tempo real e presets de instrumentos orquestrais.
- Pipeline de CI em GitHub Actions para validar build, testes e gerar binário portable Windows x64.

## Requisitos de arquitetura

- O Notarium é suportado apenas em **Windows x64 (64-bit)**.

## Compatibilidade com PCs antigos (sem aceleração GPU)

- O app usa backend **OpenGL puro via `glium`** (sem WGPU/Vulkan).
- Foi ajustado para focar em compatibilidade com PCs antigos que suportam até OpenGL 3.3 / DirectX 10.1.
- Em falhas de inicialização/execução, o app grava `notarium.log` ao lado do executável e mostra um pop-up de erro no Windows.

## Windows portable via GitHub Actions

O workflow gera o artefato **`notarium-windows-portable`** com um arquivo `notarium-windows-portable.zip` (binário x64).

Passos:
1. Abra a aba **Actions** no GitHub.
2. Entre em uma execução de workflow com status verde.
3. Baixe o artefato `notarium-windows-portable`.
4. Extraia o `.zip`.
5. Rode `notarium.exe` (sem instalador, estilo portable).

## Sobre alerta do Windows Defender / SmartScreen

Não é possível eliminar 100% dos alertas sem **assinatura digital de código** (certificado EV/OV).

Redução prática de alertas:
- assinar o executável em release com certificado de código;
- manter distribuição consistente (mesmo nome/hash por release oficial);
- publicar releases estáveis e usar reputação de download.

## Melhorias de velocidade de compilação

- CI com cache (`Swatinem/rust-cache`) para reduzir tempo em builds repetidos.
- Registro crates.io em modo `sparse` no workflow.
- Dependências com features reduzidas para evitar compilar backends/decoders desnecessários.

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
