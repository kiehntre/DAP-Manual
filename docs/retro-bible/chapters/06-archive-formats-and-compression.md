# Archive Formats and Compression

Compression is one of the easiest ways to save space and one of the quickest ways to break an emulator setup.

A file ending in `.zip`, `.7z`, `.chd`, `.rvz`, `.cso`, `.wua` or `.iso` tells you something about how the data is stored. It does not automatically tell you whether the emulator can use it, whether a frontend can launch it or whether the file is still suitable for verification.

This chapter explains which formats are useful, which are awkward and which should be avoided for particular systems.

## The Big Rule

Use the format expected by the emulator and the preservation workflow.

Do not compress everything the same way.

A tiny cartridge ROM in a ZIP file and a PlayStation 2 disc image in CHD are different situations. A GameCube game in RVZ and an arcade ROM set in ZIP are different again. The right choice depends on system, emulator, frontend, verification needs and storage constraints.

> **DAP Warning**
>
> Do not bulk-convert a verified collection until you have tested the target emulator, frontend and backup plan. Compression is easy. Unpicking a bad conversion spree is archaeology with swearing.

## Generic Archives vs Emulator Formats

There are two broad categories:

| Type | Examples | Purpose |
| --- | --- | --- |
| Generic archives | ZIP, 7z, RAR | Store one or more files in a compressed container |
| Emulator or preservation formats | CHD, RVZ, WUA, CSO, XISO | Store game media in a format designed or commonly used for emulation |

Generic archives are useful, but many emulators do not read every archive format directly. Some frontends can extract archives before launching. Others cannot. Some workflows appear to work until a game needs multiple discs, save paths or fast random access.

Emulator-native formats are often better for disc-based systems because they are designed around how the emulator reads the data.

## ZIP

ZIP is widely supported for small ROM-based systems and arcade sets.

Good uses:

- NES;
- SNES;
- Mega Drive / Genesis;
- Master System;
- Game Boy;
- Game Boy Advance;
- many small cartridge systems;
- MAME and arcade ROM sets where ZIP is expected.

Poor uses:

- large disc images;
- systems where the emulator expects CHD, ISO, RVZ, WUA or another native format;
- multi-file disc images unless the emulator explicitly supports that layout.

ZIP is old, boring and useful. That is not an insult. Boring tools often survive.

## 7z

7z can compress better than ZIP, but support is less universal.

Good uses:

- cold storage;
- archival copies;
- collections that a frontend extracts before launching;
- small ROMs where the chosen emulator or frontend supports it.

Poor uses:

- disc-based games launched directly by emulators that do not read 7z;
- large games over network storage where extraction adds delay;
- couch setups where launch speed matters;
- libraries shared across several frontends with different archive support.

The classic problem is simple: the file exists, the frontend sees it, but the emulator cannot run it directly.

> **DAP Tip**
>
> Use 7z for storage only when you know who will extract it and when. If the answer is "the emulator, probably", check first.

## RAR

RAR is common in downloaded archives but is rarely a good long-term emulator library format.

Good uses:

- temporary extraction from a source archive;
- receiving files that need unpacking once.

Poor uses:

- live emulator libraries;
- preservation sets;
- frontend scraping;
- long-term open workflows.

RAR support varies, and it is not ideal for open, Linux-first, long-term library design.

For the DAP Retro Bible, RAR is treated as an unpacking format, not a preferred storage format.

## CHD

CHD stands for Compressed Hunks of Data. It is widely associated with MAME and is also used by many emulators for disc-based systems.

Good uses:

- PlayStation;
- PlayStation 2 where supported by the emulator workflow;
- Dreamcast;
- Saturn;
- PC Engine CD;
- Sega CD;
- some arcade hard drive and disc media;
- other CD-based systems with emulator support.

Benefits:

- good compression;
- single-file output for many disc images;
- strong emulator support across several systems;
- useful for reducing BIN/CUE clutter.

Risks:

- not every emulator supports CHD for every system;
- conversion should be tested before deleting originals;
- some preservation workflows may still prefer original dump formats.

Example conversion:

```bash
chdman createcd -i "game.cue" -o "game.chd"
```

CHD is one of the most useful formats in retro storage, but it is not a universal hammer.

## RVZ

RVZ is a Dolphin format for GameCube and Wii disc images.

Good uses:

- GameCube with Dolphin;
- Wii with Dolphin;
- libraries where Dolphin is the main emulator.

Benefits:

- good compression;
- designed for Dolphin;
- better suited to modern preservation workflows than older Dolphin compression formats;
- can preserve important disc data while saving space.

Poor uses:

- non-Dolphin emulators;
- tools that only understand ISO or WBFS;
- workflows where files must stay in Redump-style original form.

