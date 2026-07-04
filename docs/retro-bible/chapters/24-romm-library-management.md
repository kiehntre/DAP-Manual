# RomM Library Management

RomM gives a retro library a web face, but it should not become the only source of truth.

This draft chapter will place RomM in the DAP stack as a manager, browser and metadata layer over storage that remains understandable without it.

## Historical Context

- Explain the rise of self-hosted media managers.
- Compare retro libraries with music and film library tools.
- Position RomM as part of a homelab setup.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Library manager | Tracks games and metadata. | Not the same as verified storage. |
| Scanner | Reads folders and imports entries. | Depends on naming and paths. |
| Metadata provider | Supplies artwork and descriptions. | Can be wrong or incomplete. |
| Database | RomM's internal state. | Needs backup. |

## Practical Setup

- Define storage mounts before importing.
- Keep system folder names predictable.
- Decide which metadata RomM owns.
- Back up the database and config.
- Test one small system before scanning everything.

## Recommended Layout

```text
/mnt/games/roms/
/mnt/games/bios/
/mnt/games/metadata/romm/
/mnt/games/backups/romm/
```

## Real-World DAP Setup

- Document Docker compose location.
- Document mounted paths.
- Document backup path.
- Note scan behaviour that differs from frontends.

> **DAP Tip**
>
> Let RomM enrich the library, but keep folder structure readable even if RomM is stopped.

## Common Mistakes

- Importing a messy folder and expecting metadata to fix it.
- Forgetting database backups.
- Giving the container paths that differ from host notes.
- Letting duplicate files create duplicate entries.

## Troubleshooting

### RomM cannot see games

- Check container mounts.
- Check file permissions.
- Check system folder mapping.
- Check supported extensions.

### Metadata looks wrong

- Check filename.
- Check region.
- Check selected provider.
- Check whether the game is a hack, demo or translation.

## Key Points

- RomM is useful, but storage remains the foundation.
- Container paths must be documented clearly.
- The database is part of the library and needs backup.

## Further Reading

- Add RomM documentation.
- Add Docker backup references.
