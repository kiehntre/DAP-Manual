# Proton

## Overview

Proton allows Windows games to run on Linux through Wine, DXVK, VKD3D and Steam integration.

## When to Use Proton

Use Proton for:

- Steam Windows games
- Non-Steam Windows games added to Steam
- Games streamed through Sunshine
- Games that benefit from Steam Input

## Recommended Versions

Start with:

1. Proton Experimental
2. Latest stable Proton
3. Proton-GE
4. Older Proton versions for stubborn games

## Non-Steam Game Pattern

1. Add the `.exe` to Steam.
2. Right click game.
3. Properties.
4. Compatibility.
5. Force a Proton version.
6. Launch once.
7. Adjust launch options if needed.

## Logs

```bash
PROTON_LOG=1 %command%
```

Logs usually appear in the home directory as:

```text
steam-APPID.log
```

## Common Fixes

| Symptom | First Checks |
|---|---|
| Game does not launch | Proton version, missing redistributables, path spaces |
| Black screen | Video codecs, fullscreen mode, Gamescope |
| No audio | PipeWire/PulseAudio, Wine audio config |
| Poor FPS | Vulkan driver, shader compilation, DXVK cache |
| Controller missing | Steam Input, udev, Bluetooth, Moonlight forwarding |

## DAP Notes

- Prefer Steam + Proton for non-Steam games when they are streamed through Sunshine.
- Use separate prefixes for messy games.
- Do not assume the newest Proton is always best.
