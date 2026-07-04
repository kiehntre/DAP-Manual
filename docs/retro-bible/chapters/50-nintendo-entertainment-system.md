# Nintendo Entertainment System

The NES is one of the great reset buttons in games history.

This chapter is the skeleton for preserving and playing NES games on Linux with clean ROMs, sensible emulator choices and enough respect for mappers to avoid easy mistakes.

## History

The Famicom arrived in Japan before the NES helped rebuild the North American console market. Its library became a shared language for platformers, action games, RPGs and arcade conversions.

## Hardware Overview

- 8-bit CPU family.
- Cartridge media.
- Mapper chips used to extend cartridge behaviour.
- Regional differences between NTSC and PAL releases.

## Why It Mattered

The NES set expectations for home console libraries, third-party publishing and long-running franchises.

## Preservation Status

NES preservation is mature, but header formats and mapper support still matter. A file can be mostly right and still have metadata issues.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy | Mesen |
| Frontend/libretro workflows | Mesen or FCEUmm core |
| Lightweight play | Nestopia UE |

## BIOS Requirements

No normal BIOS is required for standard NES emulation.

## Recommended File Formats

- `.nes` for normal cartridge images.
- Keep verified No-Intro-style sets separate from hacks and translations.

## Folder Layout

```text
/mnt/games/roms/nes/
/mnt/games/patches/nes/
/mnt/games/saves/nes/
```

## Enhancements

Use CRT shaders gently. Widescreen is not a normal NES expectation, though individual hacks may exist.

## Common Problems

- Bad or obsolete headers.
- PAL and NTSC speed mismatch.
- Hacks mixed into verified sets.
- Controller turbo behaviour left unexplained.

> **DAP Tip**
>
> For NES, a clean verified ROM and a good shader do more than a pile of random enhancement toggles.

## DAP Gold Standard Setup

- Verified ROM set.
- Mesen tested as the accuracy reference.
- Hacks and translations stored separately.
- Save folder backed up.

## What Comes Next

Next comes the SNES, where cartridge preservation is still friendly but enhancement chips and video behaviour deserve more attention.
