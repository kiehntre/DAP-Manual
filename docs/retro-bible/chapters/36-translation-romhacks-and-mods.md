# Translations, ROM Hacks and Mods

Translations, hacks and mods are part of how old games stay alive.

This draft chapter will explain how to keep them visible and playable without polluting verified preservation sets.

## Historical Context

- Explain fan translations and preservation of inaccessible releases.
- Cover ROM hacks, improvement patches and randomisers.
- Separate community creativity from original archival material.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Patch | Difference file applied to a clean source. | Common formats include IPS, BPS and xdelta. |
| Hack | Modified game. | Can be minor or complete overhaul. |
| Translation | Language patch. | Often tied to exact ROM revision. |
| Randomiser | Tool that changes game data. | May produce many generated variants. |

## Practical Setup

- Keep clean originals separate.
- Store patch files and notes.
- Record required source checksum.
- Name patched copies clearly.
- Keep generated randomiser files out of main verified folders.

## Recommended Layout

```text
/mnt/games/patches/translations/
/mnt/games/patches/romhacks/
/mnt/games/roms-patched/
/mnt/games/randomisers/
```

## Real-World DAP Setup

- Document patched library location.
- Record frontend naming policy.
- Note whether patched games appear beside originals.

> **DAP Tip**
>
> A patched copy should tell its story in the filename and notes: original game, patch name, version and language where relevant.

## Common Mistakes

- Applying patches to the wrong ROM revision.
- Overwriting clean originals.
- Losing the patch after creating a patched copy.
- Scraping hacks as the original game without notes.

## Troubleshooting

### Patch tool says checksum mismatch

- Check source ROM version.
- Check headered versus unheadered ROM.
- Check patch documentation.

### Patched game boots but crashes

- Check emulator compatibility.
- Check patch version.
- Check whether another patch was already applied.

## Key Points

- Hacks and translations are valuable, but they are not clean originals.
- Patch notes and source checksums matter.
- Frontend metadata should make modified games obvious.

## Further Reading

- Add patch format references.
- Add translation project sources.
