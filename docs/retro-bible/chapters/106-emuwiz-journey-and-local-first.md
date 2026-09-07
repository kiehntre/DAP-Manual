# The EmuWiz Journey: A Safer Way to Handle a Messy Collection

EmuWiz began with a practical problem: a large collection is not a neat list of
games. It is a mixture of archives, dumps, disc images, tapes, BIOS files,
multi-file sets, damaged downloads, and things whose names stopped making sense
years ago. The project first looked like an archive browser and gradually became
an evidence-led collection tool. That journey is useful because it mirrors the
journey many collectors take themselves.

## From ArchiveFS to EmuWiz

The early idea was to browse and mount archives. That exposed a limitation:
being able to open a file is not the same as knowing what it is. The next layer
was collection scanning, followed by structural media inspection and DAT
verification. From there came repair and organisation planning, a curated
Playing Library/1G1R projection, emulator readiness, launch planning, cheats,
and preservation evidence.

The important change was philosophical. A filename-led tool asks, “What should
I call this?” An evidence-led tool asks, “What can I prove, what remains
uncertain, and what is safe to change?” The latter is slower for an ambiguous
disk, but it prevents a confident wrong rename from becoming permanent history.

In this guide, EmuWiz status is deliberately honest:

| Status | Meaning |
| --- | --- |
| **Implemented** | Present in the current EmuWiz line and backed by tests or a documented workflow. |
| **Partial** | Useful coverage exists, but some formats, providers, or physical QA remain incomplete. |
| **Researched** | The evidence and design have been audited, but the user feature is not promised. |
| **Planned/Future** | A useful direction, not a shipped capability. |

Human Ground Truth, community evidence, a broad real-corpus benchmark runner,
and Museum Mode are future-facing concepts. They should not be described as
features that a current build silently performs.

## Local-first is a preservation feature

The safest default for an irreplaceable collection is local, inspectable and
offline-capable. A scanner should be able to tell you that an image is a valid
CHD, a likely PC-98 disk, or an unknown archive without asking a web service for
permission. Network metadata is enrichment, not the foundation of identity.

Local-first means:

1. inspect before writing;
2. keep source files in place;
3. make a plan that can be reviewed;
4. require an explicit confirmation for mutation;
5. journal the change and retain rollback information.

This matters more on a NAS full of old dumps than on a disposable downloads
folder. A provider can disappear, a DAT can be replaced, and a network mount can
go read-only. Your evidence and source preservation should still make sense.

Read-only inspection is not timidity. It is a way to make the dangerous step
small. When a rename plan says “this is the proposed destination, this is the
evidence, and this is the collision,” you can make a decision without gambling
the original.

## Keep evidence separate

Container evidence, filesystem evidence, a DAT match, a successful launch and a
human observation answer different questions. A valid D88 proves a container
family. A PC-98 boot signature may prove a platform. A Redump logical-track hash
can identify disc content. None of these alone necessarily proves the retail
title named by a file.

Keep those layers visible. A good record can say “PC-98 platform, exact software
unknown, DAT not checked” without being a failure. It is much more useful than a
wrong title that looks finished.

## The simple interface over the forensic engine

Under the hood, EmuWiz has bounded readers, timing analysis, hash checks,
profile discovery, command planning and transaction journals. A normal user
should see a small number of useful states: **Ready**, **Needs review**,
**Missing BIOS**, **Duplicate**, **Unidentified**, or **Can be organised**.
Advanced evidence should be available on demand, not forced into every row.

That is a general design lesson: progressive disclosure is not hiding facts. It
is showing the next safe action first and the forensic trail when someone needs
to audit it.

## A working mental model

Treat a collection as three related things:

- a preserved source library;
- an evidence and identity ledger;
- one or more disposable projections for playing or sharing.

The source library is not cleaned up merely to make a frontend happy. A Playing
Library, RomM projection, or ES-DE tree is a view built from evidence. If the
projection is wrong, rebuild the projection; do not throw away the source.

For more detail, continue to [Evidence-Based Identification](102-evidence-based-identification.md), [Scanning and Library Organisation](107-scanning-and-library-organisation.md), and [Repair Without Destruction](110-cheats-mods-and-reversible-repair.md).
