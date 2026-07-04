# Docker Services for Retro Libraries

Docker is useful when it keeps services tidy. It is painful when it hides where the data went.

This draft chapter will cover Docker patterns for RomM, metadata tools and supporting services in a DAP-style setup.

## Historical Context

- Explain why homelab users reach for containers.
- Cover the difference between application data and game storage.
- Note that containers do not remove the need to understand paths.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Image | Packaged application. | Updated separately from data. |
| Container | Running instance. | Should be replaceable. |
| Volume | Managed Docker storage. | Needs backup strategy. |
| Bind mount | Host path mounted into container. | Preferred for clear game storage. |

## Practical Setup

- Keep compose files in a known folder.
- Use bind mounts for large libraries.
- Use volumes or bind mounts deliberately for databases.
- Pin or record image versions.
- Back up before upgrades.

## Recommended Layout

```text
/opt/retro-services/
/mnt/games/
/mnt/games/backups/docker/
```

## Real-World DAP Setup

- Document compose project names.
- Document environment files.
- Document database backup commands.
- Record service restart order.

> **DAP Tip**
>
> A container can be disposable. The database, config and library are not.

## Common Mistakes

- Keeping important data only inside anonymous volumes.
- Changing container paths without updating notes.
- Upgrading several services at once.
- Forgetting `.env` backups.
- Letting containers write into verified source storage.

## Troubleshooting

### Container starts but data is missing

- Check mounts.
- Check volume names.
- Check UID and GID.
- Check compose project name.

### Upgrade breaks the service

- Check release notes.
- Restore database backup.
- Roll back image tag.
- Inspect logs before changing config again.

## Key Points

- Docker helps when paths and backups are explicit.
- Compose files are part of the documentation.
- Service data needs the same respect as ROMs.

## Further Reading

- Add Docker Compose documentation.
- Add service-specific backup references.
