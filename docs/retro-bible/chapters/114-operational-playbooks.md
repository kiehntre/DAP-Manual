# Operational Playbooks for Real Collections

The best preservation advice is a repeatable order of operations. The following
playbooks are deliberately conservative and can be adapted to another scanner,
frontend or emulator manager.

## The first weekend with a large collection

**Day one: protect the source.** Make sure there is a second copy, check free
space, and decide which mount is read-only. Do not begin while a USB disk is
reporting I/O errors. Save a manifest of paths, sizes and timestamps before any
tool touches the tree.

**Day two: inventory.** Scan containers and archives. Separate obvious BIOS,
documentation, artwork, saves and patches from candidate software without
deleting anything. Quarantine only a copy of suspicious material.

**Day three: identify.** Import the right DAT families and verify exact matches.
Inspect unknown optical and computer media structurally. Review set members and
multi-disc relationships. Keep the unresolved queue.

**Day four: make it playable.** Discover emulators and firmware, then test a
small representative sample. Fix profiles and BIOS paths before building a
large frontend projection.

**Day five: project.** Preview a Playing Library or RomM/ES-DE publication.
Review region policy, parent/clone selection, multi-disc order, path length,
portability target and collisions. Apply only after the preview is understood.

This order avoids spending hours polishing names for files that later turn out
to be a different platform or an incomplete set.

## The uncertain-item playbook

When an item says Unknown, write down the question before choosing a tool:

1. Is the container valid?
2. Is the content complete?
3. Is there a platform-specific structure?
4. Is a suitable DAT available?
5. Is this a member of a larger set?
6. Can a human safely test it?

For a D88, inspect track/sector structure before assuming PC-88 or PC-98. For a
generic `.dsk`, inspect filesystem and boot evidence before naming a computer.
For a CHD, compare logical tracks where a Redump expectation exists rather than
comparing only the container digest. For a WHDLoad archive, inspect the slave
and package relationship rather than trusting its filename.

## The repair playbook

If a scanner reports a duplicate, malformed archive or destination collision:

- preserve the source and capture the report;
- make a plan with exact source and destination paths;
- review the evidence and portability warnings;
- confirm with the required phrase or control;
- apply transactionally;
- verify the result and journal it;
- test rollback on a disposable fixture.

If an operation fails halfway through, do not “finish” it with an ad-hoc shell
move. Open the recovery record, inspect which operations succeeded, and resume
or roll back through the tool that created the journal.

## The release-and-QA playbook

Before a public build, use a clean or isolated checkout and record authority
HEAD. Run focused media, launch and GUI tests first. Build the release artifact,
then test it in the environment users will actually receive. For Debian and
RPM, inspect dependencies, install and uninstall in disposable containers. For
AppImage, test both normal execution and extract-and-run when FUSE is absent.

On a real desktop, check onboarding, Home, Sources, DAT Sources, Doctor,
Library Organisation, Cheats & Mods and one launch. Resize to a compact
viewport. Capture screenshots only when the display session is genuinely
available, and label headless limitations honestly.

## The recovery checklist

If something goes wrong, stop and record:

- exact authority HEAD and worktree;
- command and environment;
- source path and destination path;
- transaction or plan identifier;
- files already changed;
- whether the original still exists;
- whether another process owns the tree;
- safe next action.

This is more valuable than a vague “it broke during import”. Recovery is an
engineering feature and a user-facing trust feature.

## Choosing copies, links and caches

For a local SSD playing library, a hardlink or symlink may be a sensible
projection. For a removable drive or a containerised frontend, a real copy may
be safer. State the choice in the plan. A cache is disposable; a source dump is
not. Never let a cache cleanup routine discover and delete a source path merely
because the projection no longer references it.

When storage is slow, cache structural observations and failed reads with their
timestamp. Recheck failures rather than treating a cached error as permanent,
but avoid an infinite retry loop against a dying disk. A bounded retry plus a
clear “source unavailable” state protects both the disk and the operator.

## Working with incomplete sets

An arcade parent without its BIOS, a multidisc game with one missing disc, or a
split archive missing part three should remain a named incomplete set. Show what
is present and what is missing. Do not create a fake single-file winner just to
make a frontend row appear.

For WHDLoad, a package with several slaves needs an explicit entry-point choice.
For computer disks, install and data disks may have different roles. For optical
media, audio tracks and pregaps can be part of the software’s behavior. Set
coherence is preservation information, not clutter.

## A small example

Suppose a folder contains `game.zip`, `game.cue`, `game.bin`, and a file called
`bios.bin`. The safe answer is not to rename all four immediately. Inspect the
archive member list, parse the CUE topology, identify whether the BIOS is a
system dependency, and compare logical track hashes if a Redump record exists.
Then build a set-aware plan with the data and audio tracks together. If the
title is still uncertain, leave the source names intact and record the evidence.
The user can play the verified set later without losing the path back to the
original dump.
