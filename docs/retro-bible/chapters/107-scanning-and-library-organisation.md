# Scanning Huge Libraries and Building Safe Projections

A large collection is a storage and evidence problem before it is a frontend
problem. Start with a scan that can be stopped and resumed. Do not begin by
renaming a tree you have not measured.

## Scan first, mutate later

For a USB or NAS collection, make the first pass read-only. Record path, size,
mtime, container signature, archive members where bounded inspection is safe,
and any hashes that are actually needed. Avoid reading every byte of every
disc just to display a platform. Optical and tape analysis can be expensive;
make deep verification a deliberate second pass.

Use a staging location for indexes and reports. If a network mount disconnects,
the scan should report an incomplete item rather than quietly treating it as an
empty directory. A zero-byte result, permission error, or truncated archive is
evidence about the scan, not evidence that the source is empty.

### Practical scan rules

- Use bounded archive readers. Refuse path traversal, decompression bombs and
  absurd member counts.
- Keep a stable source identity so an incremental scan can notice replacement
  or drift.
- Hash only when a DAT, duplicate check or logical-media verification needs it.
- Preserve the original path as provenance even if a later projection uses a
  friendly name.
- Separate “not inspected” from “inspected and unknown”.
- Record failures for review instead of retrying a failing disk forever.

Nested archives are common in Internet Archive downloads. An outer ZIP name is
not the identity of an inner ROM, and a split RAR is not a complete file until
all parts are present. Inspect members before extracting, and extract only to a
quarantine or staging directory when a consumer truly requires loose files.

## Source library, Playing Library and 1G1R

Keep the raw library boring and stable. It may contain verified dumps, original
archive names, duplicate regions, BIOS dependencies and files that no frontend
can launch yet. A curated Playing Library is a projection that selects one
playable representative according to explicit preferences.

1G1R (“one game, one region”) is a selection policy, not a deletion policy. A
parent/clone relationship, region preference, revision preference and language
preference should be visible in the plan. A Japanese alternate, prototype or
translation can remain in the source library even when it is not selected for
the everyday view.

Multi-disc games and multi-disk computer software are sets. The plan should
retain the relationship and order. Never flatten `Disc 1`, `Disc 2`, data discs,
scenario discs or install discs into unrelated single files merely because a
frontend has a simple folder convention.

## Links, copies and destination portability

A projection can use links where the filesystem and backup policy make that
safe, or copies where portability matters more. Symlinks are convenient but can
break when a frontend runs in a container or on another machine. Hardlinks save
space but share inode content and do not cross filesystems. Copies are the most
portable and the most expensive.

For RomM, think in terms of a server-facing projection: stable platform roots,
predictable slugs, artwork and metadata that can be regenerated, and paths the
service can actually read. RomM is optional and should not become the authority
for local identity. For ES-DE, preserve its platform naming and gamelist
conventions, but keep publication separate from source scanning. LaunchBox and
RetroDECK have their own import and metadata assumptions; an export that looks
right in one frontend may be a poor input for another.

Before applying a projection, test a sample containing a multi-disc game, a
parent/clone pair, a non-ASCII title, a very long path, a BIOS dependency and a
file with no exact DAT match. This catches path and metadata assumptions before
they are multiplied across thousands of entries.

Choose a destination naming profile explicitly when the target may be Windows,
NTFS, FAT or exFAT. Portable naming must be applied by the planner, not by a
last-minute shell script that silently replaces characters. Always preview the
result and treat a case-fold collision as a conflict.

## Frontend projections

RomM, ES-DE, LaunchBox and RetroDECK have different metadata and path
expectations. A folder called “PS2” is not a universal contract, and a slug that
works in one frontend may be ugly or invalid in another. Build a projection from
the identity ledger:

- platform mapping;
- title and region policy;
- multi-disc ordering;
- BIOS relationship;
- artwork and metadata provenance;
- exact destination path;
- link/copy choice.

ES-DE publication and RomM mapping should remain optional. A frontend should
not become the only place where the collection’s identity exists. If a
publication is wrong, regenerate it from the preserved source and the reviewed
plan.

## When an item is not ready

An unknown item should offer next actions: inspect the archive, add the relevant
DAT, verify a disc, inspect a boot sector, or ask for human confirmation. An
ambiguous item should show competing evidence and the missing decisive clue.
That is a workflow, not a dead-end label.

See [Evidence-Based Identification](102-evidence-based-identification.md),
[RomM Library Management](24-romm-library-management.md), and [Recommended
Folder Structures](97-recommended-folder-structures.md).

## A large-collection day plan

For an Internet Archive haul or a recovered USB disk, use separate passes:

**Pass one: inventory.** Count files, directories, archives and obvious sets.
Record failures and storage errors. Do not rename anything.

**Pass two: structural inspection.** Identify containers, archives, disc
topology, tape headers and filesystem candidates. Keep deep reads bounded.

**Pass three: identity.** Apply the relevant DAT or logical-media verifier.
Resolve exact matches first and queue the rest by evidence quality.

**Pass four: readiness.** Discover emulators and firmware without changing their
configuration. A game can be correctly identified but not launch-ready.

**Pass five: projection.** Build a Playing Library, RomM mapping or ES-DE
publication plan. Review paths, collisions and portability before applying.

This ordering means a failing NAS does not turn into a destructive rename job,
and a missing BIOS does not get mistaken for a bad dump. It also makes retries
cheap: a projection can be rebuilt from the ledger without rescanning every
byte.
