# Preface

The DAP Retro Bible is a Linux-first book about retro gaming, emulation, preservation and the practical work needed to keep old games playable.

It is not a quick-start pamphlet. It is not a list of random emulator links. It is meant to become a long-lived reference for people who want to build a clean, understandable and maintainable retro setup without drowning in folklore.

Retro gaming has a strange habit of looking simple from the outside. A person sees a game running and thinks the job is done. Underneath that single launch, however, there may be a disc image, a verified dump, a BIOS file, a controller profile, a frontend entry, a metadata scrape, a shader preset, a save folder, a patch, a texture pack and a streaming session. When those parts are tidy, the experience feels effortless. When they are not, the whole thing becomes a drawer full of cables.

This book exists to make the hidden structure visible.

## What This Book Covers

The DAP Retro Bible covers the full stack around retro gaming on modern Linux systems:

- preservation projects and verified sets;
- ROM images, disc images and archive formats;
- BIOS and firmware organisation;
- emulators and libretro cores;
- frontends such as ES-DE, LaunchBox, RomM and related tools;
- Linux desktop, Docker and Flatpak workflows;
- Steam, Proton, Sunshine and Moonlight integration;
- HD texture packs, widescreen patches, 60 FPS patches and mods;
- metadata, artwork and library presentation;
- digital store closures, lost services and preservation history.

The technical chapters are practical. The historical chapters explain why the work matters. Both belong together.

## Linux First

This book starts from Linux because Linux is where much of the DAP setup lives. The examples favour Ubuntu, Nobara, Docker, Flatpak, Steam, Proton, Sunshine, Moonlight and network-backed storage.

That does not mean other platforms are ignored. Windows tools still matter. Some frontends, dumpers and utilities are easier to use outside Linux. The rule is not purity. The rule is usefulness.

If a tool works well on Linux, it belongs here. If a tool needs Wine, Proton or another workaround, the workaround should be explained honestly. If a tool is better handled elsewhere, the book should say so rather than pretending every road must lead through a terminal.

## Preservation, Not Piracy

Preservation is often discussed badly. Some people use it as a polite mask for downloading anything they can find. Others treat the word as suspicious by default. Neither approach is useful.

This book takes a practical and careful position:

- preserve what you own or have the right to archive;
- understand what verified dumps and checksums are for;
- do not confuse metadata scraping with game preservation;
- do not treat random internet archives as automatically trustworthy;
- respect the legal and ethical difference between documentation and distribution.

The book may discuss tools, formats and preservation projects. It will not provide copyrighted games, BIOS files, firmware, keys or infringing links.

## The DAP Approach

The DAP approach is simple: build the library as if future-you has to maintain it at 2 a.m. after something broke.

That means:

- predictable folder structures;
- plain names where possible;
- verified files where practical;
- documented emulator choices;
- frontends treated as views, not the source of truth;
- backups before destructive conversions;
- real troubleshooting notes rather than clean-room fantasy.

A setup that only works because one person remembers a dozen hidden fixes is not finished. It is a trap with a nice theme song.

## How to Read This Book

You do not need to read every chapter in order. A reader setting up Dolphin can jump to the GameCube section. A reader sorting disc images can jump to file formats. A reader interested in history can start with the Museum chapters.

However, the early chapters matter. They define the language of the book: preservation, metadata, BIOS, firmware, frontends, archive formats and real-world Linux paths. Those ideas return throughout the later sections.

If you are building from scratch, read Part I and Part II first. They will save time later.

## The Goal

The goal is not to create the biggest retro gaming guide on the internet. Size is easy. Quality is harder.

The goal is to create a reference that is:

- accurate enough to trust;
- practical enough to use;
- structured enough to maintain;
- historical enough to explain why the work matters;
- honest enough to admit when a tool has limits.

If this book does its job, it should help a reader build a better retro setup and understand the preservation work behind it.

That is the standard. Now we start laying bricks.