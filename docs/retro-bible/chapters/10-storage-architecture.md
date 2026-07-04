# Storage Architecture

Storage architecture is the difference between a retro library that grows gracefully and one that becomes a cupboard full of unlabeled cables.

A good storage layout is boring in the best possible way. You should be able to look at the folder tree and understand what lives where, which files are source material, which files are generated, where saves live and which frontends are merely pointing at the collection.

This chapter lays out a practical Linux-first storage model for the DAP Retro Bible.

## The Goal

The storage layer should be:

- readable from the filesystem;
- stable across frontends;
- easy to back up;
- friendly to emulators;
- clear about BIOS, saves, patches and artwork;
- suitable for local disks or network storage;
- able to survive a frontend rebuild.

The storage layout is the foundation. Everything else sits on top.

## Source of Truth

The source of truth should be the organised library folder, not a frontend database.

A frontend can scrape metadata, generate artwork caches and keep launch configuration. That is useful, but the files themselves should live in a structure that still makes sense when the frontend is closed.

Recommended base path:

```text
/mnt/games/
```

This can be a local SSD, a mounted NAS path, a mergerfs pool or another reliable Linux mount. The exact backend matters less than the structure and documentation.

## Recommended Layout

A practical starting layout:

```text
/mnt/games/
  bios/
  roms/
  saves/
  states/
  patches/
  texture-packs/
  metadata/
  tools/
  docs/
```

Each top-level folder has a clear purpose.

| Folder | Purpose |
| --- | --- |
| `bios/` | Controlled BIOS and firmware source files. |
| `roms/` | ROM images, disc images and game folders. |
| `saves/` | Save files and memory cards where they can be redirected. |
| `states/` | Save states where they can be redirected. |
| `patches/` | Translation patches, widescreen patches and fixes. |
| `texture-packs/` | HD packs and replacement texture folders. |
| `metadata/` | Optional shared metadata exports and notes. |
| `tools/` | Helper tools, scripts and version notes. |
| `docs/` | Local notes about the library layout. |

## ROM System Folders

System folders should be predictable.

Example:

```text
/mnt/games/roms/
  amiga/
  arcade/
  dreamcast/
  gamecube/
  n64/
  ps1/
  ps2/
  ps3/
  saturn/
  scummvm/
  switch/
  wii/
  wiiu/
  xbox/
```

Folder names should be lowercase and boring unless a frontend requires a specific name. If a frontend needs a different name, document it rather than renaming the source library every time.

## Original Files vs Working Files

Some libraries need both source files and working files.

For example:

- a Redump BIN/CUE source may be converted to CHD for daily use;
- a GameCube ISO may be converted to RVZ for Dolphin;
- a PSP ISO may be converted to CSO;
- a ScummVM game may stay as a folder.

If source and working formats differ, record it.

A simple notes file can prevent confusion:

```text
/mnt/games/roms/ps1/_conversion-notes.md
/mnt/games/roms/gamecube/_format-policy.md
```

## Saves and States

Saves are precious. Treat them separately from source media.

Where possible, document or redirect:

- memory cards;
- native saves;
- save states;
- emulator profiles;
- screenshots;
- controller profiles.

Not every emulator makes this easy. Some store saves under the user's home directory. Some Flatpak builds store saves inside sandboxed paths. Some standalone emulators use their own config folders.

The important rule is simple: know where the saves live before something breaks.

## BIOS Source Folder

BIOS and firmware files should have one controlled source.

Recommended path:

```text
/mnt/games/bios/
```

Emulators may require copies or links elsewhere, but the source folder should remain clear.

Good practice:

- group by system;
- record region where useful;
- keep checksum notes;
- avoid mystery packs;
- document any symlinks.

## Patches

Patches should not be mixed blindly with original games.

Useful categories:

```text
/mnt/games/patches/
  translations/
  widescreen/
  60fps/
  bugfixes/
  romhacks/
```

Where possible, record:

- target game;
- target region;
- target revision;
- patch version;
- patch source;
- whether the patched output is stored separately.

A patch for the wrong revision can fail or create subtle bugs.

## Texture Packs

Texture packs can become large and messy.

Recommended structure:

```text
/mnt/games/texture-packs/
  dolphin/
  pcsx2/
  ppsspp/
  retroarch/
```

Within each emulator folder, use system or game names that match the emulator's expectations.

Do not assume every frontend needs to see texture packs. Usually the emulator needs them, not the library browser.

## Symlinks

Symlinks are useful when a frontend expects its own folder structure.

Example concept:

```text
frontend-roms/gamecube -> /mnt/games/roms/gamecube
```

Symlinks should be documented because they are invisible until they are not.

Record:

- source path;
- target path;
- reason;
- tool that depends on it.

## Docker Bind Mounts

Docker services see container paths, not host paths.

A service might see this:

```text
/roms
/bios
```

while the host uses this:

```text
/mnt/games/roms
/mnt/games/bios
```

Document both paths. Many Docker problems are really path translation problems wearing a false moustache.

## Flatpak Paths

Flatpak applications may not see external storage unless permission is granted.

If an emulator installed through Flatpak cannot see `/mnt/games/`, check sandbox permissions before blaming the ROM.

Record Flatpak-specific paths and overrides in the relevant system chapter.

## Local SSD vs Network Storage

Local SSD storage is usually best for:

- large disc games;
- shader compilation;
- texture packs;
- RPCS3-style workloads;
- quick frontend browsing.

Network storage can work well for:

- ROM libraries;
- archived source files;
- shared collections;
- metadata backups;
- less demanding systems.

The best answer may be a hybrid: hot games on SSD, wider library on network storage.

## Backups

A retro library backup should include more than ROMs.

Consider backing up:

- saves;
- memory cards;
- save states;
- BIOS notes;
- emulator profiles;
- frontend metadata;
- patches;
- texture pack configuration;
- conversion notes;
- custom scripts.

If a rebuild loses all saves, the backup was incomplete.

## Real-World DAP Setup

A DAP-style storage model should favour a single shared source library:

```text
/mnt/games/
```

Frontends and emulators should point into that library through direct paths, symlinks or bind mounts.

The guiding principle:

> **DAP Tip**
>
> Let the library own the files. Let frontends own presentation. Let emulators own execution.

## Common Mistakes

Common storage mistakes include:

- keeping the only copy of games inside a frontend folder;
- mixing saves with ROMs;
- copying BIOS files everywhere;
- failing to document symlinks;
- converting files without recording the source;
- treating network storage like a local SSD;
- forgetting Flatpak and Docker path differences;
- backing up games but not saves.

## Key Points

- Storage is the source of truth.
- Frontends should be rebuildable.
- Saves and states need deliberate handling.
- BIOS files need one controlled source.
- Patches and texture packs should be separate from base games.
- Symlinks and bind mounts are powerful but must be documented.
- Backups should include configuration and saves, not only games.

## What Comes Next

Next comes Linux Foundations: the operating system layer, permissions, packages, drivers and desktop choices that make the retro stack reliable.