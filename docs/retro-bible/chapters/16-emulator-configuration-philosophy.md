# Emulator Configuration Philosophy

Emulator configuration is where a tidy retro setup can either become reliable or slowly turn into a drawer full of tiny mysteries.

Every emulator has settings. Some should apply globally. Some belong to a system. Some belong to one awkward game that needs special handling. The trick is knowing which is which.

This chapter sets the configuration philosophy for the DAP Retro Bible.

## The Main Rule

Use the broadest setting that is safe.

If a setting works well for the whole emulator, make it global. If it only fits one system, keep it at system level. If it fixes one game, keep it per-game and document why.

Do not make every setting per-game by habit. That creates a maintenance swamp.

## Configuration Layers

A useful model is:

| Layer | Use for |
| --- | --- |
| Global | Input defaults, save folders, general renderer choice, hotkeys. |
| System | BIOS path, system-specific controller layout, preferred file format. |
| Game | Compatibility fixes, special patches, unusual controller needs. |
| Frontend | Launch command, artwork, metadata and collection behaviour. |

Keeping these layers separate makes troubleshooting easier.

## Global Settings

Global settings should be safe defaults.

Examples:

- save path policy;
- screenshot location;
- controller hotkeys;
- fullscreen preference;
- shader default for simple systems;
- audio backend where stable;
- default renderer when broadly reliable.

A global setting should not silently break a group of games.

## System Settings

System settings belong to a platform rather than one title.

Examples:

- PS2 memory card behaviour;
- GameCube controller profile;
- Saturn BIOS region;
- Dreamcast VMU handling;
- MAME ROM set version;
- PSP rendering scale;
- ScummVM folder expectations.

These settings should be documented in system-specific chapters later.

## Per-Game Settings

Per-game settings are useful, but they should be treated like exceptions.

Good reasons for per-game settings include:

- compatibility fix;
- widescreen patch;
- unusual controller setup;
- performance workaround;
- game-specific texture pack;
- required renderer change;
- known timing issue.

Every per-game setting should have a reason.

> **DAP Tip**
>
> If a game needs a special setting, leave a note. A future fix may make the workaround unnecessary, but only if you remember why it existed.

## Frontend Configuration

Frontend configuration should launch the emulator correctly, not hide emulator problems.

A frontend may define:

- emulator command;
- platform association;
- artwork;
- metadata;
- collection membership;
- visible title;
- favourite status.

It should not be the only place where important emulator behaviour is understood.

## Save Paths

Save paths deserve special attention.

Track:

- native saves;
- memory cards;
- save states;
- cloud sync behaviour;
- emulator-specific save folders;
- Flatpak or AppImage path differences.

A beautiful configuration is not worth much if it loses saves.

## Controller Profiles

Controllers should be configured deliberately.

Useful questions:

- Is the profile global or system-specific?
- Does the system need analogue triggers?
- Does it need motion input?
- Does it need a keyboard overlay?
- Are hotkeys consistent?
- Does Steam Input change behaviour?
- Does streaming change controller order?

Retro games can be simple. Controller stacks often are not.

## Graphics Settings

Graphics settings can be global, system-specific or per-game.

Examples:

- renderer choice;
- internal resolution;
- aspect ratio;
- widescreen patches;
- texture filtering;
- shader presets;
- frame pacing;
- VSync.

Enhancements should be documented because they change the experience. That is fine, but the reader should know what changed.

## Accuracy vs Enhancement

Some settings improve accuracy. Others improve convenience or presentation.

Both are valid, but they should not be confused.

Examples:

| Setting type | Example |
| --- | --- |
| Accuracy | Correct BIOS, timing fix, native resolution. |
| Convenience | Save states, fast forward, rewind. |
| Presentation | Upscaling, shaders, widescreen. |
| Performance | Lower resolution, alternate renderer, frame skip. |

A good guide explains the trade-off.

## Configuration Notes

Every serious setup should keep notes.

Useful note locations:

```text
/mnt/games/docs/
/mnt/games/roms/<system>/_notes.md
/docs/retro-bible/chapters/
```

Notes do not need to be perfect. They need to exist.

## Real-World DAP Setup

A DAP-style setup should prefer:

- sane global defaults;
- system-specific profiles where useful;
- per-game fixes only when needed;
- documented frontend launch commands;
- stable save locations;
- tested streaming behaviour.

The goal is to make the setup reproducible, not magical.

## Common Mistakes

Common configuration mistakes include:

- changing global settings to fix one game;
- forgetting why a per-game setting exists;
- configuring the wrong emulator build;
- letting a frontend hide launch problems;
- losing saves during emulator migration;
- mixing Steam Input and emulator profiles without testing;
- assuming settings survive package changes.

## Troubleshooting

### One game is broken after a global change

Check:

- renderer;
- aspect ratio;
- timing options;
- controller profile;
- per-game overrides;
- emulator version.

### Settings changed but nothing happens

Check:

- correct emulator build;
- config path;
- frontend launch target;
- Flatpak path;
- portable mode;
- per-game override.

### Controller works in one game but not another

Check:

- system profile;
- game override;
- Steam Input;
- frontend launch path;
- controller order.

## Key Points

- Use the broadest safe setting.
- Keep global, system, game and frontend layers separate.
- Per-game settings should have reasons.
- Save paths and controller profiles need documentation.
- Enhancements are valid but should be labelled.
- Configuration should be reproducible.

## What Comes Next

Next comes Controller Configuration: the practical layer where pads, hotkeys, Steam Input and streaming all collide.