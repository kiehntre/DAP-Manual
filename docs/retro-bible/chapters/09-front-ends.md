# Front Ends

A frontend is the visible face of a retro library.

It lists systems, shows artwork, launches games and makes the whole setup feel like something designed rather than something dragged out of a folder by accident. A good frontend makes a clean library easier to enjoy. A bad setup can make a messy library look tidy while hiding problems underneath.

This chapter explains what frontends do, what they do not do and how they fit into the DAP Retro Bible stack.

## What a Frontend Does

A frontend usually handles:

- system lists;
- game lists;
- artwork;
- metadata;
- descriptions;
- launch entries;
- controller-friendly browsing;
- favourites and collections;
- themes.

A frontend is not the same thing as storage, verification or emulation.

> **DAP Myth**
>
> If a frontend can see a game, that does not mean the emulator can run it.

## The Source of Truth

The source of truth should be the organised library, not the frontend database.

A healthy setup should survive a frontend rebuild. If the database is deleted, the folder structure should still make sense. If one frontend is replaced with another, the ROMs, disc images, BIOS files, patches and saves should not become a mystery.

The frontend is a view. The library is the thing being viewed.

## Common Frontend Roles

| Frontend | Best role | Notes |
| --- | --- | --- |
| ES-DE | Sofa-friendly browsing | Strong for controller-first setups. |
| RomM | Web library browsing | Useful for server-style collections. |
| LaunchBox | Curated desktop library | Strong metadata, Windows-first history. |
| Pegasus | Custom launcher interface | Flexible but more manual. |
| Steam ROM Manager | Steam shortcut generation | Useful for selected games and streaming. |
| RetroArch playlists | RetroArch internal lists | Useful for core-based workflows. |

No frontend is the final answer for every system.

## ES-DE

ES-DE is a strong choice for couch-style browsing.

It works best when system folders are clear and emulator launch commands are known. It can scrape metadata and present a polished interface, but it still depends on the underlying files being organised.

Watch for:

- expected system folder names;
- emulator command configuration;
- scraper matches;
- controller setup;
- Flatpak permissions where relevant;
- file formats supported by the chosen emulator.

## RomM

RomM is useful for browsing a library through a web interface.

It fits naturally into a homelab or Docker-based setup. It can help with metadata, artwork and collection visibility, but it should sit on top of the same clean storage layer as the other tools.

RomM is a library management layer. It does not replace emulator configuration.

## LaunchBox

LaunchBox is a mature library manager with strong curation tools.

It is Windows-first, but it can still be relevant for Linux users who already have LaunchBox libraries or want its metadata workflow. If used on Linux, version and compatibility notes should be documented carefully.

Use it when the curation features matter. Do not assume it is the simplest path for every Linux setup.

## Steam ROM Manager

Steam ROM Manager is useful when selected retro games should appear in Steam.

This can help with:

- Big Picture style browsing;
- Steam artwork;
- Sunshine and Moonlight workflows;
- selected non-Steam shortcuts;
- controller profiles through Steam Input.

The safest approach is to test a small set first. Importing an entire library before the parser is right can create a splendid shortcut hedgehog.

## RetroArch Playlists

RetroArch playlists are useful inside RetroArch itself.

They are helpful for core-based workflows, but they are not a full replacement for an external frontend. They work best when the library paths are stable and the chosen cores are known.

## Metadata Is Not Verification

Frontends often scrape metadata from services such as ScreenScraper or artwork from sources such as SteamGridDB.

That metadata improves presentation. It does not prove that a file is correct.

A bad dump can have beautiful artwork. A wrong region can have the right screenshot. A hack can be matched as the original game. Presentation and verification are separate jobs.

## Real-World DAP Setup

A practical DAP-style setup may use several frontends at once:

- ES-DE for controller-first browsing;
- RomM for web-based library access;
- Steam ROM Manager for selected streamed games;
- standalone emulators for complex systems;
- RetroArch for selected older systems.

The important rule is that these tools should point back to the same organised source library where practical.

## Common Mistakes

Common frontend mistakes include:

- scraping before organising files;
- trusting artwork as proof of correctness;
- importing too many games into Steam at once;
- storing the only copy of games inside frontend folders;
- ignoring save locations;
- forgetting sandbox or permission limits;
- failing to document path changes.

## Troubleshooting

### A game appears but will not launch

Check:

- emulator selection;
- file format support;
- path access;
- BIOS or firmware requirements;
- launch arguments;
- frontend logs.

### Artwork is wrong

Check:

- scraper match;
- filename clarity;
- region label;
- duplicate entries;
- whether the game is a hack, translation or compilation.

### A game appears twice

Check:

- duplicate files;
- compressed and uncompressed copies;
- multi-disc helper files;
- old metadata cache;
- multiple scanned folders.

## Key Points

- A frontend is a view over the library.
- Storage remains the source of truth.
- Metadata improves browsing but does not verify files.
- Different frontends solve different problems.
- Test launches from the frontend, not only the emulator.
- Keep path, save and metadata behaviour documented.

## What Comes Next

Next comes storage architecture: how to organise folders, mounts, saves, BIOS files and enhancements so the setup can grow without turning into spaghetti.