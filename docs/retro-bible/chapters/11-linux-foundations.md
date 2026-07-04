# Linux Foundations

Linux is the foundation of the DAP Retro Bible stack.

It is flexible, scriptable and excellent for long-term retro gaming, preservation and homelab workflows. It also expects paths, permissions and package choices to be handled properly. A good Linux setup is not difficult, but it rewards structure.

This chapter covers the Linux basics that matter before building the rest of the retro stack.

## Why Linux Works Well

Linux is strong for retro gaming because it gives control over:

- filesystems;
- storage mounts;
- permissions;
- scripting;
- package sources;
- desktop applications;
- controllers;
- remote access;
- streaming workflows.

It works especially well when a library is shared across a desktop, server, NAS or living-room streaming setup.

## Distribution Choices

There is no single perfect distribution.

| Distribution | Strengths | Notes |
| --- | --- | --- |
| Ubuntu | Stable base and broad documentation | Good for homelab and service-heavy setups. |
| Nobara | Gaming-focused desktop | Useful for gaming laptops and modern GPU stacks. |
| Fedora | Modern packages | Good for newer desktop software. |
| Debian | Conservative and stable | Excellent for servers. |
| SteamOS-style systems | Console-like Steam interface | Best when Steam is the main launcher. |

The best distribution is the one you can maintain confidently.

## Package Types

Linux software may arrive as native packages, Flatpaks, AppImages or source builds.

| Package type | Strength | Watch for |
| --- | --- | --- |
| Native package | Good system integration | Version may lag. |
| Flatpak | Easy desktop installs | Sandbox permissions. |
| AppImage | Easy testing | Manual updates. |
| Source build | Maximum control | More maintenance. |

For the DAP Retro Bible, the package type should always be documented because it affects paths, updates and troubleshooting.

## Paths Matter

Use stable paths for the library.

Recommended base path:

```text
/mnt/games/
```

Avoid paths that change between sessions or users. A frontend is only as reliable as the path it points at.

A simple layout can be:

```text
/mnt/games/bios/
/mnt/games/roms/
/mnt/games/saves/
/mnt/games/states/
/mnt/games/patches/
/mnt/games/texture-packs/
```

## Permissions

Permissions are a common source of Linux trouble.

A retro stack may involve:

- the desktop user;
- frontend applications;
- emulator configuration folders;
- save folders;
- mounted storage;
- service users.

Good practice:

- keep game media readable;
- keep save folders writable;
- avoid root-owned files in user libraries;
- document ownership expectations;
- test the same launch path the player will use.

## Case Sensitivity

Linux filesystems are usually case-sensitive.

These are different filenames:

```text
Game.iso
game.iso
GAME.iso
```

This matters for patches, texture packs, scripts, metadata, ScummVM folders and emulator paths.

## Controllers

Controller handling depends on the emulator, desktop session and launch path.

Check:

- USB and Bluetooth behaviour;
- controller ordering;
- hotkeys;
- Steam Input behaviour;
- frontend launch behaviour;
- streaming input passthrough.

A controller working in one launcher does not guarantee it works everywhere.

## Audio, Display and GPU

Retro gaming on Linux may involve Wayland, X11, PipeWire, OpenGL, Vulkan and different GPU drivers.

Check:

- fullscreen behaviour;
- refresh rate;
- audio latency;
- shader performance;
- Vulkan support;
- OpenGL support;
- window focus after exit.

These details matter even more when using streaming.

## Real-World DAP Setup

A DAP-style setup may use:

- Ubuntu for services;
- Nobara or another gaming desktop for client play;
- `/mnt/games/` as shared library storage;
- Flatpak or AppImage for some emulators;
- Docker for web services;
- Sunshine and Moonlight for streaming;
- symlinks from frontend folders into the main library.

The strength of Linux is that these layers can work together. The danger is forgetting which layer owns which path.

## Common Mistakes

Common Linux mistakes include:

- using unstable mount paths;
- ignoring Flatpak permissions;
- mixing root-owned files into a user library;
- forgetting case sensitivity;
- testing only from a terminal;
- assuming one controller setup applies everywhere;
- backing up games but not saves or config.

## Troubleshooting

### Emulator cannot see the library

Check:

- path spelling;
- mount availability;
- file permissions;
- sandbox access;
- symlink targets.

### Saves are missing

Check:

- emulator save path;
- write permissions;
- user ownership;
- package-specific config folders;
- whether another launcher uses a different profile.

### Streaming behaves differently from local play

Check:

- fullscreen mode;
- controller passthrough;
- audio device;
- display session;
- GPU encoder;
- frontend launch command.

## Key Points

- Linux is powerful because it is explicit.
- Stable paths matter.
- Package type affects paths and permissions.
- Permissions should be designed, not guessed.
- Controllers, display and audio must be tested through the final launch path.
- A reliable setup is documented.

## What Comes Next

Next comes Docker for Retro Gaming: where containers fit, where they do not fit and how to keep paths sane when services join the library.