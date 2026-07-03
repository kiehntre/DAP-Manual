# Metadata vs Storage

Metadata and storage are often confused because frontends make them look like one thing.

A frontend shows cover art, descriptions, release years and neat system lists. Underneath that presentation, the real library may be clean, chaotic, verified, duplicated, broken or barely understood. The frontend can make any of those states look attractive.

This chapter separates the library itself from the information used to describe it.

## The Short Version

Storage is the actual collection.

Metadata is information about the collection.

A frontend is a way to browse and launch the collection.

These three jobs overlap in daily use, but they should not be treated as the same thing.

| Concept | What it is | What it is not |
| --- | --- | --- |
| Storage | ROMs, disc images, BIOS files, patches, saves and related files | Artwork or descriptions |
| Metadata | Titles, artwork, release dates, genres and descriptions | Proof that the files are correct |
| Frontend | A user interface for browsing and launching games | The source of truth |
| Verification | Checksums and DAT matching | Pretty presentation |

## Storage Is the Source of Truth

The storage layer is where the real files live.

Examples include:

```text
/mnt/games/roms/ps2/
/mnt/games/roms/gamecube/
/mnt/games/bios/
/mnt/games/patches/
/mnt/games/texture-packs/
/mnt/games/saves/
```

A clean storage layer should be understandable without opening a frontend. If ES-DE, RomM, LaunchBox or Steam ROM Manager disappeared tomorrow, the collection should still make sense from the filesystem.

That does not mean every filename must be beautiful. It means there should be a deliberate structure and a reason for the naming.

## Metadata Makes the Library Pleasant

Metadata is what makes a collection easier to browse.

Useful metadata includes:

- game title;
- platform;
- region;
- release date;
- publisher;
- developer;
- genre;
- description;
- box art;
- screenshots;
- video previews;
- logos;
- fan art;
- play count;
- favourite status.

Metadata turns a file browser into a library shelf.

But metadata can be wrong. A scraper can match the wrong game. A clone can inherit artwork from a parent. A translation patch can be displayed as the original release. A hack can look official. A duplicate can appear as two separate games.

Metadata is useful, but it is not evidence.

## Verification Proves Identity

Verification is how we identify what a file actually is.

A verified file may be matched against a checksum from a known database or DAT file. This can tell us whether the file matches a known dump, region or revision.

A good verification workflow can answer questions such as:

- Is this file known?
- Which region is it?
- Which revision is it?
- Has it been modified?
- Is it a bad dump?
- Is it a duplicate under a different name?

Metadata answers "How should this appear?"

Verification answers "What is this file?"

## Frontends Are Views

A frontend is a view over the library.

Examples include:

- ES-DE;
- RomM;
- LaunchBox;
- Pegasus;
- Steam ROM Manager;
- RetroArch playlists.

Frontends can be excellent. They make the collection usable from a couch, handheld or TV. They make artwork and descriptions visible. They handle emulator launch commands. They can hide complexity.

That last point is both a strength and a danger.

If a frontend is the only place where the collection makes sense, the setup is fragile. A database problem, failed scrape, broken path or accidental reset can make the library feel lost even when the files still exist.

The storage layer should be able to survive a frontend rebuild.

## Why This Matters

Confusing metadata and storage causes real problems.

A user may think a collection is organised because the frontend looks clean. Then they try to move it, scan it with another tool or verify it against DAT files and discover chaos underneath.

Common symptoms include:

- duplicate games;
- wrong artwork;
- incorrect regions;
- hacks mixed with originals;
- compressed files unsupported by the emulator;
- BIOS files scattered across several folders;
- launch paths that only work on one machine;
- save files stored in unexpected places;
- frontends pointing at stale symlinks.

A good setup avoids these traps by keeping roles clear.

## Recommended Structure

A simple structure is better than a clever one.

```text
/mnt/games/
  bios/
  roms/
    arcade/
    dreamcast/
    gamecube/
    ps1/
    ps2/
    ps3/
    saturn/
    scummvm/
    switch/
    xbox/
  saves/
  states/
  patches/
  texture-packs/
  metadata/
  tools/
```

The exact folder names can change, but the idea should stay stable:

- source files live in predictable places;
- BIOS files have a controlled home;
- saves and states are not mixed with source media;
- patches and texture packs are separate from base games;
- metadata can be rebuilt if needed.

## Metadata Cache Locations

Many tools keep their own metadata caches.

That is normal. What matters is understanding that those caches are not the master library.

Examples:

| Tool | Metadata role |
| --- | --- |
| ES-DE | Scraped game lists, artwork and videos |
| RomM | Web library metadata, covers and platform data |
| LaunchBox | Game database, artwork and launch configuration |
| Steam ROM Manager | Steam shortcut artwork and launch entries |
| RetroArch | Playlists and thumbnails |

Each tool may store metadata differently. That is fine as long as the underlying storage remains sane.

## Real-World DAP Setup

The DAP approach should keep the main library independent from the frontend.

A practical model is:

```text
/mnt/games/roms/          # source game library
/mnt/games/bios/          # controlled BIOS source
/mnt/games/texture-packs/ # optional enhancements
/mnt/games/patches/       # optional patches
```

Then frontends can connect through configured paths, symlinks or Docker bind mounts.

For example:

```text
~/retrodeck/roms/gamecube -> /mnt/games/roms/gamecube
~/retrodeck/bios/saturn   -> /mnt/games/bios/saturn
```

Symlinks should be documented. They are powerful, but invisible links become future confusion if nobody records them.

## DAP Tip

> **DAP Tip**
>
> Before scraping metadata, make sure the storage layout is sane. Scraping a messy folder only gives you a prettier mess.

## DAP Warning

> **DAP Warning**
>
> Do not use a frontend's generated database as your only record of the library. Databases can corrupt, paths can change and scrape results can be wrong.

## Common Mistakes

Common mistakes include:

- treating a scraped game list as a verified collection;
- moving ROM files after scraping without updating paths;
- storing BIOS files inside frontend folders only;
- letting every tool create its own copy of the same game;
- mixing original games, hacks and translations without labels;
- deleting source files because a compressed frontend copy exists;
- ignoring where saves and states are stored.

## Troubleshooting

### The frontend shows the wrong game

Possible causes:

- filename matched the wrong metadata entry;
- region or revision missing from the name;
- hack or translation patch detected as the base game;
- scraper database has incomplete information.

Fix:

- check the actual filename;
- add region or version information;
- manually edit the metadata entry;
- keep hacks and translations in clearly labelled folders where practical.

### A game launches in one frontend but not another

Possible causes:

- one frontend extracts archives automatically and the other does not;
- launch command differs;
- emulator path differs;
- Flatpak permissions block access;
- symlink target is unavailable.

Fix:

- test the emulator directly;
- confirm the file format is supported;
- check path permissions;
- confirm the frontend points at the real storage path.

### Artwork vanished after moving files

Possible causes:

- metadata tied to old paths;
- frontend database not updated;
- artwork cache stored outside the library;
- game IDs changed after rescraping.

Fix:

- rescan carefully;
- preserve metadata folders before major moves;
- document path changes;
- avoid unnecessary renaming after scraping.

## Key Points

- Storage is the real collection.
- Metadata describes the collection.
- Verification identifies files.
- Frontends are views over the collection.
- A clean library should survive a frontend rebuild.
- Metadata can be wrong even when it looks good.
- Symlinks and bind mounts should be documented.

## What Comes Next

Next comes archive formats and compression: which formats save space, which emulators can read them and which choices create a future headache wearing a clever hat.