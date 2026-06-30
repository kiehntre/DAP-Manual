# Xemu

## Overview

Xemu emulates the original Xbox.

## Core checks

```bash
flatpak run app.xemu.xemu
```

## DAP Notes

- SDL audio may route through PipeWire.
- Direct PipeWire can be worth testing in custom builds.
- DSP JIT may help audio-heavy games.
