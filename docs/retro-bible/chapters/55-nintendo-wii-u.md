# Nintendo Wii U

The Wii U is a preservation puzzle wrapped in a misunderstood console.

This chapter is the skeleton for organising Wii U games, updates, DLC and Cemu on Linux without flattening the platform's structure too early.

## History

The Wii U followed the Wii with a tablet-style GamePad, asymmetric play and a library that later fed much of the Switch era. It sold modestly, but its ideas and games travelled far.

## Hardware Overview

- HD console with GamePad integration.
- Disc and digital software.
- Updates and DLC are common.
- Some games depend on second-screen behaviour.

## Why It Mattered

The Wii U is a reminder that commercial success and historical value are not the same thing.

## Preservation Status

Good, but the ecosystem includes base games, updates, DLC, metadata and keys. Keep source notes careful and legal.

## Recommended Emulator

Cemu is the practical recommendation for most users.

## BIOS Requirements

No traditional BIOS flow like older consoles, but title keys, account data and system files may be relevant depending on workflow. This needs careful legal treatment in later editing.

## Recommended File Formats

- WUA for compact Cemu libraries where appropriate.
- Keep unpacked or source material documented.
- Keep updates and DLC associated with the base title.

## Folder Layout

```text
/mnt/games/roms/wiiu/
/mnt/games/updates/wiiu/
/mnt/games/dlc/wiiu/
/mnt/games/saves/wiiu/
```

## Enhancements

Cemu supports graphics packs, resolution changes and community patches. Test GamePad-dependent games carefully.

## Common Problems

- Base game, update and DLC separated without notes.
- Wrong region update.
- GamePad screen ignored.
- Graphics pack overused before testing.

> **DAP Tip**
>
> For Wii U, think in title groups: base game, update, DLC, saves and notes belong together.

## DAP Gold Standard Setup

- Cemu installed from a documented source.
- WUA working library where suitable.
- Updates and DLC tracked per title.
- GamePad behaviour tested.

## What Comes Next

Next comes Switch, where current-platform ethics, keys, firmware and emulator churn make caution part of the chapter.
