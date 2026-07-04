# Steam, Proton and Non-Steam Games

Steam is not only a store on Linux. It can also act as a launcher, controller layer and streaming hub.

For the DAP Retro Bible, Steam matters because it connects several useful workflows: Proton for Windows games, Steam Input for controllers, non-Steam shortcuts for emulators and Sunshine/Moonlight style living-room play.

This chapter explains where Steam fits without letting it take over the whole retro library.

## The Role of Steam

Steam can provide:

- game launching;
- Proton compatibility;
- controller profiles;
- Big Picture mode;
- non-Steam shortcuts;
- artwork presentation;
- launch options;
- remote play and streaming workflows.

Steam is useful, but it should not become the source of truth for the library. The organised storage layer should remain independent.

## Proton Basics

Proton is Valve's compatibility layer for running many Windows games on Linux.

It combines Wine with supporting technologies and Steam integration. For many PC games, Proton is the simplest way to play on Linux.

Good uses:

- Windows PC games in Steam;
- selected non-Steam games;
- games that need controller profiles;
- games launched from Big Picture or a streaming session.

Watch for:

- per-game prefixes;
- launcher dependencies;
- anti-cheat compatibility;
- graphics API translation;
- save paths;
- version-specific regressions.

## Proton Versions

Different Proton versions can behave differently.

Common choices include:

| Version type | Use case |
| --- | --- |
| Proton Stable | Default choice for most games. |
| Proton Experimental | Useful for newer fixes. |
| GE-Proton | Community build with extra patches and media support. |
| Older Proton versions | Useful when a game regresses. |

Do not assume the newest version is always best. Test the game that matters.

## Non-Steam Games

Steam can launch applications that were not bought through Steam.

This can be useful for:

- standalone emulators;
- Windows games installed outside Steam;
- launch scripts;
- selected retro games;
- frontend shortcuts;
- Sunshine/Moonlight launch targets.

The key is to keep the shortcut clear and documented.

A non-Steam shortcut should record:

- target application;
- launch arguments;
- working directory;
- compatibility tool if used;
- controller profile;
- artwork source.

## Steam ROM Manager

Steam ROM Manager can generate Steam shortcuts from emulation libraries.

It is powerful, but it should be used carefully.

Good practice:

- test with a small system first;
- confirm artwork matches;
- check launch commands;
- avoid importing a huge library before testing;
- keep parsers documented;
- remove bad shortcuts before generating more.

> **DAP Tip**
>
> Steam ROM Manager is best for selected favourites, handheld workflows or streaming targets. It does not need to contain every game in the library.

## Steam Input

Steam Input can be excellent for controller handling.

It can help with:

- controller profiles;
- button remapping;
- radial menus;
- gyro controls;
- per-game layouts;
- couch play.

It can also confuse things if an emulator already has its own controller mapping. If controls behave strangely, test with Steam Input enabled and disabled.

## Artwork

Non-Steam games look better with proper artwork.

Artwork can include:

- grid image;
- hero image;
- logo;
- icon;
- banner.

SteamGridDB is commonly used for this. Artwork improves presentation, but it does not organise or verify the library.

## Prefixes and Saves

Each Proton game may use a compatibility prefix.

A prefix contains a Windows-like environment for that game. Save files and configuration may live inside it, while cloud saves may or may not apply.

For non-Steam games, record where the prefix lives if the game matters. Losing a prefix can mean losing settings or saves.

## Real-World DAP Setup

A DAP-style setup can use Steam selectively.

Recommended pattern:

- keep the main retro library under `/mnt/games/`;
- use ES-DE or RomM for broad browsing;
- use Steam for selected favourites and PC games;
- use Steam ROM Manager only after parser testing;
- use Steam Input where it improves controller behaviour;
- keep notes for non-Steam launch targets.

Steam should be a polished doorway, not the whole house.

## Common Mistakes

Common mistakes include:

- importing an entire ROM library into Steam too early;
- forgetting which Proton version a game needs;
- losing saves inside Proton prefixes;
- mixing Steam Input and emulator input without testing;
- assuming artwork means the shortcut is correct;
- using Steam as the only record of launch commands.

## Troubleshooting

### Game launches from desktop but not Steam

Check:

- target path;
- working directory;
- launch arguments;
- Proton version;
- permissions;
- quoted paths.

### Controller mapping is wrong

Check:

- Steam Input profile;
- emulator controller profile;
- controller order;
- Big Picture behaviour;
- streaming input path.

### Game worked before but now fails

Check:

- Proton version changed;
- game update;
- prefix corruption;
- missing launcher dependency;
- graphics driver update.

## Key Points

- Steam is useful as a launcher and controller layer.
- Proton is powerful but version-sensitive.
- Non-Steam shortcuts should be documented.
- Steam ROM Manager should be tested with small batches first.
- Steam Input can help or interfere.
- The main library should remain independent of Steam.

## What Comes Next

Next comes Sunshine and Moonlight: streaming the finished setup to laptops, TVs and handhelds without making every launch a little ritual.