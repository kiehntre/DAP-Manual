# Nintendo Game Boy Color

The Game Boy Color is a bridge: part old handheld, part new platform.

This chapter is the skeleton for preserving GBC games and handling the line between enhanced Game Boy titles and true colour-only releases.

## History

The Game Boy Color extended Nintendo's handheld life at exactly the right time. It kept compatibility while giving developers colour and a little more room.

## Hardware Overview

- Colour handheld.
- Backwards compatible with Game Boy.
- Cartridge media.
- Link cable support.

## Why It Mattered

It gave the Game Boy family a second youth and prepared the ground for the Game Boy Advance.

## Preservation Status

Excellent. The main care points are mode selection and library separation.

## Recommended Emulator

SameBoy, Gambatte or mGBA.

## BIOS Requirements

Optional boot ROM support can improve accuracy and presentation.

## Recommended File Formats

- `.gbc` for colour titles.
- `.gb` enhanced titles should be labelled where needed.

## Folder Layout

```text
/mnt/games/roms/gbc/
/mnt/games/saves/gbc/
```

## Enhancements

LCD shaders and colour-correct palettes matter more than resolution tricks.

## Common Problems

- Enhanced GB games filed inconsistently.
- Wrong colour palette.
- Saves not migrated from Game Boy emulator cores.

> **DAP Warning**
>
> Do not let frontend artwork hide compatibility details. Some games are colour-enhanced, some require GBC, and the distinction matters.

## DAP Gold Standard Setup

- Verified ROMs.
- GBC and GB policy documented.
- Same emulator family used where practical.
- Saves backed up.

## What Comes Next

Next comes Game Boy Advance, a handheld with a huge library and excellent emulator support.
