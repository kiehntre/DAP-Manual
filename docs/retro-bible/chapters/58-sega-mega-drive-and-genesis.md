# Sega Mega Drive and Genesis

The Mega Drive, known as the Genesis in North America, is Sega at full volume.

This chapter is the skeleton for preserving and playing Sega's 16-bit library with clean ROMs, sensible audio expectations and proper regional naming.

## History

Sega's 16-bit console pushed arcade attitude into the home. It fought Nintendo loudly and left a library packed with platformers, shooters, sports games and experiments.

## Hardware Overview

- 16-bit cartridge console.
- Regional branding and lockout differences.
- Add-ons include Mega-CD/Sega CD and 32X.
- Distinctive FM sound hardware.

## Why It Mattered

The Mega Drive gave the 16-bit era its rivalry and much of its speed.

## Preservation Status

Strong. The main practical issues are region naming, hacks, add-ons and accurate audio.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Accuracy and broad Sega support | BlastEm or ares |
| Practical multi-system use | Genesis Plus GX |
| 32X support | PicoDrive or ares depending on need |

## BIOS Requirements

Cartridge games normally do not require BIOS. Mega-CD and some add-on workflows do.

## Recommended File Formats

- `.md` or `.gen` for cartridge images.
- Keep Mega-CD and 32X folders separate.

## Folder Layout

```text
/mnt/games/roms/megadrive/
/mnt/games/roms/sega-cd/
/mnt/games/roms/32x/
/mnt/games/saves/megadrive/
```

## Enhancements

CRT shaders suit the system. Widescreen hacks exist for selected games, but should be labelled as modifications.

## Common Problems

- Mixing regions without labels.
- Confusing Mega Drive, Genesis and 32X folders.
- Poor audio settings.
- Patch source mismatch.

> **DAP Tip**
>
> Pick one folder name, such as `megadrive`, and use metadata to handle Genesis naming. The filesystem does not need every regional argument.

## DAP Gold Standard Setup

- Verified cartridge ROMs.
- Add-ons separated.
- Emulator choice recorded.
- Translation and improvement patches separated.

## What Comes Next

Next comes 32X, Sega's ambitious add-on with a small library and a long troubleshooting shadow.
