# Nintendo 64

The Nintendo 64 is easy to love and surprisingly awkward to emulate well.

This chapter is the skeleton for handling N64 games with realistic expectations around accuracy, controllers, texture packs and widescreen.

## History

Nintendo stayed with cartridges while much of the industry moved to discs. The result was a machine with fast loading, unusual constraints and some of the most influential 3D games ever made.

## Hardware Overview

- 64-bit era console.
- Cartridge media.
- Analogue controller with unique layout.
- Expansion Pak for selected games.

## Why It Mattered

The N64 defined console 3D movement for a generation. Its controller and camera design are part of that history, for better and worse.

## Preservation Status

Strong, but emulator behaviour varies. Graphics accuracy, timing and controller mapping are the big practical concerns.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy-focused | ares |
| Practical standalone | simple64 or RMG |
| RetroArch workflows | ParaLLEl N64 or Mupen64Plus-Next |

## BIOS Requirements

No normal BIOS is required for standard N64 emulation.

## Recommended File Formats

- `.z64` is preferred for byte-swapped clarity.
- Avoid leaving N64 games buried in unsupported archives.

## Folder Layout

```text
/mnt/games/roms/n64/
/mnt/games/texture-packs/n64/
/mnt/games/saves/n64/
```

## Enhancements

HD texture packs and widescreen patches can be excellent, but they are per-game decisions. Controller profiles need special care because the original pad is not shaped like modern controllers.

## Common Problems

- Wrong analogue sensitivity.
- Glide or plugin differences.
- Texture pack folder mismatch.
- Expansion Pak setting wrong.
- PAL and NTSC timing confusion.

> **DAP Tip**
>
> Treat N64 controller mapping as part of the game profile. One comfortable global layout rarely fits the whole library.

## DAP Gold Standard Setup

- `.z64` verified library.
- Emulator choice recorded.
- Per-game controller notes for awkward titles.
- Texture packs versioned and optional.

## What Comes Next

Next comes GameCube, where Dolphin gives us one of emulation's great success stories and a cleaner disc-format path.
