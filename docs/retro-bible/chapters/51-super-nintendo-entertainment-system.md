# Super Nintendo Entertainment System

The SNES is warm, colourful and technically stranger than it first appears.

This chapter is the skeleton for preserving SNES games while respecting enhancement chips, regional timing and the long shadow of fan translations and ROM hacks.

## History

Nintendo's 16-bit console arrived into a fierce generation. It answered the Mega Drive with colour, sound and a library full of platformers, RPGs and ambitious cartridge hardware.

## Hardware Overview

- 16-bit console architecture.
- Cartridge media.
- Enhancement chips such as Super FX, SA-1 and DSP variants.
- NTSC and PAL timing differences.

## Why It Mattered

The SNES helped define cinematic RPGs, mascot platformers and console experimentation. It is also a major home for fan translation history.

## Preservation Status

Very strong. The main risks are modified ROMs, bad headers and hacks being mixed into clean sets.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy | bsnes or ares |
| Practical frontend use | Snes9x or bsnes core |
| Research and edge cases | bsnes |

## BIOS Requirements

No normal BIOS is required for standard SNES emulation. Special add-ons may need separate handling.

## Recommended File Formats

- `.sfc` preferred for clean cartridge images.
- `.smc` still appears, often historically.
- Keep patches as patches where possible.

## Folder Layout

```text
/mnt/games/roms/snes/
/mnt/games/patches/snes/
/mnt/games/saves/snes/
```

## Enhancements

CRT shaders suit many SNES games. MSU-1 hacks, translations and improvement patches belong in clearly labelled folders.

## Common Problems

- Headered versus unheadered patch confusion.
- PAL games running differently from expected.
- Enhancement-chip games tested only on light cores.
- Save files lost during core migration.

> **DAP Warning**
>
> Many SNES patches require an exact source ROM. If the checksum does not match, stop and find out why before patching.

## DAP Gold Standard Setup

- Verified `.sfc` library.
- Translations separated from originals.
- Patch source checksums recorded.
- Emulator choice recorded per frontend.

## What Comes Next

Next comes Nintendo 64, where accuracy, controller feel and graphics plugins become much more visible.
