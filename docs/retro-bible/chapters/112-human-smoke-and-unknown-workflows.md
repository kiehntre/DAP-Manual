# Human Smoke Tests and Making “Unknown” Useful

Automated tests prove contracts. They do not tell you that a confirmation button
is below the fold, that a warning is incomprehensible, or that a real user has
no idea what to do next. A human smoke pass is a product test, not a ceremonial
click-through.

## The planned smoke route

On a disposable source and test profile, walk through:

1. onboarding and source selection;
2. scan and discovery;
3. an exact, an unknown and an ambiguous item;
4. DAT setup and evidence details;
5. emulator setup, BIOS diagnostics and a safe launch;
6. Cheats & Mods preview without an accidental apply;
7. Library Organisation and Playing Library preview;
8. RomM/ES-DE publication preview where configured;
9. tape, optical and WHDLoad evidence;
10. restart, persistence, undo and recovery.

Record a **BALL-ACHE LOG**: too many clicks, buried actions, confusing words,
unexpected reset, unclear error, visual clutter, inaccessible controls, or a
workflow that requires tribal knowledge. These are real defects even when the
backend is correct.

Physical desktop QA should record display availability, viewport and screenshots
separately from headless test results. A screenshot utility missing in a
container is not a GUI product defect.

## Unknown is a next action

Unknown should explain whether the problem is an unsupported format, missing DAT,
damaged file, generic container, hidden archive member, incomplete set, wrong
extension, or simply lack of decisive evidence. Offer the smallest next step:

- inspect deeper;
- add or select a DAT snapshot;
- verify a logical disc track;
- inspect the archive member;
- confirm a platform manually;
- mark tested or verified;
- leave it safely untouched.

Ambiguous should list candidate platforms and the evidence for each. “Generic
FAT” may be useful filesystem evidence without being PC-98 proof. Two matching
DATs may be conflicting sources rather than a reason to pick one silently.

## Human Ground Truth: planned, not magic

The useful future model separates **USER_LABELLED**, **USER_TESTED** and
**USER_VERIFIED** from machine and DAT evidence. “I know this is Amiga because I
played it in Amiberry last night” is valuable evidence when bound to the exact
content hash, emulator/profile and date. It is not a universal replacement for
a DAT match.

Correction and retraction matter. A human label should be explainable, editable
and distinct from an automated assertion. The local subsystem is a **planned**
capability unless the current build explicitly exposes it; do not invent a
database write simply to make the UI look complete.

Community evidence is also **future/planned**: opt-in, hash-bound submissions
with platform, title, emulator and launch-success details; no ROM-byte upload,
private path or filename sharing by default. Agreement can raise confidence, but
a majority vote is not proof and disagreement must remain visible.

The right question is not “Can the program guess?” but “What can the person do
next without risking the source?”

## A compact review card

For each uncertain item, a useful card can show:

| Field | Example |
| --- | --- |
| What was inspected | `game.hdi`, valid HDI header, 512-byte sectors |
| Evidence | Human68k marker found; no exact DAT match |
| Candidate | X68000 (strong platform), title unknown |
| Missing evidence | logical boot hash or catalogue record |
| Safe next action | inspect boot sector, select DAT, or leave untouched |

This format is useful in a desktop GUI, a command-line report and a future
community submission. It prevents the user from having to remember which of
twenty warnings actually matters.

The card should also show whether the next action is read-only, reversible, or
requires a backup. That one sentence often matters more than another technical
detail: users can safely inspect an item now and postpone a risky mutation until
they understand the evidence.

It is also a useful boundary for support: a reviewer can tell whether the
reported problem is a missing clue, a broken tool, or a choice that only the
collection owner can make.
