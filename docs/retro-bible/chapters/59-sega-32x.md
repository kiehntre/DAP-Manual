# Sega 32X

The 32X is awkward, fascinating and very Sega.

This chapter is the skeleton for treating the 32X as its own preservation target rather than a footnote dumped into the Mega Drive folder.

## History

The 32X arrived as an add-on for the Mega Drive and Genesis near the edge of the 32-bit generation. It was commercially short-lived, but historically useful because it shows how messy platform transitions can become.

## Hardware Overview

- Add-on hardware for Mega Drive and Genesis.
- Cartridge media.
- Depends on the base console.
- Small library with regional differences.

## Why It Mattered

It is a case study in timing, market confusion and the cost of transitional hardware.

## Preservation Status

The library is small and manageable. Emulator support varies more than for normal Mega Drive games.

## Recommended Emulator

| Use | Recommendation |
| --- | --- |
| Practical use | PicoDrive |
| Accuracy research | ares, where suitable |

## BIOS Requirements

32X emulation may require BIOS files depending on emulator. Record expected filenames and checksums in the BIOS chapter during expansion.

## Recommended File Formats

- `.32x` for cartridge images where used.
- Keep separate from plain Mega Drive.

## Folder Layout

```text
/mnt/games/roms/32x/
/mnt/games/bios/32x/
/mnt/games/saves/32x/
```

## Enhancements

Keep enhancements minimal. The historical value of 32X often lies in seeing what it really was.

## Common Problems

- Missing BIOS files.
- Wrong emulator core.
- Games filed under Mega Drive and launched with the wrong emulator.
- Controller mapping inherited incorrectly.

> **DAP Warning**
>
> Do not assume Mega Drive emulator settings automatically cover 32X. The add-on needs its own launch path and BIOS notes.

## DAP Gold Standard Setup

- Small verified set.
- BIOS files verified where required.
- Dedicated frontend system or launch rule.
- Emulator tested per game.

## What Comes Next

Next comes Saturn, where Sega's hardware becomes deeper, stranger and more demanding.
