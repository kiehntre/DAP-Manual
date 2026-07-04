# HD Texture Packs

HD texture packs can make old games shine, but they can also make a library heavy, inconsistent and hard to support.

This draft chapter will explain where texture packs belong and how to keep them organised.

## Historical Context

- Explain texture replacement in emulators.
- Cover community packs and per-game projects.
- Note the difference between preservation and enhancement.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Texture dump | Extracted original textures. | Used by creators and tools. |
| Texture pack | Replacement assets. | Often game-specific. |
| Upscale | Higher-resolution generated texture. | Quality varies. |
| Load path | Emulator folder for replacement textures. | Usually emulator-specific. |

## Practical Setup

- Keep texture packs separate from game files.
- Record source, version and target emulator.
- Test performance before enabling globally.
- Back up custom changes.
- Avoid mixing packs without notes.

## Recommended Layout

```text
/mnt/games/texture-packs/
/mnt/games/texture-packs/gamecube/
/mnt/games/texture-packs/ps2/
/mnt/games/texture-packs/notes/
```

## Real-World DAP Setup

- Document which emulators load packs.
- Record symlink or copy strategy.
- Note streaming performance impact.

> **DAP Myth**
>
> Bigger textures are not automatically better. A consistent modest pack often looks cleaner than a huge uneven one.

## Common Mistakes

- Dropping packs into random emulator folders.
- Forgetting pack versions.
- Ignoring VRAM and storage cost.
- Mixing AI upscales with hand-made packs without testing.

## Troubleshooting

### Pack does not load

- Check game ID.
- Check folder name.
- Check emulator texture setting.
- Check pack format.

### Game stutters after enabling pack

- Check VRAM use.
- Check shader compilation.
- Lower internal resolution.
- Test without streaming.

## Key Points

- Texture packs are enhancements, not replacements for clean dumps.
- Version notes matter.
- Performance testing is part of the install.

## Further Reading

- Add emulator texture replacement documentation.
- Add project-specific texture pack sources.
