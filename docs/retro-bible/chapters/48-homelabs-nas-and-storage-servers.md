# Homelabs, NAS and Storage Servers

A large retro library eventually asks where it should live. The answer is often not "on the desktop until the SSD fills up".

This chapter is a skeleton for using homelab storage without making every emulator depend on a fragile network spell.

## Historical Context

Network storage used to be specialist territory. Today, many retro users already run a NAS, media server or Docker host. That makes shared libraries practical, especially when several frontends and machines need the same files.

## Practical Advice

- Put stable library paths ahead of clever storage tricks.
- Decide which folders are read-only for clients.
- Keep saves and states close enough to avoid latency problems.
- Back up metadata databases as well as game files.
- Monitor disk health.

## Recommended Layout

```text
/mnt/games/
  roms/
  bios/
  saves/
  metadata/
  backups/
```

## Preservation Notes

Verified source folders should be protected from casual writes. Scrapers, frontends and test tools should not be able to damage the archival layer by accident.

> **DAP Tip**
>
> A NAS is not a backup by itself. It is a place where files live. Backups are copies you can restore from after something goes wrong.

## Common Mistakes

- Letting every client write everywhere.
- Running games directly from slow Wi-Fi storage.
- Forgetting database backups.
- Ignoring SMART warnings and scrub reports.

## Troubleshooting

### Games vanish after reboot

Check mount order, credentials, systemd units and whether services started before the mount appeared.

### Emulator stutters from network storage

Test local storage, check network throughput, review file format and consider keeping demanding disc systems on SSD.

## DAP Warning

If the storage server is down and nobody knows how to mount the library again, the setup is not family-proof yet.

## What Comes Next

Next comes the individual systems: the heart of the book, where history, preservation and emulator choices meet one platform at a time.
