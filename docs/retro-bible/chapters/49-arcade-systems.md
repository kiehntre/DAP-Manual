# Arcade Systems

Arcade preservation is where retro gaming stops being a simple list of consoles and becomes a map of boards, revisions, controls and locations.

This chapter introduces arcade systems as a broad platform family. The MAME-specific set management details live in the arcade and MAME chapter; here the focus is the experience and the shape of a DAP setup.

## History

Arcades were public, loud and wonderfully specific. A cabinet was not only software. It was a monitor, board, control panel, coin door, sound system and sometimes a whole physical performance.

## Why It Mattered

Arcade games drove genres forward: shooters, fighters, racing games, beat 'em ups, rhythm games and light gun titles all carry arcade DNA.

## Preservation Status

MAME is central, but not the only tool. Some systems are well preserved; others need device ROMs, CHDs, unusual controls or ongoing driver work.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy and documentation | MAME |
| Selected arcade-style convenience | FinalBurn Neo |
| Frontend browsing | ES-DE or dedicated arcade layout |

## BIOS Requirements

Some arcade platforms need BIOS or device sets. Keep them with the matching arcade set and emulator version.

## Recommended File Formats

- Keep MAME ROMs zipped.
- Use CHD where the MAME set expects CHD.
- Do not extract sets unless a specific emulator requires it.

## Folder Layout

```text
/mnt/games/roms/arcade/
/mnt/games/roms/mame/
/mnt/games/roms/fbneo/
/mnt/games/arcade/chd/
```

## Enhancements

Bezels, shaders and control overlays can help arcade games feel right, but controls matter more than decoration.

## Common Problems

- Missing parent sets.
- Wrong MAME version.
- Extracted ZIPs.
- Coin and start buttons unmapped.
- Vertical games displayed poorly.

> **DAP Tip**
>
> Build arcade favourites as a curated list first. A complete arcade set is a preservation project; a playable arcade menu is a different job.

## DAP Gold Standard Setup

- MAME version recorded.
- ROM set version recorded.
- CHDs stored where MAME expects them.
- Controller, coin and start mappings tested.
- Vertical display policy documented.

## What Comes Next

Next comes the NES, where home console preservation becomes much simpler on the surface but still rewards clean naming and emulator choice.
