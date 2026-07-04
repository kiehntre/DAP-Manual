# ES-DE in Practice

ES-DE is often the living room face of the library.

This draft chapter will cover the practical setup choices that make ES-DE feel tidy, predictable and friendly on the DAP stack.

## Historical Context

- Explain the EmulationStation family.
- Note why controller-friendly frontends matter.
- Distinguish ES-DE from emulator configuration itself.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| System | A platform shown in the frontend. | Maps to a folder and launcher. |
| Gamelist | Metadata file for a system. | Can be generated or edited. |
| Scraper | Artwork and metadata fetcher. | Needs cleanup after mistakes. |
| Alternative emulator | Different launch target. | Useful per system or game. |

## Practical Setup

- Point ES-DE at known ROM folders.
- Configure systems before large scraping.
- Test launch commands.
- Set controller navigation.
- Back up gamelists and settings.

## Recommended Layout

```text
/mnt/games/roms/
/mnt/games/metadata/es-de/
/mnt/games/themes/es-de/
```

## Real-World DAP Setup

- Document installed ES-DE method.
- Document config path.
- Document symlink strategy.
- Note which systems use standalone emulators.

> **DAP Tip**
>
> Fix launch behaviour before scraping. Pretty artwork does not help if every game opens the wrong emulator.

## Common Mistakes

- Treating ES-DE as the only place configuration lives.
- Scraping before filenames are clean.
- Forgetting hidden files and duplicate extensions.
- Changing emulator paths without updating ES-DE.

## Troubleshooting

### Game appears but does not launch

- Check alternative emulator setting.
- Check file extension.
- Check launch command.
- Check Flatpak or AppImage paths.

### Artwork is wrong

- Check filename.
- Rescrape one game first.
- Inspect gamelist entry.
- Clear bad cached media if needed.

## Key Points

- ES-DE is a frontend, not the storage authority.
- Clean folders make scraping easier.
- Launch commands need testing before polish.

## Further Reading

- Add ES-DE documentation.
- Add theme and scraper references.
