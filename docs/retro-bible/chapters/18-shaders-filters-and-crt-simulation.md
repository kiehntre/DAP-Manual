# Shaders, Filters and CRT Simulation

Modern screens are brutally sharp. Old games were not designed for them.

A sprite that looked rich on a CRT can look blocky, thin or oddly harsh on a modern LCD or OLED. Shaders and filters help bridge that gap. They can recreate scanlines, shadow masks, glow, blur, curvature and colour behaviour. Used well, they make old games feel closer to their original presentation. Used badly, they turn the image into soup.

This chapter explains the basic philosophy.

## The Main Rule

Enhance the image without destroying the artwork.

A shader should support the game, not become the main character. If the effect is the first thing you notice, it may be too strong.

## Filters vs Shaders

Filters and shaders are often discussed together, but they are not identical.

| Term | Meaning |
| --- | --- |
| Filter | A simpler image processing effect, often scaling or smoothing. |
| Shader | A GPU effect that can simulate display behaviour or alter the rendered image. |
| CRT shader | A shader designed to imitate CRT display characteristics. |
| Upscaling | Rendering or scaling the image at a higher resolution. |

The exact terminology depends on the emulator or frontend.

## Why CRT Simulation Matters

Many retro games were created with CRT displays in mind.

CRT displays affected:

- colour blending;
- pixel edges;
- dithering;
- transparency tricks;
- scanline appearance;
- perceived brightness;
- motion clarity;
- aspect ratio.

A raw pixel-perfect image is useful, but it is not always how the art was meant to be seen.

## Pixel-Perfect Output

Pixel-perfect output aims to preserve the original pixel structure.

Strengths:

- sharp image;
- clear scaling;
- useful for handheld and pixel-art fans;
- minimal processing.

Weaknesses:

- can look harsh on large displays;
- dithering may look obvious;
- some artwork loses intended blending.

Pixel-perfect is valid. It is just not the only valid answer.

## CRT Shaders

CRT shaders can simulate parts of the old display chain.

Common features include:

- scanlines;
- phosphor glow;
- mask patterns;
- curvature;
- colour bleed;
- deconvergence;
- softening;
- bloom.

Subtlety matters. A good CRT shader should feel natural from the sofa, not like someone laid a barbecue grill over the screen.

## Scaling

Scaling determines how the original image fits a modern display.

Important ideas:

- integer scaling keeps pixels evenly sized;
- non-integer scaling may fill more screen space but can shimmer or blur;
- aspect ratio should respect the original system unless intentionally changed;
- handheld screens may need different choices from TVs.

For many systems, a balanced approach is better than chasing a single universal setting.

## Widescreen and Aspect Ratio

Widescreen patches and aspect corrections are separate from shaders.

A shader changes presentation. A widescreen patch changes what the game renders or how it is displayed.

Do not confuse:

- stretching 4:3 to 16:9;
- proper widescreen patching;
- emulator aspect correction;
- bezel or border artwork.

Stretching everything to widescreen is easy. Making it look right is harder.

## Emulator Differences

Different emulators handle shaders differently.

Examples:

- RetroArch has a large shader ecosystem;
- standalone emulators may have their own filters;
- Dolphin focuses more on internal resolution and enhancements;
- PCSX2 can combine upscaling with texture filtering and patches;
- MAME has its own video options and artwork support.

System chapters should document the best practical choices.

## Performance Cost

Shaders use GPU resources.

A light shader may be cheap. A complex CRT shader at high resolution can be expensive, especially when streaming or running demanding emulators.

Check:

- GPU load;
- frame pacing;
- latency;
- streaming encode performance;
- client display resolution.

A perfect shader is useless if it makes the game stutter.

## Real-World DAP Setup

A DAP-style setup should use different presentation defaults per class of system.

Possible approach:

| System type | Suggested approach |
| --- | --- |
| 8-bit and 16-bit consoles | CRT shader or clean integer scaling. |
| Handhelds | LCD-style shader or integer scaling. |
| Arcade | MAME-style display tuning per game type. |
| 3D consoles | Internal resolution and aspect handling first. |
| Computer systems | Respect original monitor style where possible. |

Do not force one visual style across the entire library.

## Common Mistakes

Common mistakes include:

- using a heavy shader on every system;
- stretching 4:3 games to widescreen;
- confusing upscaling with preservation;
- making scanlines too dark;
- ignoring handheld display differences;
- forgetting performance when streaming;
- applying filters before testing the raw image.

## Troubleshooting

### Image looks too dark

Check:

- shader brightness;
- scanline strength;
- display HDR or SDR mode;
- emulator colour settings;
- TV picture mode.

### Image shimmers while scrolling

Check:

- scaling mode;
- integer scaling;
- refresh rate;
- shader preset;
- emulator output resolution.

### Game stutters with shader enabled

Check:

- GPU load;
- shader complexity;
- output resolution;
- streaming encoder load;
- emulator renderer.

## Key Points

- Old games were designed around old displays.
- Pixel-perfect is useful but not always historically faithful.
- CRT shaders should be subtle.
- Scaling and aspect ratio matter.
- Widescreen patches are not the same as stretching.
- Shader choices should be system-specific.
- Performance still matters.

## What Comes Next

Next comes Save Files, Save States and Backups: the part of the setup that matters most when something goes wrong.