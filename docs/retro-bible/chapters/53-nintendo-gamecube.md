# Nintendo GameCube

The GameCube is one of the nicest systems to emulate on Linux when the library is organised properly.

This chapter is the skeleton for using Dolphin, RVZ files, controller profiles and enhancements without losing the clean preservation source.

## History

Nintendo's small purple cube arrived with mini-DVD media, fast local multiplayer and a library that has aged with real affection.

## Hardware Overview

- Optical disc console.
- GameCube controller and memory cards.
- Progressive scan support in many titles.
- Link features with Game Boy Advance.

## Why It Mattered

The GameCube library bridges late arcade-style design and more modern 3D console structure. It is also a showcase for how good mature emulation can feel.

## Preservation Status

Excellent. Redump-style source images and Dolphin's RVZ format make a strong preservation-plus-playability workflow.

## Recommended Emulator

Dolphin is the standard recommendation.

## BIOS Requirements

No BIOS is required for most Dolphin use, though IPL files can be used for boot animation and some authentic behaviours.

## Recommended File Formats

- Keep verified source dumps separately.
- Use `.rvz` for playable Dolphin libraries.
- Avoid old compressed formats unless needed for compatibility.

## Folder Layout

```text
/mnt/games/roms/gamecube/
/mnt/games/verified/gamecube/
/mnt/games/saves/gamecube/
/mnt/games/texture-packs/gamecube/
```

## Enhancements

Internal resolution, widescreen patches and texture packs are all useful. Test per game, especially HUDs and effects.

## Common Problems

- Wrong controller profile.
- Memory card path confusion.
- RVZ generated from unverified source.
- Widescreen hack breaking effects.

> **DAP Tip**
>
> Keep Dolphin's RVZ files as the daily drivers, but do not throw away the verified source disc images.

## DAP Gold Standard Setup

- Verified source stored.
- RVZ working library.
- Dolphin config backed up.
- GameCube controller or adapter profile documented.

## What Comes Next

Next comes Wii, where Dolphin remains central but motion controls, NAND data and WiiWare add more moving parts.
