# Arch and Rolling Release Setups

Arch is attractive because it gives retro gaming users current packages, clear documentation and direct control. It also expects the owner to pay attention.

This chapter is a skeleton for using rolling-release Linux without turning the retro setup into a permanent science project.

## Historical Context

Arch earned its place in Linux culture through simplicity, transparency and excellent documentation. For emulation, that can be valuable. New emulator builds, new GPU features and niche tooling often become available quickly.

## Practical Advice

- Keep a maintenance log before large updates.
- Snapshot the system if possible.
- Know which packages come from official repos and which come from the AUR.
- Avoid replacing stable emulator workflows just because a newer build exists.
- Keep controller, GPU and Sunshine notes close to the system notes.

## Linux-First Recommendations

Arch suits users who are comfortable reading package news and fixing their own machine. It is less ideal for a box that should be invisible to everyone else in the house.

```text
/mnt/games/maintenance/arch-updates/
/mnt/games/backups/configs/
```

> **DAP Tip**
>
> Before a big rolling update, record the emulator versions that matter. If a favourite game regresses, you will know what changed.

## Common Mistakes

- Updating during a troubleshooting session.
- Installing several AUR variants of the same emulator.
- Ignoring `.pacnew` files.
- Assuming the latest kernel or Mesa stack is always the fix.

## Troubleshooting

### Emulator breaks after a system update

Check package logs, emulator release notes, GPU driver changes and whether a config migration happened.

### AUR build fails

Check comments, upstream changes and whether a Flatpak or AppImage would be a better fit for that tool.

## DAP Warning

Rolling release does not mean reckless release. If the retro machine has to be dependable, updates need the same respect as backups.

## What Comes Next

Next comes AppImage and portable builds: useful when packaging is the problem, but not a free pass to forget updates.
