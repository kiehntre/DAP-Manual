# Evidence-Based Identification

Preservation becomes much easier when a library can say not only *what* it
thinks a file is, but *why*. A filename is useful. An extension is useful. A
verified hash is better. None of those facts should be quietly promoted into a
stronger claim than the evidence supports.

This chapter describes the evidence ladder used by the DAP preservation
workflow and reflected, where implemented, in EmuWiz. The names below are
deliberately human-friendly: **implemented** means the current EmuWiz tree has
that evidence path, **partial** means only a bounded part is available, and
**planned** means a useful design direction rather than a released feature.

## The evidence ladder

An item can accumulate several independent facts:

| Layer | What it can tell you | Typical strength |
| --- | --- | --- |
| Filename and folder | A user's organisation, a likely system, a hint about region | Hint only |
| Extension | Which parser to try (`.d88`, `.cue`, `.adf`, `.tap`) | Routing, not identity |
| Container structure | Whether the bytes form a valid D88, CHD, ZIP, ADF, or similar object | Strong format claim |
| Internal header or boot/IPL | Machine-family signatures, product codes, media geometry | Family or platform evidence |
| Filesystem/set structure | Volume label, directory model, required members, coherent disk set | Corroboration |
| Logical fingerprint | Track topology, normalized tape timing, or other content-level identity | Strong corroboration |
| DAT/hash | Exact bytes or logical tracks match a named preservation entry | Exact identity for that scope |
| Human confirmation | A person checks the evidence or successfully launches the item | Authoritative review, separately recorded |

The ladder is not a score that simply adds points. A valid FAT boot sector does
not become a PC-98 disk because a filename says `pc98`; a D88 header does not
become an exact game because its disk-name field looks familiar. The safest
answer is often a combination such as “valid D88, likely PC-98 from NEC boot
evidence, exact release unverified”.

### Why Unknown is useful

**Unknown is not failure.** It is a useful statement that the current evidence
does not justify a narrower claim. A confidently wrong platform is more
expensive: it can send a disk to the wrong emulator, cause a destructive rename,
or make a bad hash match look authoritative.

When an item is Unknown, preserve the original path and bytes, record the
reason, and ask what evidence would change the result. That might be a missing
DAT, a companion disk, a better dump, or a human review. Never “fix” Unknown by
silently trusting a weak signal.

## Evidence types in everyday language

### Names and extensions

`MyGame.d88` tells the scanner which D88 parser to attempt. It does not prove
PC-88, PC-98, FM Towns, or X68000: all of those ecosystems use D88 tooling.
Similarly, `.dsk` is shared by many computer families, and `.cas` can refer to
unrelated cassette conventions. Folder names are valuable provenance, but they
remain user-supplied evidence.

### Internal structure

A parser can validate lengths, tables, sector headers, checksums, and bounds.
That proves that the file is a *valid container* or a plausible disk, not that
it is a particular title. EmuWiz's bounded structural readers deliberately
stop before extraction or whole-image guessing. See [ROM dumps and
verification](21-rom-dumps-and-verification.md) for the same principle applied
to cartridges.

### Headers, boot records, and IPLs

Machine-specific bytes are stronger. Examples include a coherent NEC OEM/BPB
combination for PC-98, an X68000 IPL branch shape plus 1024-byte geometry, and
platform-specific optical boot evidence. Even then, platform evidence is not a
game title. It narrows the search and improves emulator selection; a DAT or
human review still owns exact identity.

### Archive members and sets

An archive can contain the evidence that the outer filename hides. A WHDLoad
LHA may contain one validated `.slave`; an arcade ZIP may contain every ROM in a
set; a multi-disc title may be represented by a CUE plus several BIN files.
Archive-member evidence must retain the member path and source archive. Do not
flatten it into “one file equals one game”.

### Humans and successful launches

A person can confirm something a parser cannot: “I know this is the Amiga
version because it launched and played correctly in Amiberry.” That is valuable
human evidence, but it should remain labelled as human evidence, not rewritten
as a DAT fact. A successful launch also proves compatibility with that emulator
profile, not necessarily exact release identity.

