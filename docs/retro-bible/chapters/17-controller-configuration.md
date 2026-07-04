# Controller Configuration

Controllers are where a retro setup either feels like a console or reminds you that computers enjoy practical jokes.

A game can launch perfectly and still feel wrong if the controller order changes, hotkeys collide, analogue triggers fail or Steam Input quietly decides to help a bit too much. Controller configuration deserves its own chapter because it sits across several layers: Linux, the frontend, the emulator, Steam, streaming and the game itself.

This chapter sets the basic controller strategy for the DAP Retro Bible.

## The Main Rule

Test the controller through the final launch path.

If the goal is Moonlight into Sunshine, into ES-DE, into Dolphin, then testing Dolphin alone on the host is not enough. Every layer can change input behaviour.

## Controller Layers

A typical stack may involve:

```text
Controller
Moonlight client
Sunshine host
Steam Input or desktop input
Frontend
Emulator
Game
```

Each layer can remap, reorder or ignore input.

## Common Controller Types

Common options include:

- Xbox controllers;
- PlayStation controllers;
- 8BitDo controllers;
- arcade sticks;
- fight pads;
- keyboard encoders;
- handheld built-in controls;
- original controller adapters.

Modern Xbox-style controllers are usually the easiest baseline. PlayStation pads are excellent too, but button labels and layouts may need attention.

## Global vs System Profiles

Use global controller defaults where they make sense.

Use system profiles when the original machine had a specific layout.

Examples:

| System | Controller consideration |
| --- | --- |
| GameCube | Analogue triggers and unusual button layout. |
| Wii | Motion, pointer and extension controllers. |
| N64 | Six face buttons, analogue stick and C buttons. |
| Saturn | Six-button layout matters for many games. |
| Dreamcast | VMU and trigger behaviour. |
| Arcade | Different layouts per game genre. |

A single modern pad can cover many systems, but not all systems map cleanly.

## Hotkeys

Hotkeys are essential for couch play.

Common hotkeys include:

- exit game;
- save state;
- load state;
- pause;
- fast forward;
- screenshot;
- open emulator menu;
- reset game.

Hotkeys should be consistent where possible, but not at the cost of breaking normal gameplay.

> **DAP Warning**
>
> Do not bind dangerous actions such as reset or load state to easy accidental button combinations. Nothing says romance like deleting progress during a boss fight.

## Steam Input

Steam Input can help or interfere.

It is useful for:

- per-game profiles;
- controller remapping;
- gyro support;
- radial menus;
- desktop control;
- Big Picture workflows.

It can cause confusion when an emulator also handles the controller directly. If input is doubled, missing or oddly mapped, test with Steam Input enabled and disabled.

## Streaming Controllers

Streaming adds another layer.

When using Sunshine and Moonlight, the client controller is passed back to the host. The host then presents that controller to the frontend or emulator.

Test:

- controller order;
- reconnect behaviour;
- hotkeys;
- analogue triggers;
- rumble;
- Steam Input interaction;
- multiple players.

Four-player retro sessions are where bad assumptions go to be publicly humiliated.

## Frontend Mapping

Frontends need their own navigation mapping.

Frontend controls should cover:

- up, down, left, right;
- confirm;
- back;
- menu;
- favourites;
- quit or shutdown actions where appropriate.

Do not assume frontend mapping and emulator mapping are the same. They usually live in different configuration layers.

## Emulator Profiles

Each emulator may store controller profiles differently.

Track:

- where profiles live;
- whether profiles are global or per-game;
- whether the emulator uses SDL or another input backend;
- whether profiles survive updates;
- whether Flatpak paths differ from native paths.

Controller profiles should be part of the backup plan.

## Special Controllers

Some systems need special handling.

Examples:

- Wii pointer and motion controls;
- DS touch input;
- light gun games;
- racing wheels;
- arcade spinners;
- trackballs;
- keyboard-heavy computer games.

These should be handled in system-specific chapters later. This chapter only sets the general strategy.

## Real-World DAP Setup

A DAP-style setup should prefer:

- one reliable default controller;
- documented system-specific profiles;
- tested hotkeys;
- Steam Input only where useful;
- Moonlight testing for streamed play;
- backups of controller profiles.

The test is simple: can someone sit down, launch a game, play it, save it and exit without needing a keyboard?

## Common Mistakes

Common mistakes include:

- configuring the emulator but not the frontend;
- testing locally but not through Moonlight;
- allowing controller order to change;
- forgetting analogue trigger behaviour;
- binding hotkeys dangerously;
- relying on Steam Input without documenting it;
- failing to back up controller profiles.

## Troubleshooting

### Controller works in frontend but not game

Check:

- emulator profile;
- controller order;
- input backend;
- Steam Input;
- game-specific settings.

### Controller works locally but not streamed

Check:

- Moonlight client detection;
- Sunshine host input;
- Steam Input layer;
- emulator profile;
- reconnect behaviour.

### Buttons are doubled or wrong

Check:

- Steam Input and emulator both remapping;
- duplicate devices;
- wrong controller profile;
- frontend launching a different emulator build.

## Key Points

- Test through the final launch path.
- Controller input crosses many layers.
- Use global defaults only where safe.
- System-specific profiles matter.
- Hotkeys should be consistent and safe.
- Steam Input is useful but must be documented.
- Controller profiles belong in the backup plan.

## What Comes Next

Next comes Shaders, Filters and CRT Simulation: making old games look good on modern screens without turning every pixel into melted jam.