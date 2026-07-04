# LaunchBox and Big Box on Linux

LaunchBox is a Windows-first tool, but it can still have a place in a Linux-centred retro setup when used carefully.

This draft chapter will explain where it helps, where it fights the grain and how to avoid turning the library into a Windows-shaped knot.

## Historical Context

- Explain LaunchBox and Big Box as library and couch interfaces.
- Note Windows origins and Wine or Proton considerations.
- Compare with ES-DE and RomM roles.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| LaunchBox | Desktop library manager. | Strong metadata tools. |
| Big Box | Controller-friendly mode. | Premium feature. |
| Wine prefix | Windows compatibility environment. | Needs backup and care. |
| Import | Library entry creation. | Can copy or reference files. |

## Practical Setup

- Decide whether LaunchBox references shared ROM paths or manages copies.
- Keep emulator paths clear.
- Test Wine behaviour before a large import.
- Back up LaunchBox data.
- Avoid using it as the only metadata source.

## Recommended Layout

```text
/mnt/games/roms/
/mnt/games/metadata/launchbox/
/mnt/games/backups/launchbox/
```

## Real-World DAP Setup

- Document whether LaunchBox is used directly, through Wine or only for metadata.
- Record import settings.
- Record any path translation notes.

> **DAP Warning**
>
> Windows path assumptions can quietly break a Linux library. Write down every path mapping before importing thousands of games.

## Common Mistakes

- Letting LaunchBox copy files unexpectedly.
- Mixing Windows paths and Linux paths in notes.
- Forgetting the Wine prefix in backups.
- Assuming Big Box behaviour matches ES-DE.

## Troubleshooting

### Imported games cannot launch

- Check emulator executable path.
- Check Wine drive mapping.
- Check quoted paths.
- Check working directory.

### Media takes too much space

- Check image download options.
- Remove duplicate media types.
- Move media to documented storage.

## Key Points

- LaunchBox can be useful, but Linux path discipline matters.
- Back up the application data, not just the ROMs.
- Keep it in its lane beside ES-DE and RomM.

## Further Reading

- Add LaunchBox documentation.
- Add Wine path references.
