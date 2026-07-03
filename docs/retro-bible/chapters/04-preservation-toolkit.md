# Preservation Toolkit

The Preservation Toolkit is the part of the DAP Retro Bible that separates a tidy retro library from a folder of guesses.

A good preservation workflow is not built from one magic program. It is built from several kinds of tools: verification databases, emulators, metadata services, archive formats, conversion utilities, artwork sources, checksum tools and documentation. Each has a job. Trouble starts when those jobs get confused.

This chapter introduces the main tools and projects that appear throughout the book.

## The Main Layers

A retro library has several layers:

| Layer | Purpose | Examples |
| --- | --- | --- |
| Verification | Identify whether files match known dumps | Redump, No-Intro, TOSEC, MAME DATs |
| Emulation | Run the software | Dolphin, PCSX2, RPCS3, MAME, RetroArch cores |
| Metadata | Describe and decorate the library | ScreenScraper, SteamGridDB, MobyGames-style data |
| Storage | Keep files organised and maintainable | Folder structure, checksums, archive formats |
| Frontend | Present and launch the library | ES-DE, RomM, LaunchBox, Pegasus, Steam ROM Manager |
| Enhancement | Improve presentation or playability | HD packs, shaders, widescreen patches, 60 FPS patches |

No single layer replaces the others.

> **DAP Tip**
>
> Treat verification, metadata and frontend presentation as separate jobs. A game can have perfect artwork and still be a bad dump.

## Redump

Redump is focused on optical disc preservation.

It is important for systems where disc layout, tracks, offsets, regions and revisions matter. A disc image is not automatically good because an emulator boots it. Redump-style verification helps identify whether a dump matches known data.

Commonly relevant systems include:

- PlayStation;
- PlayStation 2;
- Dreamcast;
- GameCube;
- Wii;
- Saturn;
- many computer CD formats.

Redump is best understood as a verification and documentation project, not a frontend and not a general-purpose launcher.

## No-Intro

No-Intro focuses heavily on cartridge and digital content verification.

It grew out of a need for cleaner sets without scene intros, trainers and avoidable modifications. A No-Intro-style workflow helps answer the basic question: "What exactly is this file?"

No-Intro is especially useful for:

- cartridge-based consoles;
- handheld systems;
- some digital store content;
- clean naming and DAT workflows.

> **DAP Warning**
>
> Renaming files by hand may make a frontend look cleaner, but it can break the relationship between the file and its verification data.

## TOSEC

TOSEC is broad and especially valuable for home computer preservation.

Computer libraries are messy. They include games, demos, applications, cover disks, cracked variants, trainers, compilations, utilities and obscure regional releases. A console-style "one game, one ROM" mindset does not always fit.

TOSEC is particularly relevant for systems such as:

- Amiga;
- Atari ST;
- Commodore 64;
- ZX Spectrum;
- Amstrad CPC;
- MS-DOS and other computer ecosystems.

Its value is in cataloguing breadth.

## MAME

MAME is both emulator and documentation project.

Arcade preservation is not simply about owning ZIP files. Arcade games depended on boards, chips, sound hardware, display timing, controls and sometimes unusual protection. MAME records a huge amount of that structure.

MAME also has a strict relationship between emulator versions and ROM sets. A random arcade set may not work with a random MAME build.

> **DAP Tip**
>
> Keep arcade ROM sets paired with the MAME version they were built for. Arcade troubleshooting becomes much easier when the set and emulator are not fighting each other in the dark.

## ScreenScraper

ScreenScraper provides metadata and media for frontends.

It can help populate:

- covers;
- screenshots;
- videos;
- descriptions;
- release dates;
- genres;
- publisher and developer information.

ScreenScraper is useful for ES-DE and similar frontends, but it does not verify the underlying ROM or disc image.

Metadata makes the shelf look good. Verification tells you what is on the shelf.

## SteamGridDB

SteamGridDB is useful when integrating non-Steam games with Steam-style libraries.

It helps provide:

- grid images;
- heroes;
- logos;
- icons;
- artwork for non-Steam shortcuts.

It is especially helpful when using Steam ROM Manager or launching emulators through Steam for Sunshine and Moonlight streaming.

SteamGridDB is presentation infrastructure. It is not preservation infrastructure.

## Wii U Downloader and Similar Tools

Some ecosystems require special handling because content is split into titles, updates, DLC, tickets, metadata and installed formats.

Tools such as Wii U Downloader-style workflows are useful because they understand platform-specific packaging better than generic archive managers.

The important distinction is this:

- a platform-aware tool may understand updates, DLC and installable content;
- a generic archive tool only sees files;
- a frontend may only see a launch target.

Do not flatten these workflows too early. Keep source notes.

## NoPayStation-Style Workflows

NoPayStation-style workflows are often discussed around PlayStation content and metadata.

For the purposes of this book, these workflows should be handled carefully and legally. The book may explain concepts such as packages, licence files, updates, DLC organisation and emulator expectations, but it should not provide infringing links, copyrighted packages, firmware, BIOS files or keys.

The useful preservation lesson is that modern console libraries are not always a single file. They often include:

- base titles;
- updates;
- DLC;
- licences;
- account-related entitlements;
- metadata;
- firmware dependencies.

A preservation workflow must account for that complexity.

## DAT Files

DAT files describe known files and their checksums.

They are used by tools that scan a collection and compare files against known entries. A DAT-based workflow can help identify:

- correct files;
- renamed files;
- missing files;
- duplicate files;
- bad dumps;
- unexpected revisions.

DAT files are not glamorous. That is part of their charm. They are the quiet librarians of retro preservation.

## Checksums

A checksum is a fingerprint for data.

Common examples include:

- CRC32;
- MD5;
- SHA-1;
- SHA-256.

For preservation work, checksums help prove that two files are the same or identify that they are different. They do not explain why a file is different, but they give you a reliable starting point.

Useful Linux commands include:

```bash
sha1sum game.iso
sha256sum game.iso
md5sum bios.bin
```

## Archive and Conversion Tools

Different systems use different storage formats. Some formats are generic archives. Others are emulator-native or preservation-aware formats.

Common tools and formats include:

- `zip`;
- `7z`;
- `rar`;
- CHD;
- RVZ;
- WUA;
- CSO;
- ISO;
- BIN/CUE;
- GDI;
- XISO.

A later chapter covers archive formats and compression in detail.

The short rule is simple: do not compress blindly. Choose the format that the emulator, frontend and preservation workflow expect.

## Frontend Tools

Frontends help browse and launch games, but they should sit on top of a clean library.

Important frontend tools include:

- ES-DE;
- RomM;
- LaunchBox;
- Pegasus;
- Steam ROM Manager;
- RetroArch playlists.

A frontend can make a collection pleasant to use, but it should not be the only thing making the collection understandable.

## Real-World DAP Setup

The DAP workflow should favour a clean source library and then connect frontends to it.

A sensible starting point is:

```text
/mnt/games/
  bios/
  roms/
  saves/
  states/
  texture-packs/
  patches/
  metadata/
```

From there, frontends can use symlinks, bind mounts or configured paths rather than forcing every tool to own its own copy of the same library.

## Common Mistakes

Common toolkit mistakes include:

- treating metadata scraping as verification;
- mixing Redump, No-Intro, TOSEC and random files without labels;
- converting files before confirming emulator support;
- deleting original dumps too soon;
- using one archive format for every system;
- assuming a frontend knows what a preservation set is;
- storing BIOS files inside multiple emulator folders with no source record.

## Key Points

- Preservation is built from layers.
- Redump, No-Intro, TOSEC and MAME help identify and document files.
- ScreenScraper and SteamGridDB improve presentation, not verification.
- DAT files and checksums are boring but powerful.
- Archive formats should be chosen per system.
- Frontends should not be treated as the source of truth.

## What Comes Next

Next we separate two ideas that are constantly confused: metadata and storage. One describes the library. The other is the library.