For a Dolphin-first setup, RVZ is usually the preferred working format.

## GCZ

GCZ is an older Dolphin compressed format.

It may still appear in old collections, but RVZ is generally the better modern choice for Dolphin workflows.

Good uses:

- legacy collections that already use it and still work.

Poor uses:

- new conversions;
- long-term planning when RVZ is available.

If starting fresh, prefer RVZ.

## WBFS

WBFS is associated with Wii backup workflows.

It was useful historically, especially for loaders and older storage setups, but it is not usually the best modern archival choice.

Good uses:

- compatibility with specific older Wii workflows;
- existing collections that depend on it.

Poor uses:

- new Dolphin-first libraries;
- preservation-focused workflows;
- mixed frontend setups where RVZ or ISO is easier to reason about.

For the DAP Retro Bible, WBFS belongs mostly in the historical and compatibility bucket.

## WUA

WUA is a Wii U archive format used by Cemu.

Good uses:

- Wii U libraries for Cemu;
- combining game, update and DLC content into a cleaner format where supported.

Benefits:

- tidier than loose installed folders;
- convenient for frontend launching;
- useful for keeping Wii U content manageable.

Risks:

- support depends on emulator and tooling;
- source content should be understood before conversion;
- updates and DLC handling must be documented.

Wii U preservation is not only about a base game. Updates and DLC often matter.

## CSO

CSO is a compressed ISO format commonly associated with PSP.

Good uses:

- PSP games where storage space matters;
- PPSSPP workflows that support CSO.

Benefits:

- saves space;
- widely recognised in PSP emulation.

Risks:

- compression level can affect loading performance;
- some games may behave better as ISO;
- not ideal as the only archival copy if original verification matters.

For PSP, CSO is practical. Keep important originals or verification notes where possible.

## ISO

ISO is a broad term and sometimes used loosely.

Good uses:

- simple disc images;
- systems and emulators that expect ISO directly;
- temporary source before conversion;
- PS2, PSP, Xbox and other disc workflows depending on emulator support.

Risks:

- large file size;
- may not capture every disc layout correctly for every system;
- can be confused with generic renamed files;
- multi-track systems may need BIN/CUE, GDI or another structure instead.

Do not assume ISO is always the most accurate format. It depends on the system.

## BIN/CUE

BIN/CUE is common for CD-based systems.

Good uses:

- PlayStation;
- Saturn;
- Sega CD;
- PC Engine CD;
- multi-track CD images;
- source format before CHD conversion.

Benefits:

- represents track layout;
- widely used by preservation workflows;
- good source for CHD conversion.

Risks:

- multiple files per game;
- easy to break by moving BIN files without the CUE;
- messy in frontends if not handled carefully.

If using BIN/CUE, keep all related files together and do not rename only one part.

## GDI

GDI is commonly associated with Dreamcast disc dumps.

Good uses:

- Dreamcast preservation source files;
- conversion to CHD where supported.

Risks:

- multi-file layout;
- can be messy in frontends;
- CDI files are often not equivalent preservation-quality dumps.

For Dreamcast, CHD is often a convenient working format, while GDI may be retained as a source format depending on the preservation goal.

## XISO

XISO is commonly used for original Xbox emulation workflows.

Good uses:

- original Xbox games for emulators such as xemu;
- converting extracted Xbox disc content into an emulator-friendly image.

Risks:

- Xbox formats are often misunderstood;
- a generic ISO is not necessarily an Xbox XISO;
- extracted folders may need conversion before use.

Xbox emulation deserves its own careful handling. Do not assume PC-style ISO rules apply.

## NSP and XCI

NSP and XCI are associated with Nintendo Switch content.

Good uses:

- Switch emulator workflows where legally obtained and supported;
- distinguishing installable packages from cartridge-style images.

Risks:

- firmware, keys, updates and DLC handling are legally and technically sensitive;
- compression is usually not the main problem;
- emulator compatibility and legal ownership must be handled carefully.

The DAP Retro Bible should explain structure and emulator expectations without providing copyrighted content, keys or firmware.

## Quick Reference Table