## DAT ecosystems without the mystery

DAT files are catalogues of expected names, sizes and hashes. They are not all
the same catalogue and should not be blended into one anonymous truth.

| Ecosystem | Best suited to | Practical caution |
| --- | --- | --- |
| No-Intro | Clean cartridge/handheld dumps and variants | Headered, headerless, BIOS and aftermarket variants are distinct metadata |
| Redump | Optical discs and logical track identities | A container hash is not the same as a track-by-track match |
| MAME | Arcade machine software lists and set dependencies | Parent, clone, BIOS and device relationships matter |
| FBNeo | Arcade sets represented by the FinalBurn Neo ecosystem | Version and set naming are ecosystem-specific |
| Logiqx-style DATs | XML catalogue interchange across many projects | The parser format is shared; the authority and scope still differ |
| Platform catalogues | Computer disks, tapes, firmware, or specialist media | Coverage and identity fields vary widely |

An exact hash match means “these bytes match this entry under this DAT”. It
does not mean every DAT agrees, and it does not automatically settle parent or
clone relationships. A MAME clone can intentionally share some ROMs with its
parent while differing in a program or graphics region.

Watch for these ordinary DAT states:

- **BIOS dependency:** the set expects a separate BIOS or device ROM.
- **nodump:** the catalogue knows a component exists but no trustworthy dump is
  available; there is nothing to hash-match.
- **baddump:** a known dump is imperfect. Preserve the warning rather than
  pretending it is a clean match.
- **parent/clone:** the name is a relationship, not a second copy of the same
  set.
- **conflicting sources:** two DATs may use different revisions, scopes, or
  definitions. EmuWiz preserves disagreement instead of averaging it away.

Choose DATs by ecosystem and version, keep the source file and import date, and
avoid mixing a “latest” arcade DAT with an unrelated optical or cartridge
catalogue. When maintaining a large collection, store DAT snapshots so a later
change can be explained rather than appearing as a mysterious identity flip.

## ROM, set, and software object

The useful question is often “what software object is this?” rather than “what
game is this file?”

- An arcade set may require a parent set, a BIOS set, and several region or
  device ROMs.
- A Neo Geo cartridge is a multi-ROM set; program, graphics, sound and BIOS
  components are one playable object. A single ROM file cannot identify the
  complete set.
- A multi-disc PlayStation game is one software object with disc 1, disc 2 and
  sometimes an audio/data topology that must remain intact.
- A Japanese computer release can ship as boot, data, install and scenario
  disks. “Disk 2” is not a duplicate just because its title matches disk 1.
- A WHDLoad package is a directory/archive plus a validated `.slave`, and the
  slave may be the most useful exact identity anchor.
- Parent/clone relationships describe how an emulator assembles a set; they do
  not authorise deleting a clone or replacing it with its parent.

Keep set membership explicit. A good library can show the playable object,
its required members, and each physical file that provides one member.

## Safe identity and renaming

A renamer should explain *why* it proposes a name. A useful review card might
say:

| State | Plain-language meaning | Safe action |
| --- | --- | --- |
| `VERIFIED` | Exact DAT/hash or equivalent strong identity | Propose a canonical name; keep a reversible transaction |
| `STRONG` | Platform/media evidence is strong, title is not exact | Propose only a conservative family/platform name |
| `HUMAN_VERIFIED` | A person recorded a confirmation | Show the reviewer and evidence date |
| `AMBIGUOUS` | Multiple platforms, members, or interpretations remain | Do not auto-rename; ask for review |
| `CONFLICT` | Independent evidence disagrees | Preserve all sources and stop automation |
| `UNKNOWN` | Evidence is insufficient | Leave the path unchanged and explain the next useful check |

Automatic proposals should require an explicit threshold appropriate to the
operation. Exact release renames need stronger evidence than sorting an item
into a broad platform folder. Every apply operation should be previewed,
journalled, reversible, and safe to resume. Never use a random suffix to hide a
collision, and never auto-rename an ambiguous or conflicting item.

## A practical big-collection workflow

For an Internet Archive export, NAS share, or years of loose files:

1. Point the scanner at a source folder and keep the source read-only during
   the first pass.
