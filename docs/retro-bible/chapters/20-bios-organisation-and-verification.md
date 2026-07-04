# BIOS Organisation and Verification

BIOS files are small, but they cause a large amount of confusion.

This draft chapter will turn the BIOS folder into something calm: named, verified, backed up and easy to explain later.

## Historical Context

- Explain why many consoles used separate firmware or boot ROMs.
- Separate BIOS, firmware, keys and system data.
- Note that emulator requirements vary by system and by project.

## Concepts

| Term | Meaning | Draft note |
| --- | --- | --- |
| BIOS | Low-level system firmware. | Often required for accurate emulation. |
| Firmware | Broader system software. | May include updates or system modules. |
| Checksum | File identity check. | Used to prove the file is the expected one. |
| Region | Hardware or software territory. | Can affect boot behaviour and compatibility. |

> **DAP Warning**
>
> A file with the right name is not automatically the right BIOS. Verify the checksum before blaming the emulator.

## Practical Setup

- Create one controlled BIOS source folder.
- Keep original filenames where they help verification.
- Record required filenames per emulator.
- Keep checksum notes beside the files.
- Symlink or copy into frontend-specific paths only when needed.

## Recommended Layout

```text
/mnt/games/bios/
/mnt/games/bios/_checksums/
/mnt/games/bios/_notes/
```

## Real-World DAP Setup

- Document the actual shared BIOS path.
- Note which frontends read directly from it.
- Note which emulators still need local copies.
- Include Flatpak path quirks where they appear.

## Common Mistakes

- Mixing BIOS files with ROM folders.
- Renaming files without recording original names.
- Trusting random archive labels.
- Forgetting regional BIOS differences.
- Backing up games but not firmware.

## Troubleshooting

### Emulator says BIOS is missing

- Check the path the emulator actually uses.
- Check case sensitivity.
- Check Flatpak sandbox paths.
- Check checksum, not only filename.

### Game boots to the wrong region

- Check BIOS region.
- Check game region.
- Check emulator region settings.

## Key Points

- BIOS files deserve the same care as game images.
- Checksums matter more than filenames.
- One clean source folder makes the whole setup easier to maintain.

## Further Reading

- Add official emulator BIOS documentation.
- Add checksum and DAT references where appropriate.
