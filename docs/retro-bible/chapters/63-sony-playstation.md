# Sony PlayStation

The PlayStation changed the centre of gravity for console games.

This chapter is the skeleton for preserving PS1 discs, BIOS files, memory cards and the early 3D era on Linux.

## History

Sony entered the console market with a CD-based machine that made 3D games, full-motion video and a new publishing ecosystem feel normal.

## Hardware Overview

- CD-ROM console.
- Memory cards.
- Regioned BIOS.
- DualShock and analogue support later in the generation.

## Why It Mattered

The PlayStation carried games into a more mainstream, disc-based, 3D-aware era.

## Preservation Status

Excellent. Redump-style disc images, mature emulators and BIOS verification make a strong setup possible.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy | DuckStation or Mednafen |
| Practical standalone | DuckStation |
| Libretro workflow | Beetle PSX HW or PCSX ReARMed for light devices |

## BIOS Requirements

BIOS is strongly recommended. Region files should be named and checksummed carefully.

## Recommended File Formats

- BIN/CUE for source images.
- CHD for working library where supported.
- M3U playlists for multi-disc games.

## Folder Layout

```text
/mnt/games/roms/ps1/
/mnt/games/bios/ps1/
/mnt/games/saves/ps1/
```

## Enhancements

PGXP, upscaling and texture filtering can improve 3D games, but the original wobble and dithering are part of the period's look.

## Common Problems

- Missing BIOS.
- Broken CUE files.
- Multi-disc saves not shared.
- Analogue mode not enabled.

> **DAP Tip**
>
> Use M3U playlists for multi-disc PlayStation games so the frontend sees one game and the emulator can switch discs cleanly.

## DAP Gold Standard Setup

- Verified Redump source.
- CHD working copies.
- BIOS checksums recorded.
- Shared memory card policy documented.

## What Comes Next

Next comes PlayStation 2, where compatibility is strong but configuration choices start to matter more.
