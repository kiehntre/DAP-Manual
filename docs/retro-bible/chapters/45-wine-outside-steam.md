# Wine Outside Steam

Wine is older, broader and less hand-holding than Proton. It is also still essential for some launchers, editors, Windows tools and older PC games.

This chapter is a skeleton for using Wine in a Linux-first retro setup without letting prefixes become mystery cupboards.

## Historical Context

Wine made Windows compatibility on Unix-like systems possible long before Proton made it fashionable for Steam users. Retro preservation still leans on it because many useful tools were written for Windows and never received Linux ports.

## Practical Advice

- Use one prefix per important tool or game group.
- Name prefixes clearly.
- Record Wine version and helper tools.
- Back up prefixes that contain saves, licences or configuration.
- Prefer native Linux tools when they are mature and easier to maintain.

## Recommended Layout

```text
/mnt/games/wine-prefixes/
/mnt/games/wine-tools/
/mnt/games/backups/wine/
```

> **DAP Tip**
>
> A Wine prefix is part application, part fake Windows machine and part memory box. Label it like something you will need to restore later.

## Common Mistakes

- Installing unrelated tools into one giant prefix.
- Deleting a prefix without checking for saves.
- Forgetting 32-bit dependencies.
- Mixing Lutris, Bottles and manual Wine commands without notes.

## Troubleshooting

### Tool opens but cannot see mounted games

Check Wine drive mappings, Linux permissions and whether the path contains spaces or unusual characters.

### Game worked once and now fails

Check prefix changes, Wine version, installed runtimes and whether the game wrote broken config.

## DAP Warning

Wine is powerful, but it is not magic. When it matters, document the prefix like you would document an emulator.

## What Comes Next

Next comes remote gaming beyond the basic Sunshine chapter: the network, display and input choices that decide whether streaming feels natural.
