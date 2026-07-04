# Disc Images, CHD, RVZ and WUA

Disc-based systems bring bigger files, stranger formats and more chances to make a mess.

This draft chapter will explain the main disc image formats and when each one belongs in a DAP-style library.

## Historical Context

- Explain why optical media created multi-track dumps.
- Cover cue/bin, ISO and system-specific disc formats.
- Introduce modern compressed formats used by emulators.

## Concepts

| Format | Common use | Draft note |
| --- | --- | --- |
| CUE/BIN | CD-based systems. | Preserves tracks and layout. |
| ISO | Simple data discs. | Not enough for every system. |
| CHD | Compressed disc storage. | Strong support in many emulators. |
| RVZ | Dolphin GameCube and Wii. | Designed for verified compression. |
| WUA | Cemu Wii U. | Useful for compact Wii U libraries. |

> **DAP Tip**
>
> Keep the verified source somewhere safe before converting to a smaller working format.

## Practical Setup

- Identify the system first.
- Check emulator support before converting.
- Convert one test game before bulk work.
- Record source checksum and target format.
- Keep conversion commands in notes.

## Recommended Layout

```text
/mnt/games/discs/source/
/mnt/games/discs/chd/
/mnt/games/discs/rvz/
/mnt/games/discs/wua/
```

## Real-World DAP Setup

- Document which systems use CHD.
- Document where Dolphin RVZ files live.
- Document Wii U conversion policy.
- Note any frontend scraping differences caused by extensions.

## Common Mistakes

- Converting everything to one format.
- Deleting cue files after keeping only bin files.
- Assuming ISO means accurate.
- Using a format before checking emulator support.
- Forgetting multi-disc naming.

## Troubleshooting

### Game disappears from frontend

- Check extension allowlist.
- Check system folder mapping.
- Check whether compressed files are inside another archive.

### Audio is missing from a CD game

- Check cue/bin pairing.
- Check multi-track preservation.
- Check whether conversion skipped audio tracks.

## Key Points

- Disc image format is a system-specific decision.
- Compression is useful only when the emulator supports it properly.
- Verified originals and playable conversions should be tracked separately.

## Further Reading

- Add CHD documentation.
- Add Dolphin RVZ documentation.
- Add Cemu WUA documentation.
