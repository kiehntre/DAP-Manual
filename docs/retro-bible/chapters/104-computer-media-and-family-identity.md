# Computer Media, Amiga, and Japanese Families

Computer software is often a small ecosystem of disks, loaders, firmware and
machine-specific conventions. A generic disk extension can be the beginning of
an investigation, not the end of one.

## Amiga preservation

### ADF, ADZ, and HDF

An **ADF** is a floppy image representation. **ADZ** is a compressed ADF-style
image and should be decompressed or read through a bounded adapter without
changing the source. **HDF** is a hard-disk image and can contain an Amiga RDB,
partitions and filesystems. A file named `game.adf` is not automatically a
particular Amiga release; disk structure, catalogue metadata and DAT evidence
still matter.

Keep an ADF/ADZ with its companion disks and any preservation notes. Do not
rename a boot disk to the game title simply because its volume label resembles
one.

### WHDLoad packages

WHDLoad is especially useful because a validated `.slave` file carries a
machine-readable identity and runtime contract inside a package or directory.
An LHA/LZH archive may contain one slave, several slaves, documentation, data
files, or an incomplete installation. The safe workflow is:

1. inspect archive members with bounds and path-traversal protection;
2. validate the slave structure;
3. bind the selected slave to the package/files it needs;
4. reconcile a DAT or TOSEC identity where available;
5. only then prepare an emulator launch.

One archive with one validated slave is easier to identify than a multi-slave
collection. Multiple valid slaves are not permission to choose one silently.

### Kickstart and launch readiness

Kickstart readiness is a separate claim from identity. A package can be
correctly identified yet fail to launch because the selected Amiberry or
FS-UAE profile lacks a required Kickstart. Conversely, a launch-ready profile
does not prove that a similarly named archive is the exact release.

Keep these questions separate:

| Question | Evidence |
| --- | --- |
| What is this? | Slave/package/hash/DAT evidence |
| Can this profile launch it? | Emulator executable, profile, Kickstart and content checks |
| Did it launch successfully? | Explicit run result, optionally human-confirmed |

Amiberry and FS-UAE are execution choices, not identity authorities. A
successful launch should be recorded as compatibility evidence and should never
overwrite the source identity.

## Apple II disk evidence

Apple II media demonstrates why “`.dsk`” is not enough. DOS 3.3 and ProDOS have
different allocation and directory conventions. A 2MG container adds a header
and may describe a block image, while WOZ and NIB preserve lower-level or
flux-oriented information that a simple sector dump cannot express.

The safe order is:

1. validate the container or raw geometry;
2. identify DOS 3.3, ProDOS, or another filesystem only from coherent internal
   structures;
3. preserve WOZ/NIB as preservation formats when their lower-level evidence is
   unavailable;
4. use a DAT or human review for exact title identity.

Two generic `.dsk` files can be different machines, different DOS families, or
damaged media. Do not repair a disk image in place and do not infer a title from
the directory label alone.

## Japanese computer disks

PC-88, PC-98, X68000 and FM Towns share tooling, extensions and sometimes
geometry. The evidence must be layered.

### PC-98: D88, HDI, and NHD

Current EmuWiz wiring can inspect a validated D88 track map or the declared
payload sector of an HDI/NHD container, then pass one bounded 512-byte sector to
the PC-98 boot-evidence primitive. Strong evidence requires a coherent FAT/BPB
and a NEC OEM marker. Generic FAT remains generic.

| Input | What it proves by itself | Stronger PC-98 evidence |
| --- | --- | --- |
| D88 | Container and track/sector structure | Validated track 0/head 0/sector 1 with NEC BPB evidence |
| HDI | Header and C/H/S geometry | NEC BPB evidence at declared payload sector 0 |
| NHD | T98HDDIMAGE header and geometry | NEC BPB evidence at declared payload sector 0 |

The D88 suffix is not a PC-98 identity. A valid D88 with generic FAT remains
shared by PC-88, PC-98, FM Towns and X68000 tooling. Conflicting surrounding
evidence is retained as ambiguous, and an exact DAT/hash match remains a
separate, stronger release claim.

### X68000: XDF, DIM, and Human68k

XDF and DIM can provide strong structural evidence when their expected geometry,
track map, and X68000-oriented IPL shape validate. Human68k filesystem evidence
would be stronger still, but a valid XDF/DIM container is not an exact game
identity. D88 is shared and should not be upgraded to X68000 from the extension.

For an X68000 collection, keep disk sides and companion images together, record
whether the image is raw XDF or a DIM container, and use a verified catalogue
for release names. A 1024-byte BPB-like geometry is evidence, not a title.

### FM Towns

FM Towns media can be floppy, hard-disk, or optical. A generic ISO, D88, HDI or
NHD does not establish Towns. Family-specific IPL4, TownsOS, or firmware/boot
evidence would be useful when validated against real specimens; in the current
workflow those claims remain partial or research-only. Keep FM Towns folder
metadata as corroboration, never as a substitute for a missing parser.

### False-positive protections

The practical rules are simple:

- do not identify from extension alone;
- do not identify from capacity or geometry alone;
- do not turn generic FAT into PC-98;
- do not use a disk label as exact game identity;
- do not discard a conflict because one source “looks more likely”.

## Non-retail and preservation software

Prototype, alpha, beta, debug, review, kiosk/demo, developer SDK, diagnostic,
magazine-coverdisc, homebrew and preservation-only material are not garbage.
They are software objects with different provenance and sometimes incomplete
runtime expectations.

Classify at two levels:

1. **What is it?** Platform, media type, build or set identity when supported.
2. **What kind of release is it?** Retail, prototype, debug, kiosk, homebrew,
   sample, internal, or unknown.

Do not force a prototype into a retail DAT entry merely because its title is
similar. Preserve source notes, build strings, serials and the reason a DAT
match was not possible. Human confirmation is particularly valuable here.

## Multi-file computer media checklist

- Keep install/play/data/scenario disks together.
- Keep WHDLoad archives and selected slaves bound.
- Preserve disk labels as descriptive metadata, not automatic filenames.
- Record boot and filesystem evidence separately.
- Store exact DAT source/version and hash scope.
- Leave incomplete or damaged sets visible and explain what is missing.

## Related chapters

- [Amiga](77-amiga.md)
- [Amstrad CPC](78-amstrad-cpc.md)
- [Computer systems, keyboards and media](38-computer-systems-keyboards-and-media.md)
- [Disc images, CHD, RVZ and WUA](22-disc-images-chd-rvz-and-wua.md)
- [BIOS organisation and verification](20-bios-organisation-and-verification.md)
- The companion WHDLoad identity audit is maintained in EmuWiz at
  `/home/davedap/archivefs/docs/research/PACKAGE_INSTALLED_MEDIA_IDENTITY_AUDIT.md`.
