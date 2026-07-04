# Widescreen Hacks and Patches

Widescreen can be lovely when it is done properly. Stretching a 4:3 image is not that.

This draft chapter will explain widescreen patches, emulator settings and the difference between a better view and a broken one.

## Historical Context

- Explain 4:3 design assumptions.
- Cover emulator widescreen hacks and game patches.
- Note HUD, culling and cutscene issues.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Aspect ratio | Shape of the displayed image. | 4:3 and 16:9 are common. |
| Stretching | Scaling without correction. | Usually wrong for preservation. |
| Widescreen hack | Emulator-side adjustment. | Can reveal missing geometry. |
| Patch | Game-specific modification. | Often more accurate, but still needs testing. |

## Practical Setup

- Start from correct original aspect ratio.
- Test emulator widescreen options.
- Prefer known game-specific patches where appropriate.
- Check menus, HUD, cutscenes and gameplay.
- Keep patch notes beside the game.

## Recommended Layout

```text
/mnt/games/patches/widescreen/
/mnt/games/patches/notes/
```

## Real-World DAP Setup

- Document which systems use widescreen by default.
- Record games with per-title exceptions.
- Note streaming display behaviour.

> **DAP Warning**
>
> Widescreen hacks can look fine for five minutes and then break a boss fight, menu or cutscene. Test the boring parts too.

## Common Mistakes

- Stretching every game to fill the TV.
- Enabling global widescreen hacks.
- Forgetting HUD alignment.
- Mixing patches with incompatible game revisions.

## Troubleshooting

### Objects disappear at screen edges

- Disable the hack.
- Check game-specific patch notes.
- Test original aspect ratio.

### HUD is stretched or misplaced

- Check patch compatibility.
- Check emulator aspect setting.
- Check texture pack interaction.

## Key Points

- Widescreen is a presentation choice, not a universal upgrade.
- Proper patches beat blind stretching.
- Per-game notes save repeated testing.

## Further Reading

- Add emulator widescreen documentation.
- Add patch repositories and compatibility notes.
