# ScummVM

ScummVM is one of preservation's great practical gifts: it lets many classic adventure games run cleanly without emulating an entire old computer.

This chapter is the skeleton for using ScummVM in a Linux-first library while preserving original data files and notes.

## History

ScummVM began around LucasArts adventure games and grew into a broad engine reimplementation project. It preserves playability by recreating game engines rather than whole machines.

## Hardware Overview

ScummVM does not emulate one hardware platform. It runs supported game engines using original data files.

## Why It Mattered

It makes many important adventure games more accessible, portable and maintainable than raw DOS or Windows setups.

## Preservation Status

Excellent for supported games. Unsupported games still need their original platform workflows.

## Recommended Emulator

ScummVM itself is the recommended tool.

## BIOS Requirements

None.

## Recommended File Formats

- Original game data folders.
- Disc images retained separately where owned.
- Keep audio tracks and speech files intact.

## Folder Layout

```text
/mnt/games/scummvm/
/mnt/games/scummvm-data/
/mnt/games/scummvm-notes/
/mnt/games/saves/scummvm/
```

## Enhancements

Scalers, aspect correction, subtitles and audio options should be set per game where needed.

## Common Problems

- Missing data files.
- Wrong game variant detected.
- Saves stored outside the main backup path.
- Original media discarded after copying data.

> **DAP Tip**
>
> ScummVM is not a dumping ground for every old adventure. Check official support and keep unsupported games in their proper DOS or Windows workflow.

## DAP Gold Standard Setup

- Original data retained.
- ScummVM game ID recorded.
- Save path backed up.
- Original media archived separately.

## What Comes Next

Next comes other historically important systems, gathering machines that may later earn full standalone chapters.
