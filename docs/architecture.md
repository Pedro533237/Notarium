# Arquitetura do Notarium

## Objetivo
Separar claramente núcleo musical e interface para facilitar evolução para recursos avançados (layout editorial, import MIDI/MusicXML completo, VST etc).

## Crates

### 1) `notarium-core`
- Tipos de teoria musical: nota, altura, acidente, oitava, compasso, armadura.
- Modelo de domínio: `Score`, `Staff`, `Measure`, `Note`.
- Operações: inserir, mover, deletar e editar duração de nota.

### 2) `notarium-render`
- Renderização vetorial via `egui::Painter`.
- Layout de pautas e compassos com suporte a zoom.
- `hit_test` para converter clique do mouse em posição musical.

### 3) `notarium-playback`
- Engine simples com `cpal`.
- Agenda notas e gera onda sintética por instrumento.
- Metrônomo embutido no áudio.

### 4) `notarium-io`
- Persistência em `.notarium` (JSON).
- Exportador `.musicxml` simplificado.

### 5) `notarium` (aplicativo)
- Janela principal com `eframe/egui`.
- Layout de UI (menu, painel lateral, área central, transporte).
- Estado global e orquestração dos módulos.

## Expansões futuras sugeridas
- Múltiplas vozes por pauta com regras de colisão de haste.
- Quantização visual e layout de ligaduras/articulações.
- Importador MusicXML robusto e MIDI file.
- Playback por backend MIDI real (midir).