2. Scan and classify containers, archives, sets, firmware, and sidecars.
3. Inspect strong evidence and collect DATs by ecosystem.
4. Review Unknown and Ambiguous items before making names canonical.
5. Reconcile duplicates at the software-object/set level, not only by filename.
6. Build a playing library from verified or explicitly reviewed objects.
7. Preview a destination naming profile and all proposed moves.
8. Apply as a transaction with a journal and rollback path.
9. Re-scan the destination and investigate the remaining unknowns.

Do not promise “99% supported” as a substitute for measurement. A useful
benchmark reports correct platform identity, exact identity accuracy, unknown
rate, ambiguity rate, false-positive rate, crashes, and a breakdown by system
and media type. A 99% format-recognition headline is less useful than a tiny
false-positive rate on platform identity.

## Evidence fusion: present and future

The robust model keeps evidence lanes separate and combines them at the point
where a decision is made:

```text
machine evidence  ─┐
DAT evidence      ──┼─> effective identity + confidence + reasons
human evidence    ──┤
launch evidence   ──┤
community evidence┘
```

EmuWiz already has substantial machine and DAT evidence paths, plus human
review in selected workflows. Community evidence and a full cross-lane human
ground-truth service remain future-facing concepts, not claims of a released
feature.

Planned human labels should be explicit (`USER_LABELLED`, `USER_TESTED`,
`USER_VERIFIED`) and bound to the observed file/hash or set. A correction should
retract or supersede the old observation rather than silently deleting history.
“I played this in Amiberry” is excellent `USER_TESTED` evidence for emulator
compatibility; it is not a replacement for an exact DAT match.

Planned opt-in community evidence follows the same rule: submit a hash-bound
claim and platform/title/emulator result, never ROM bytes by default, private
paths, or filenames. Agreement can raise confidence; conflict remains visible.

## Benchmarking real corpora

Treat the corpus as a test set, not a marketing number. Track:

- platform accuracy and exact identity accuracy separately;
- Unknown and Ambiguous rates, with reason buckets;
- false-positive rate, especially for shared extensions;
- crash/panic rate and bounded-read failures;
- per-system and per-media breakdowns;
- tape timing tolerance and optical track-topology accuracy.

For tapes, compare logical recovered blocks and checksums across sample rates,
speed drift, and noise. For optical media, compare track count, modes, indexes,
pregaps, subchannel state, and logical hashes—not merely the outer CHD file
hash. Keep a held-out corpus so a rule does not “learn” from the same reviewed
recording it is later scored against.

## What to do with an Unknown or Ambiguous item

Use this decision tree:

```text
Unknown
├─ unsupported extension or parser? -> keep original, record a research gap
├─ damaged/truncated?              -> obtain a better dump; do not repair in place
├─ generic container?               -> inspect internal boot/filesystem evidence
├─ archive member hidden?           -> list safely, preserve member provenance
├─ incomplete set?                  -> find required discs/BIOS/parent members
├─ missing DAT?                     -> import the correct ecosystem/version
└─ none of the above?               -> request human confirmation

Ambiguous
├─ multiple platform candidates?    -> preserve all and review independently
├─ conflicting DATs?                -> record source/version disagreement
├─ weak boot evidence?              -> treat as family-only
├─ generic FAT/geometry?            -> do not upgrade to a platform
└─ set conflict?                    -> inspect members, never choose a winner silently
```

The next action should reduce uncertainty without changing the source bytes.
That is the core preservation habit.

## Related chapters

- [ROM dumps and verification](21-rom-dumps-and-verification.md)
- [Disc images, CHD, RVZ and WUA](22-disc-images-chd-rvz-and-wua.md)
- [ROM managers and DAT files](23-rom-managers-and-dat-files.md)
- [Arcade and MAME sets](37-arcade-and-mame-sets.md)
- [Computer systems, keyboards and media](38-computer-systems-keyboards-and-media.md)
- [Internet Archive and public preservation](84-internet-archive-and-public-preservation.md)
- [Maintenance, audits and the long game](40-maintenance-audits-and-the-long-game.md)
