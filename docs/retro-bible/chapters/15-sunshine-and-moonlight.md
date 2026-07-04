# Sunshine and Moonlight

Sunshine and Moonlight turn a Linux gaming machine into something that can be played from the sofa, laptop, TV or handheld.

For the DAP Retro Bible, streaming matters because the best retro setup is not always sitting under the television. The library may live on a server. The GPU may be in another room. The frontend may run on a Linux desktop while the player uses a lightweight client elsewhere.

This chapter explains the role of Sunshine and Moonlight in a Linux-first retro stack.

## The Basic Idea

Sunshine is the host.

Moonlight is the client.

The host runs the game. The client receives video and audio, then sends controller input back. If the network is strong and the host is configured well, the experience can feel close to local play.

## Why Streaming Helps

Streaming is useful when:

- the powerful machine is not near the TV;
- a laptop or handheld is more convenient;
- the same library should be played in several rooms;
- the setup uses one main GPU host;
- emulators are easier to maintain on one machine;
- couch play matters.

It also lets a Linux desktop act like a console without moving the whole machine.

## What Needs to Work

A streamed emulator session needs more than the game booting.

The final setup must handle:

- video encoding;
- audio capture;
- controller input;
- fullscreen behaviour;
- frontend launching;
- emulator exit;
- save paths;
- network stability.

A game that works from a terminal is not automatically streaming-ready.

## Host Requirements

The host should have:

- a working desktop session;
- GPU drivers with hardware encoding support;
- stable network connectivity;
- configured audio output;
- emulators that launch cleanly;
- a frontend or app list suitable for remote use.

For demanding systems, host performance matters. Streaming does not make a slow emulator faster. It only moves the display and input somewhere else.

## Client Requirements

The client should have:

- Moonlight installed;
- controller support;
- a stable network connection;
- suitable display resolution;
- low-latency decoding;
- sensible audio output.

A wired network is ideal. Strong Wi-Fi can work well, but bad Wi-Fi turns streaming into interpretive dance.

## Frontend Launching

Sunshine can launch individual applications or a frontend.

Useful launch targets include:

- ES-DE;
- Steam Big Picture;
- selected emulator shortcuts;
- selected PC games;
- scripts that prepare the environment.

For a large retro library, launching a frontend is usually cleaner than listing every game individually.

## Controller Behaviour

Controller behaviour should be tested through the full path:

```text
Controller -> Moonlight client -> Sunshine host -> frontend -> emulator -> game
```

Each layer can affect input.

Common issues include:

- wrong controller order;
- duplicate input;
- missing hotkeys;
- Steam Input conflicts;
- emulator profile mismatch;
- controller not detected after reconnect.

## Audio and Display

Streaming depends heavily on audio and display behaviour.

Check:

- correct audio device;
- fullscreen mode;
- desktop resolution;
- refresh rate;
- HDR expectations;
- window focus;
- mouse cursor visibility.

Some emulators behave differently in borderless window, exclusive fullscreen or normal windowed mode.

## Network Notes

A good network makes streaming feel invisible.

Important factors:

- latency;
- packet loss;
- Wi-Fi quality;
- wired backhaul;
- bitrate;
- client decoding speed;
- host encoding performance.

For retro games, latency can matter more than raw image quality. Platformers, shooters, rhythm games and fighters expose lag quickly.

## Real-World DAP Setup

A DAP-style setup may use:

- Linux host with GPU acceleration;
- Sunshine on the host;
- Moonlight on a laptop, TV device or handheld;
- ES-DE for couch-friendly browsing;
- Steam for selected PC and non-Steam games;
- `/mnt/games/` as the shared library source.

The goal is a setup that can be launched from the client without walking back to the host to fix focus, audio or controller state.

## DAP Tip

> **DAP Tip**
>
> Test the exact route you intend to use. If the goal is Moonlight into ES-DE into Dolphin, testing Dolphin alone on the host is only half the job.

## Common Mistakes

Common mistakes include:

- testing locally but not through Moonlight;
- ignoring controller order;
- using unstable Wi-Fi;
- forgetting audio device selection;
- relying on terminal-only launch commands;
- not configuring a clean exit path;
- assuming every emulator handles fullscreen the same way.

## Troubleshooting

### Game launches but controller does not work

Check:

- Moonlight controller detection;
- Sunshine input settings;
- frontend input mapping;
- emulator controller profile;
- Steam Input interaction.

### Audio is missing

Check:

- host audio device;
- Sunshine audio capture;
- muted desktop session;
- client audio output;
- emulator audio backend.

### Stream is laggy

Check:

- wired vs wireless connection;
- bitrate;
- host encoder load;
- client decode performance;
- network congestion;
- emulator performance on the host.

## Key Points

- Sunshine is the host and Moonlight is the client.
- Streaming needs video, audio, input and launch behaviour to work together.
- Frontends are often better launch targets than individual games.
- Controller testing must include the whole chain.
- Network quality matters more than marketing numbers.
- A streaming-ready setup must launch, play, save and exit cleanly.

## What Comes Next

Next comes Emulator Configuration Philosophy: how to decide what should be global, what should be per-system and what should be per-game.