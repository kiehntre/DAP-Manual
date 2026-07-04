# Network Storage and Permissions

Shared storage is wonderful until one process cannot read it and another process writes files as the wrong user.

This draft chapter will cover mounts, permissions and the quiet Linux details that keep a shared retro library reliable.

## Historical Context

- Explain why retro libraries outgrow local disks.
- Cover NAS, SMB, NFS and local merger-style layouts at a high level.
- Note why Linux permissions are part of preservation hygiene.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Mount | Attached filesystem path. | Must exist before services scan. |
| UID/GID | Numeric user and group identity. | Containers care about numbers. |
| Permission | Read, write and execute access. | Directories need execute to enter. |
| Ownership | User and group assigned to files. | Affects emulator and service access. |

## Practical Setup

- Pick stable mount paths.
- Decide read-only versus write access.
- Align container user IDs.
- Test frontend, emulator and RomM access.
- Document recovery steps after mount failure.

## Recommended Layout

```text
/mnt/games/
/mnt/games/roms/
/mnt/games/bios/
/mnt/games/saves/
/mnt/games/metadata/
```

## Real-World DAP Setup

- Document actual mounts.
- Record ownership policy.
- Record which services need write access.
- Note systemd mount dependencies where relevant.

> **DAP Warning**
>
> Do not give every container full write access because one path failed. Fix the permission model instead.

## Common Mistakes

- Mounting storage after Docker services start.
- Mixing root-owned and user-owned files.
- Forgetting execute permission on directories.
- Letting scrapers write into verified source folders.

## Troubleshooting

### Service sees an empty library

- Check mount exists.
- Check service start order.
- Check container path mapping.
- Check permissions from inside the service.

### Files are created as root

- Check container UID and GID.
- Check service user.
- Check bind mount options.

## Key Points

- Mount paths should be stable and boring.
- Permissions decide whether the stack behaves.
- Write access should be deliberate.

## Further Reading

- Add Linux permissions references.
- Add Docker bind mount documentation.
- Add SMB and NFS notes.
