# ROM Dumps and Verification

A retro library is only as strong as the files underneath it.

This draft chapter will explain how to treat ROM images and disc images as verifiable data, not mystery files with nice artwork.

## Historical Context

- Explain cartridge dumps, disc dumps and preservation sets.
- Introduce why projects such as No-Intro and Redump matter.
- Keep legal ownership separate from technical verification.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Dump | A copy made from original media. | Quality depends on method and hardware. |
| Good dump | A file that matches a known reference. | The reference needs a trusted source. |
| Bad dump | A corrupt, altered or incomplete copy. | Sometimes still boots, which is the trap. |
| DAT file | Verification database. | Used by ROM managers and auditing tools. |

> **DAP Myth**
>
> If a game launches, that does not prove the dump is clean. Some bad dumps fail much later.

## Practical Setup

- Decide whether the set is archival, playable or both.
- Keep raw verified files separate from converted working copies.
- Use checksums before format conversion.
- Record the verification source and date.

## Recommended Layout

```text
/mnt/games/verified/
/mnt/games/roms/
/mnt/games/conversions/
/mnt/games/dat/
```

## Real-World DAP Setup

- Document where verified originals live.
- Document which folder frontends scan.
- Note when CHD, RVZ or WUA copies are generated from originals.

## Common Mistakes

- Treating filenames as proof.
- Mixing hacks and translations into verified sets.
- Losing the original after conversion.
- Verifying after moving files through several tools.

## Troubleshooting

### DAT tool reports many missing files

- Check naming convention.
- Check region filter.
- Check archive format support.
- Check whether the DAT covers the exact system revision.

### Emulator accepts one copy but not another

- Compare checksums.
- Check file format support.
- Check headered versus unheadered ROM expectations.

## Key Points

- Verification protects time, not only storage.
- Keep originals and working copies conceptually separate.
- A playable library can still be built from a preservation-minded source.

## Further Reading

- Add No-Intro documentation.
- Add Redump documentation.
- Add ROM manager documentation.
