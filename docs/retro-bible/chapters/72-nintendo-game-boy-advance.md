# Nintendo Game Boy Advance

The Game Boy Advance feels like a portable SNES until it suddenly becomes something else entirely.

This chapter is the skeleton for GBA preservation, BIOS handling, saves and the library's mix of ports, originals and experiments.

## History

The GBA brought 32-bit handheld hardware to Nintendo's portable line. It inherited old design instincts while opening the door to faster, richer games.

## Hardware Overview

- 32-bit handheld.
- Cartridge media.
- Link cable and wireless adapter support.
- Backwards compatibility on original hardware.

## Why It Mattered

It hosted some of the best portable games ever made and kept 2D design alive during a 3D-heavy console era.

## Preservation Status

Excellent. BIOS, save type and colour correction are the main practical concerns.

## Recommended Emulator

mGBA is the standard recommendation for most Linux users.

## BIOS Requirements

GBA BIOS is optional in some emulators but recommended for accuracy and boot behaviour.

## Recommended File Formats

- `.gba` ROM images.
- Keep patches and translations separate.

## Folder Layout

```text
/mnt/games/roms/gba/
/mnt/games/bios/gba/
/mnt/games/saves/gba/
```

## Enhancements

Colour correction, LCD shaders and careful scaling matter. Avoid over-sharpening.

## Common Problems

- Missing BIOS for games that expect it.
- Save type mismatch in old emulators.
- Washed-out or oversaturated colours.
- ROM hacks patched against wrong revisions.

> **DAP Tip**
>
> GBA deserves colour-correction testing. The screen it was designed for was not your modern OLED or LCD monitor.

## DAP Gold Standard Setup

- mGBA configured.
- BIOS checksum recorded if used.
- Verified ROMs.
- Saves backed up.

## What Comes Next

Next comes Nintendo DS, where two screens and touch input change the whole frontend problem.
