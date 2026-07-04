# AppImage and Portable Builds

AppImages and portable builds are handy when distribution packages are old, missing or awkward. They can also become forgotten binaries sitting in a downloads folder with no update path.

This chapter is a skeleton for using portable Linux builds deliberately.

## Historical Context

Portable application formats grew from a real need: Linux distributions package software differently, and emulator projects move quickly. AppImage gives developers a way to ship one file that runs on many systems.

## Practical Advice

- Store portable builds in a controlled application folder.
- Record version numbers and download sources.
- Keep desktop shortcuts or frontend launch commands pointed at stable paths.
- Replace old builds deliberately rather than collecting copies.
- Back up configs separately from the AppImage itself.

## Recommended Layout

```text
/home/davedap/Applications/emulators/
/mnt/games/maintenance/appimage-notes/
/mnt/games/backups/emulator-configs/
```

> **DAP Tip**
>
> Rename AppImages with version numbers, then symlink a stable launcher name if frontends need a fixed path.

## Common Mistakes

- Launching an emulator from `~/Downloads`.
- Losing track of which AppImage a frontend uses.
- Assuming AppImage config paths match Flatpak or native builds.
- Keeping old builds without notes.

## Troubleshooting

### AppImage will not launch

Check execute permission, FUSE support, missing libraries and whether the build is compatible with the distribution.

### Frontend still opens the old version

Check the configured path. A frontend may be pointing at an old copy even after a new AppImage was downloaded.

## DAP Warning

Portable does not mean unmanaged. If a file launches games and writes saves, it belongs in the maintenance plan.

## What Comes Next

Next comes Wine outside Steam: the useful, messy layer for Windows tools and older PC games that Proton does not neatly cover.
