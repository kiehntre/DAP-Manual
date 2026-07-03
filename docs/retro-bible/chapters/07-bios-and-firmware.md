# BIOS and Firmware

BIOS and firmware files are some of the most misunderstood parts of emulation.

People often treat them like magic keys: copy enough mysterious files into enough mysterious folders and eventually something boots. That approach works just often enough to create bad habits, then fails at the worst possible time.

This chapter explains what BIOS and firmware files are, why some emulators need them, how to organise them and how to avoid turning a working setup into a haunted filing cabinet.

## What BIOS Means

BIOS stands for Basic Input/Output System.

In retro gaming discussions, the term is often used broadly to describe low-level system software that original hardware used during startup or operation. Some systems have a small boot ROM. Others have larger firmware environments. Some emulators need these files. Others use high-level emulation or reimplemented firmware.

The exact technical meaning varies by system, but the practical question is usually simple:

Does this emulator need a dumped file from the original hardware to behave correctly?

## BIOS vs Firmware

The words BIOS and firmware are often used together, but they are not always the same thing.

| Term | Meaning in this book |
| --- | --- |
| BIOS | Low-level boot or system ROM required by some emulators. |
| Firmware | Broader system software used by consoles, handhelds or devices. |
| System files | A general phrase for BIOS, firmware, keys, fonts or required support files. |

Use the emulator's own documentation when naming files. If PCSX2 calls something BIOS, call it BIOS. If RPCS3 refers to firmware, call it firmware.

## Why Some Emulators Need BIOS Files

Some systems depend heavily on original system software.

A BIOS or firmware file may contain:

- startup code;
- region checks;
- CD or DVD access routines;
- memory card managers;
- system menus;
- fonts;
- audio routines;
- security behaviour;
- hardware initialisation.

An emulator can sometimes recreate this behaviour. In other cases, the original file is required for accuracy or compatibility.

> **DAP Deep Dive**
>
> Some emulators use high-level emulation to replace firmware behaviour with their own code. That can be convenient, but it may not behave exactly like the original system software.

## Legal Position

BIOS and firmware files are usually copyrighted software.

The DAP Retro Bible does not provide BIOS files, firmware files, console keys or links to copyrighted system software.

The clean preservation position is:

- dump required system files from hardware you own where legally allowed;
- keep source notes;
- do not mix unknown BIOS files into a trusted setup;
- verify checksums where emulator documentation provides them;
- do not share copyrighted firmware casually.

If a guide treats BIOS files as an afterthought, be cautious.

## Folder Organisation

A controlled BIOS folder prevents confusion.

Recommended base layout:

```text
/mnt/games/bios/
  dreamcast/
  ps1/
  ps2/
  saturn/
  sega-cd/
  pc-engine-cd/
  neogeo/
  ps3-firmware/
  switch-firmware/
```

Keep original source files in one place. If an emulator or frontend requires a different path, use a documented copy or symlink.

Avoid this:

```text
random-bios.zip
BIOS_NEW/
BIOS_WORKING_DO_NOT_DELETE/
retroarch/system/old/
pcsx2/bios/maybe/
```

That way lies madness with a folder icon.

## Naming Matters

Many emulators expect exact filenames.

Examples may include region-specific BIOS names, lowercase filenames or specific extensions. A file can have the right data but the wrong name. The emulator may still fail.

Good practice:

- keep original dumped filename if useful;
- keep a copy with the emulator-required name if needed;
- document checksum and source;
- avoid renaming blindly;
- do not assume two BIOS files are identical because they are the same size.

## Checksums

Checksums help confirm identity.

Useful commands:

```bash
md5sum bios.bin
sha1sum bios.bin
sha256sum bios.bin
```

A checksum cannot make an illegal file legal, but it can help confirm that a file matches an expected dump.

For book chapters and system guides, record checksums when they are essential to emulator behaviour.

## Region Differences

Some systems have regional BIOS files.

A region can affect:

- boot screens;
- video mode;
- language;
- compatibility;
- copy protection behaviour;
- default settings;
- CD or cartridge region checks.

