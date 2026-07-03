# Choosing an Emulator

Choosing an emulator is not only a question of which one launches a game first.

A good emulator choice balances accuracy, compatibility, performance, Linux support, controller handling, save behaviour, frontend integration and long-term development. Sometimes the best emulator for one game is not the best emulator for the whole system. Sometimes the standalone emulator is the right choice. Sometimes a RetroArch core is convenient enough.

This chapter explains how to choose deliberately.

## The Decision Factors

When choosing an emulator, consider:

| Factor | Why it matters |
| --- | --- |
| Accuracy | Does it behave like the original system? |
| Compatibility | How much of the library works? |
| Performance | Can your hardware run it well? |
| Linux support | Is Linux a first-class platform or an afterthought? |
| Controller handling | Can pads, hotkeys and profiles be managed cleanly? |
| File format support | Does it read the formats used in your library? |
| BIOS and firmware handling | Are requirements clear and documented? |
| Frontend integration | Can ES-DE, RomM, LaunchBox or Steam launch it reliably? |
| Maintenance | Is the project active and trustworthy? |

No emulator is best at everything.

## Accuracy vs Playability

Accuracy means the emulator behaves like the original hardware as closely as possible.

Playability means the game is enjoyable to play.

These overlap, but they are not identical. A highly accurate emulator may need more CPU power. A faster emulator may rely on shortcuts that break unusual games. A player may prefer widescreen, upscaling, save states and reduced latency even when those features go beyond the original machine.

The DAP Retro Bible does not treat enhancements as cheating. It treats them as choices that should be documented.

> **DAP Tip**
>
> Keep a note of any non-default settings used for a system. Future-you will not remember why one game needed a strange renderer option.

## Standalone vs RetroArch Core

Many systems can be emulated through standalone emulators or RetroArch cores.

| Choice | Strengths | Weaknesses |
| --- | --- | --- |
| Standalone emulator | Often best features, latest compatibility and clearer system-specific settings | More separate interfaces to manage |
| RetroArch core | Unified interface, shaders, hotkeys and controller profiles | Core versions may lag and settings can be confusing |

Standalone emulators are often preferred for complex systems such as GameCube, Wii, PS2, PS3, Wii U and original Xbox.

RetroArch cores can be excellent for many older cartridge systems and some CD systems, especially where unified controller handling matters.

## Active Development Matters

An emulator does not need daily commits to be useful, but activity matters.

Signs of a healthy project include:

- recent releases;
- readable documentation;
- issue tracker activity;
- clear build instructions;
- Linux packages or AppImages where appropriate;
- transparent compatibility notes;
- responsible project communication.

Signs of concern include:

- abandoned builds;
- no documentation;
- closed binaries with no clear source;
- dramatic compatibility claims with no evidence;
- forks that mainly exist to bundle questionable extras.

## Linux Support

For this book, Linux support is central.

A Linux-friendly emulator should ideally provide at least one of:

- native packages;
- AppImage;
- Flatpak;
- source builds with clear instructions;
- documented dependencies;
- controller support through SDL, evdev or similar systems;
- good behaviour under Wayland or X11.

Wine or Proton can still be useful, but they should be treated as compatibility layers, not invisible magic.

## File Format Support

The emulator must support the library's working format.

Examples:

- Dolphin works well with RVZ for GameCube and Wii.
- Many CD-based systems work well with CHD in modern emulators.
- PSP workflows often use ISO or CSO.
- Arcade sets must match MAME expectations.
- ScummVM usually prefers game folders rather than generic archives.

Do not assume a frontend seeing the file means the emulator can run it.

## Controller Support

Controller handling can make or break a living-room setup.

Check:

- whether the emulator detects controllers reliably;
- whether profiles can be saved per system;
- whether hotkeys can be configured;
- whether multiple controllers work cleanly;
- whether Steam Input helps or hurts;
- whether Sunshine and Moonlight pass input correctly.

For systems with unusual controls, such as Wii, DS, light gun games or arcade cabinets, controller planning is part of emulator selection.

## Frontend Integration

A good emulator for manual launching may still be awkward in a frontend.

Consider:

- command-line launch support;
- fullscreen flags;
- exit hotkeys;
- per-game configuration;
- save paths;
- portable configuration;
- Flatpak application IDs;
- whether the emulator returns focus correctly after launch.

A sofa setup needs more than compatibility. It needs clean launch and clean exit.

## DAP Gold Standard

For the DAP Retro Bible, a recommended emulator should ideally be:

- actively maintained;
- open source where possible;
- available on Linux;
- documented;
- compatible with sensible archive formats;
- friendly to frontends;
- clear about BIOS or firmware requirements;
- usable without hidden bundled system files.

If an emulator fails one of these tests, it may still be useful, but the limitation should be stated plainly.

## Example Recommendations

These are starting points, not final law.

| System | Likely first choice | Notes |
| --- | --- | --- |
| GameCube / Wii | Dolphin | Strong Linux support and RVZ workflow. |
| PlayStation 2 | PCSX2 | Mature and essential for PS2. |
| PlayStation 3 | RPCS3 | Complex but impressive. Updates matter. |
| Original Xbox | xemu | Requires careful setup and XISO handling. |
| Wii U | Cemu | WUA is useful where supported. |
| PSP | PPSSPP | Strong compatibility and CSO/ISO support. |
| Arcade | MAME | Versioned sets matter. |
| Adventure games | ScummVM | Engine-based approach, folder layout important. |
| Older cartridge systems | RetroArch cores or standalone | Depends on frontend and controller preference. |

System-specific chapters should refine these recommendations.

## Real-World DAP Setup

For DAP-style use, emulator choice should account for:

- ES-DE launching;
- RomM library browsing;
- Sunshine and Moonlight streaming;
- network-backed storage;
- controller profiles;
- save location clarity;
- Flatpak permissions where used;
- GPU driver behaviour.

An emulator is not considered fully integrated until it launches from the frontend, accepts controller input, exits cleanly and stores saves somewhere documented.

## Common Mistakes

Common mistakes include:

- choosing an emulator only because it appears in a frontend list;
- using an abandoned emulator out of habit;
- assuming RetroArch cores are always worse or always better;
- ignoring file format support;
- forgetting BIOS requirements;
- failing to test controller hotkeys;
- testing only from the desktop, not through the frontend;
- ignoring save paths until after something is lost.

## Troubleshooting

### Game runs manually but not from frontend

Possible causes:

- wrong command-line arguments;
- spaces in file paths;
- Flatpak permissions;
- frontend passing an archive the emulator cannot read;
- working directory issue;
- missing fullscreen flag.

Fix:

- copy the exact launch command and test it in a terminal;
- quote paths correctly;
- test with a simple file path;
- confirm emulator can read the format;
- check frontend logs.

### Controller works in Steam but not emulator

Possible causes:

- Steam Input intercepting the pad;
- emulator using a different input backend;
- controller profile not saved;
- Moonlight input mapping issue;
- Flatpak device permission issue.

Fix:

- test outside Steam;
- test inside Steam;
- save emulator profiles;
- check Sunshine and Moonlight input settings;
- confirm Flatpak permissions.

## Key Points

- Choose emulators deliberately.
- Accuracy, performance and usability all matter.
- Standalone and RetroArch workflows both have a place.
- Linux support is central to this book.
- File format support must match the library.
- Frontend integration should be tested, not assumed.
- A recommended emulator should be maintainable and documented.

## What Comes Next

Next comes frontends: the library interfaces that make a clean storage layer usable from the desktop, sofa, handheld or browser.