| System | Preferred working format | Also common | Avoid as live format | Notes |
| --- | --- | --- | --- | --- |
| NES / SNES / Mega Drive | ZIP or native ROM | 7z | RAR | ZIP is widely supported. |
| Game Boy / GBA | ZIP or native ROM | 7z | RAR | Keep hacks/translations labelled. |
| Arcade / MAME | ZIP plus CHD where required | 7z in some sets | Random recompression | Match set to MAME version. |
| PlayStation | CHD or BIN/CUE | ISO in some cases | RAR | Keep CUE with BIN files. |
| Saturn | CHD or BIN/CUE | ISO rarely | RAR | Multi-track layout matters. |
| Dreamcast | CHD or GDI | CDI | RAR | CDI is not ideal for preservation. |
| PlayStation 2 | CHD or ISO | GZ in some workflows | RAR | Confirm emulator support. |
| PSP | CSO or ISO | CHD in some workflows | RAR | Test loading performance. |
| GameCube | RVZ | ISO | 7z/RAR | Dolphin-first choice. |
| Wii | RVZ | ISO, WBFS | 7z/RAR | WBFS is mostly legacy. |
| Wii U | WUA | Loadiine folder, installed content | ZIP/RAR | Updates and DLC matter. |
| Original Xbox | XISO | Extracted folders | Generic ZIP/RAR | Generic ISO may be wrong. |
| Switch | NSP/XCI | Installed content | ZIP/RAR as live format | Keys/firmware are sensitive. |
| DOS | Folder or ZIP depending on launcher | 7z for storage | RAR | Many games expect writable folders. |
| ScummVM | Game folder | ZIP only when supported | RAR | Folder layout matters. |

## Frontend Extraction

Some frontends or launcher scripts can extract archives before launching.

This sounds convenient, but it has trade-offs:

- slower first launch;
- temporary storage requirements;
- confusion around saves;
- problems with multi-disc games;
- repeated extraction over network storage;
- harder troubleshooting.

For small ROMs, direct ZIP support is usually fine. For large disc games, emulator-native compressed formats are usually better than generic archives.

## Network Storage and Streaming

Compression interacts with storage speed.

A large `.7z` file on network storage may need to be extracted before launch. A CHD or RVZ may stream data more naturally because the emulator reads it as a disc image. Over Sunshine and Moonlight, launch delay and emulator behaviour matter more than the frontend merely detecting the file.

For a sofa-friendly setup:

- avoid formats that require long extraction;
- favour emulator-native compression;
- keep saves on reliable storage;
- test launch times from the frontend, not only the emulator;
- document any wrapper scripts.

## Real-World DAP Setup

For the DAP setup, a practical policy is:

```text
/mnt/games/roms/gamecube/   -> RVZ
/mnt/games/roms/wii/        -> RVZ
/mnt/games/roms/ps1/        -> CHD or BIN/CUE
/mnt/games/roms/ps2/        -> CHD or ISO after testing
/mnt/games/roms/psp/        -> CSO or ISO
/mnt/games/roms/arcade/     -> ZIP plus required CHDs
/mnt/games/roms/scummvm/    -> folders
/mnt/games/roms/xbox/       -> XISO where required
```

Keep notes beside any system where the working format differs from the preservation source.

Example:

```text
/mnt/games/roms/gamecube/_notes.md
/mnt/games/roms/ps2/_conversion-notes.md
```

## Common Mistakes

Common mistakes include:

- putting every game into 7z and expecting every emulator to cope;
- using RAR as a live library format;
- deleting BIN files and keeping only the CUE;
- converting Redump sources without keeping notes;
- treating WBFS as the best Wii archival format by default;
- confusing Xbox ISO and XISO;
- assuming frontend detection means emulator support;
- mixing compressed and uncompressed files without a policy.

## Troubleshooting

### The frontend sees the game but it will not launch

Possible causes:

- emulator does not support the archive format;
- frontend passes the archive directly instead of extracting it;
- Flatpak sandbox cannot access the extracted location;
- multi-file disc image is incomplete;
- wrong emulator selected for the platform.

Fix:

- launch the file directly in the emulator;
- check emulator documentation;
- test an uncompressed version;
- confirm frontend launch command;
- inspect permissions and paths.

### A CHD will not boot

Possible causes:

- bad source conversion;
- wrong input file;
- missing CUE data;
- emulator does not support CHD for that system;
- original dump was already bad.

Fix:

- test the original source files;
- recreate CHD from the CUE, not a random BIN;
- verify emulator support;
- compare checksums where possible.

### A GameCube game works as ISO but not RVZ

Possible causes:

- old Dolphin version;
- corrupt conversion;
- unsupported external tool;
- file copied incompletely.

Fix:

- update Dolphin;
- convert using Dolphin itself where possible;
- test locally before moving to network storage;
- keep the original until the RVZ is proven.

## Key Points

- Compression is system-specific.
- ZIP is useful for many small ROMs and arcade sets.
- 7z is good for storage but not universally playable.
- RAR should usually be extracted, not used as a live format.
- CHD is excellent for many disc systems but not universal.
- RVZ is the modern Dolphin-first choice for GameCube and Wii.
- WUA is useful for Wii U workflows where supported.
- CSO is practical for PSP but should be tested.
- Frontend support and emulator support are not the same thing.

## What Comes Next

Next comes BIOS and firmware: what they are, why some emulators need them and how to organise them without creating a cursed cupboard of duplicate files.