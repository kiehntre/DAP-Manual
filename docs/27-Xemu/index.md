# Xemu

## Overview

Xemu emulates the original Xbox.

It is useful for Xbox titles that do not have good PC ports or modern compatibility.

## Core Requirements

- MCPX boot ROM
- BIOS image
- Xbox HDD image
- Game ISO/XISO
- Working GPU acceleration

## Basic Checks

```bash
flatpak run app.xemu.xemu
```

For native builds:

```bash
xemu --version
```

## Performance Areas

- Renderer backend
- GPU driver
- CPU performance
- DSP/audio emulation
- Storage path
- Game compatibility

## Audio Problems

Audio dropouts usually come from emulation overhead, backend issues, or host scheduling.

Check:

```bash
pactl info
pw-top
nvidia-smi
top
```

## DAP Notes

- SDL audio may route through PipeWire underneath.
- Direct PipeWire backend can be worth testing if compiled in.
- DSP JIT can help games with heavy audio processing.
- Vulkan/OpenGL behaviour depends on the build and driver stack.

## Troubleshooting

| Problem | First Move |
|---|---|
| Game slow | Check renderer, CPU load and resolution scale |
| Audio drops | Check DSP JIT, audio backend, host load |
| No boot | Check BIOS/HDD paths |
| Controller issue | Check SDL/gamepad mapping |