For systems such as Sega Saturn, PlayStation and Sega CD, region awareness can matter.

> **DAP Tip**
>
> Keep region information in notes. A BIOS folder full of anonymous files is not a preservation workflow. It is a lucky dip.

## Emulator-Specific Expectations

Different emulators handle BIOS and firmware differently.

| Emulator | Typical requirement |
| --- | --- |
| PCSX2 | PlayStation 2 BIOS files required. |
| DuckStation | PlayStation BIOS recommended or required depending on configuration. |
| Beetle Saturn / Mednafen | Saturn BIOS often expected for best compatibility. |
| Flycast | Dreamcast BIOS optional for some workflows, useful for accuracy. |
| MAME | Some systems require BIOS ROMs as part of the set. |
| RPCS3 | PS3 firmware installed through emulator workflow. |
| Xemu | Original Xbox MCPX and BIOS handling required by setup. |
| Cemu | Wii U system content may be relevant depending on workflow. |

Always check the emulator documentation. This table is a guide, not a substitute for version-specific instructions.

## RetroArch System Folder

RetroArch usually expects BIOS files under its system directory.

That directory can vary depending on install method:

```text
~/.config/retroarch/system/
~/.var/app/org.libretro.RetroArch/config/retroarch/system/
```

Flatpak builds may need filesystem permissions before they can see external BIOS folders.

Symlinks can help, but only if the sandbox can access the target.

## Flatpak Considerations

Flatpak applications are sandboxed.

A BIOS path that works for a native emulator may not work for the Flatpak version unless permissions are granted.

Useful tools include:

- Flatseal;
- `flatpak override`;
- emulator-specific folder configuration.

Example concept:

```bash
flatpak override --user --filesystem=/mnt/games/bios org.example.Emulator
```

Check the actual Flatpak application ID before running commands.

## Docker Considerations

If a web frontend or service runs in Docker, BIOS paths may need bind mounts.

Host path:

```text
/mnt/games/bios/
```

Container path:

```text
/bios/
```

The service only sees the container path. Documentation should show both when relevant.

## Real-World DAP Setup

The DAP setup should keep BIOS files in a controlled source path:

```text
/mnt/games/bios/
```

Frontend or emulator-specific paths can then point back to that source.

Example:

```text
~/retrodeck/bios/saturn -> /mnt/games/bios/saturn
```

This keeps the source clear while still satisfying tools that expect local folders.

## Common Mistakes

Common BIOS mistakes include:

- downloading random BIOS packs with no source notes;
- copying the same files into ten folders;
- renaming files without recording original names;
- ignoring checksums;
- mixing regions;
- assuming Flatpak apps can see host paths;
- forgetting that Docker containers see container paths;
- deleting BIOS files because a frontend was rebuilt.

## Troubleshooting

### Emulator says BIOS missing

Possible causes:

- wrong folder;
- wrong filename;
- wrong region;
- Flatpak permission issue;
- Docker bind mount missing;
- compressed BIOS file not extracted;
- emulator expects a different file.

Fix:

- check emulator documentation;
- verify the configured BIOS path;
- confirm filename and checksum;
- test with a native build if Flatpak paths are confusing;
- document the working path.

### Game boots to system menu only

Possible causes:

- disc image not detected;
- wrong region pairing;
- bad dump;
- emulator setting issue;
- missing CD subsystem BIOS.

Fix:

- test another known-good game;
- check disc image format;
- verify BIOS region;
- review emulator logs.

## Key Points

- BIOS and firmware files are system software, not decoration.
- Some emulators require them for compatibility or accuracy.
- They are usually copyrighted and must be handled carefully.
- Keep one controlled BIOS source folder.
- Use checksums and source notes.
- Document symlinks, Flatpak permissions and Docker bind mounts.
- Do not build a setup around mystery BIOS packs.

## What Comes Next

Next comes emulator selection: how to choose between standalone emulators, RetroArch cores, accuracy, performance, Linux support and long-term maintainability.