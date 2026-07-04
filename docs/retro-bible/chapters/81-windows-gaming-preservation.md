# Windows Gaming Preservation

Old Windows games are not old in a simple way. They are installers, registry keys, DirectX versions, codecs, launchers, DRM and save folders hiding in places nobody remembers.

This chapter is the skeleton for preserving Windows games on a Linux-first setup.

## History

Windows gaming grew through the 1990s and 2000s with rapid hardware change, CD-ROM installs, patches, demos, mods and storefronts. The result is rich, messy and worth preserving.

## Hardware Overview

For preservation purposes, the "hardware" is often the expected software environment: Windows version, DirectX, GPU features, sound APIs and input devices.

## Why It Mattered

PC gaming shaped modding, online play, strategy games, shooters, simulation and independent development.

## Preservation Status

Mixed. Some games are easy in Proton or Wine; others rely on dead DRM, missing patches or ancient middleware.

## Recommended Tools

| Use | Tool |
| --- | --- |
| Steam games | Proton |
| Non-Steam Windows games | Wine, Bottles or Lutris |
| Older DOS-era Windows overlap | DOSBox-X or PCem-style tools |
| Preservation notes | Per-game runbooks |

## BIOS Requirements

Not normally relevant, but virtual machines may need their own installation media and licence handling.

## Recommended File Formats

- Preserve original installers or disc images.
- Keep patches, serial notes and manuals privately and legally.
- Keep mod installers versioned.

## Folder Layout

```text
/mnt/games/pc/windows/
/mnt/games/pc/windows-media/
/mnt/games/pc/windows-patches/
/mnt/games/pc/windows-notes/
```

## Enhancements

Wrappers, widescreen fixes, dgVoodoo-style tools and community patches can be essential. Record them like dependencies, not decoration.

## Common Problems

- Missing disc check or DRM workaround.
- Installer needs old runtimes.
- Save path hidden in prefix.
- Widescreen patch breaks UI.
- Mods depend on exact game version.

> **DAP Warning**
>
> Do not preserve only the installed folder. Without installers, patches and notes, many Windows games become archaeology projects.

## DAP Gold Standard Setup

- Original media or installer retained.
- Working Wine or Proton prefix documented.
- Patch level recorded.
- Saves backed up.

## What Comes Next

Next comes ScummVM, a preservation-friendly home for many adventure games that deserve their own treatment.
