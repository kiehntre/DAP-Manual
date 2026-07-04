# Sega Dreamcast

The Dreamcast feels like the future arriving slightly too early.

This chapter is the skeleton for preserving Dreamcast games, GD-ROM images, VMU saves, online history and emulator choices on Linux.

## History

Sega's final console brought arcade-quality games, online features and a creative library that still feels fresh. Its commercial life was short, but its influence lasted.

## Hardware Overview

- GD-ROM media.
- VMU memory units.
- Built-in modem in many regions.
- Strong arcade hardware relationship through Naomi.

## Why It Mattered

The Dreamcast bridged arcade culture, online console play and the sixth generation. It also became a symbol of what can be lost when a platform ends too soon.

## Preservation Status

Good, but GD-ROM formats, CDI rips and online-service restoration need careful separation.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Practical standalone | Flycast |
| Libretro workflow | Flycast core |
| Research alternatives | redream where appropriate |

## BIOS Requirements

BIOS and flash files are commonly used. Region and boot behaviour should be documented.

## Recommended File Formats

- GDI for source-style disc images.
- CHD for working copies where supported.
- Avoid treating CDI rips as equivalent to clean GD-ROM dumps.

## Folder Layout

```text
/mnt/games/roms/dreamcast/
/mnt/games/bios/dreamcast/
/mnt/games/saves/dreamcast/
/mnt/games/online/dreamcast/
```

## Enhancements

Widescreen, upscaling and texture filtering can work well, but some games reveal HUD or geometry issues.

## Common Problems

- BIOS missing.
- CDI and GDI mixed without notes.
- VMU files overwritten.
- Online setup treated as normal launch setup.

> **DAP Warning**
>
> Do not treat selfboot CDI images as archival equals to proper GD-ROM preservation dumps.

## DAP Gold Standard Setup

- GDI or verified source retained.
- CHD working library where supported.
- BIOS and flash files verified.
- VMU saves backed up.

## What Comes Next

Next comes Game Gear, Sega's handheld branch of the 8-bit family.
