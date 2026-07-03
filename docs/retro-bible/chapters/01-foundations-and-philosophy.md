# Chapter 1: Foundations and Philosophy

The DAP Retro Bible starts with a simple idea: build a retro gaming setup that is useful, tidy, repeatable, and enjoyable instead of a haunted cupboard full of random ZIP files.

This guide is not trying to turn retro gaming into homework. It is here to make the boring parts reliable so the fun parts are easy.

## What this project is for

The goal is a Linux-first retro gaming and preservation setup that can scale from a laptop to a homelab.

The setup should support:

- clean ROM and disc organisation;
- BIOS and firmware sanity checks;
- ES-DE and LaunchBox-style frontends;
- RomM for library browsing and metadata;
- emulator-specific configuration;
- Sunshine and Moonlight streaming;
- HD texture packs, widescreen patches, and 60 FPS patches;
- preservation-aware naming, verification, and archive formats;
- troubleshooting notes based on real problems.

In other words: less random clicking, more "I know where everything lives".

## The main rule

Own the chaos before it owns you.

Retro libraries grow fast. A few games become a few hundred. A few hundred become multiple systems, BIOS files, artwork, save states, patches, texture packs, controller profiles, and frontend metadata.

Without structure, it turns into digital compost. Useful compost, maybe, but still compost.

The DAP Retro Bible keeps things split into clear layers:

1. **Storage** - where the files live.
2. **Preservation** - how files are named, verified, compressed, and archived.
3. **Emulation** - how each system is run.
4. **Frontend** - how the library is browsed.
5. **Streaming** - how games reach the sofa, laptop, handheld, or TV.
6. **Troubleshooting** - what broke and how it was fixed.

## What this guide is not

This guide is not a piracy manual.

It will cover preservation projects, archive formats, BIOS placement, emulator compatibility, and tooling. It will not provide copyrighted games, firmware, keys, or links to infringing downloads.

The practical stance is:

- document legal and technical preservation workflows;
- assume users are responsible for their own dumped media and firmware;
- avoid mystery packs and magic folders where nobody knows what is inside;
- prefer verified sets, checksums, and known-good documentation.

Retro gaming should not require pretending the law is a fog machine.

## Linux-first, but not Linux-only

The main target is Linux because the DAP setup already leans that way:

- Ubuntu and Nobara machines;
- Docker services;
- Sunshine/Moonlight streaming;
- ES-DE and RetroDECK-style layouts;
- Wine and Proton where Windows tools are still useful;
- homelab storage and symlinks.

Windows tools may still appear where they make sense. LaunchBox, for example, remains useful even if Linux needs Wine and some version discipline. The rule is not purity. The rule is: use what works, write down the sharp edges.

## Preferred library layout

The exact paths can change, but the idea should stay boring and obvious:

```text
/mnt/games/
  bios/
  roms/
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
    xbox/
  saves/
  states/
  texture-packs/
  patches/
  metadata/
```

A boring folder layout is a beautiful thing. It means future-you does not need a séance to remember where the Saturn BIOS went.

## Archive formats: the early rule of thumb

Different systems prefer different formats. The full table belongs in a later appendix, but the early rule is:

- use **CHD** for many CD-based systems when emulator support is solid;
- use **RVZ** for GameCube and Wii with Dolphin;
- use **WUA** for Wii U where appropriate;
- use **CSO** carefully for PSP if space matters;
- keep arcade/MAME sets in the expected ZIP structure;
- avoid leaving playable disc games buried inside `.7z` unless the frontend or wrapper extracts them cleanly.

Compression is good. Confusion is not.

## Frontends are not the library

ES-DE, LaunchBox, RomM, Steam Rom Manager, Pegasus, RetroArch playlists, and other frontends are views over the collection. They are not the collection itself.

That distinction matters.

The library should remain useful even if a frontend breaks, a database corrupts, or a container wanders into a ditch wearing a traffic cone.

A healthy setup lets you rebuild frontends from the same clean source folders.

## BIOS files need discipline

BIOS and firmware files should be treated like system dependencies, not random treasure.

A good BIOS setup records:

- required filename;
- expected checksum where available;
- target emulator;
- target system;
- source notes;
- whether the file is optional or required.

Later chapters will cover BIOS folders and emulator-specific expectations. The important starting point is: do not scatter BIOS files everywhere and hope the emulator sniffs them out like a truffle pig.

## Streaming is part of the design

For the DAP setup, streaming is not an afterthought. Sunshine and Moonlight are part of the plan.

That means emulator choices need to consider:

- controller handling;
- fullscreen behaviour;
- audio latency;
- save paths;
- performance on the host machine;
- whether the frontend launches cleanly from the sofa.

A game that only works when launched from a terminal with six cursed flags is not sofa-ready yet.

## Preservation toolkit direction

A later chapter will cover preservation tools and databases such as:

- Redump;
- No-Intro;
- TOSEC;
- MAME;
- ScreenScraper;
- Wii U Downloader;
- NoPayStation-style metadata workflows where legally appropriate;
- DAT verification tools;
- checksum workflows;
- archive conversion tools.

The point is not to worship acronyms. The point is to know what each project is for and which emulator/front-end workflows it helps.

## First build target

The first practical target is:

1. create the folder structure;
2. collect BIOS files into one controlled location;
3. choose emulators per system;
4. wire ES-DE to the library;
5. add RomM metadata later;
6. test Sunshine/Moonlight launching;
7. document every fix.

No grand cathedral on day one. Start with a decent shed, label the shelves, then add stained glass when it earns its keep.

## Chapter checklist

After this chapter, the project should have:

- a defined purpose;
- a clear legal and preservation stance;
- a Linux-first direction;
- a starting folder layout;
- a rule that frontends are disposable views, not the source of truth;
- a path toward emulator, BIOS, streaming, and preservation chapters.

That is enough foundation. Next comes the storage layout, where the real cable-tidy goblin work begins.
