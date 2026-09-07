# Weird Stuff, Non-Retail Media, and Museum Mode

The edges of a collection are where filename-based identification fails most
often. A kiosk demo, a Symbian SIS package, a VMU mini-game, and a Neo Geo 64
cartridge may all be real preservation objects while having little shared
catalogue support. The correct response is not to throw them away or to invent
a confident title.

## A field guide to unusual systems

The table is an audit guide, not a promise that every item has a released
EmuWiz parser or launcher.

| Family | Useful evidence | Safe current posture |
| --- | --- | --- |
| N-Gage | Package metadata, platform headers, verified hashes | DAT/hash-led; do not infer from a generic mobile archive |
| Symbian | SIS/SISX structure, UID, signing metadata | Structural identity is possible; exact release needs a catalogue |
| J2ME | JAR manifest, MIDlet metadata, device/profile information | Many regional builds; preserve vendor/version fields |
| Palm OS | PRC/PDB headers and creator/type IDs | Header evidence can identify family; exact application may need a catalogue |
| Pocket PC | CAB/PE metadata and platform version | Separate executable identity from installed application state |
| Dreamcast VMU | VMU header, icon/data layout, save/game association | Preserve as a companion object, not a standalone console ROM |
| Neo Geo 64 | Multi-ROM set structure and MAME/software-list identity | Never reuse ordinary Neo Geo cartridge identity |
| Satellaview / BS-X | Broadcast metadata, memory map, event/date context | Date/event identity is often as important as title |
| PICO / Beena | Platform-specific headers, paired software/audio/media | Child-focused media can be a set, not a single ROM |
| V.Smile | Cartridge/container structure and region/catalogue | DAT/hash-led; generic educational cartridges are easy to mislabel |
| Leapster / LeapPad / Explorer | Cartridge or package headers, device generation | Family evidence first; exact edition may remain Unknown |
| StudyBox / Pocket Challenge | Proprietary cartridge/package metadata | Human confirmation and hash evidence are likely necessary |
| Picno / Design Master | Device-specific cartridge or disk structure | Research-only until a bounded parser and corpus exist |
| Little Jammer | Media/card identity plus hardware model | Keep accessory/media relationships explicit |
| Copera / Omni | Specialist cartridge/software metadata | Do not collapse into the nearest mainstream platform |
| Project EGG | Store/container metadata and original platform identity | Digital-release metadata is not a ROM hash match |
| Kiosk/demo/distribution media | Disc labels, build IDs, executable strings, provenance | Classify the release type separately from game identity |

“DAT/hash-led” is a useful answer. It says the safest path is an authoritative
catalogue or human confirmation, not that the media is unimportant.

## Classify before you identify

For unusual items, record three independent fields:

1. **Structural family:** what format or hardware does the object validate as?
2. **Software identity:** what title, build, or set does evidence support?
3. **Preservation class:** retail, prototype, debug, kiosk, sample, homebrew,
   broadcast event, or unknown.

This avoids the common mistake of treating “not in a retail DAT” as “not real”.
A debug build may have a perfect internal version string and still have no
retail catalogue entry. A Project EGG release may have excellent store metadata
but no byte-identical relation to a cartridge dump.

## Community evidence (planned)

A future opt-in community evidence service could accept a statement such as:

```text
hash: <content identity>
platform: Amiga
title: Example prototype
emulator: Amiberry
launch result: successful
review label: USER_TESTED
```

That service is **planned**, not a claim that the current DAP Bible or EmuWiz
ships it. Submissions should be hash-bound, should not upload ROM bytes by
default, and should never include private filesystem paths or filenames unless
the user explicitly chooses to share them. Agreement can be displayed as
community corroboration; disagreement must remain visible and cannot override a
machine or DAT result automatically.

## Human ground truth (planned architecture)

Human review is strongest when it is precise about what was observed:

- `USER_LABELLED`: a person supplied a label or likely identity;
- `USER_TESTED`: a person tested the item in a named emulator/profile;
- `USER_VERIFIED`: a person compared evidence and confirmed the result.

Bind the observation to a content hash, set identity, or other stable evidence
where possible. Keep the reviewer, date, scope and confidence. A correction
should supersede the old observation with an audit trail rather than silently
rewriting history.

Example: “I know this is Amiga because I played it in Amiberry” is useful
`USER_TESTED` compatibility evidence. It does not prove the exact regional
release, the original disk dump's completeness, or a DAT hash.

## Evidence fusion without false certainty

The intended model is a set of labelled lanes:

| Lane | Example | What it should not pretend to be |
| --- | --- | --- |
| Machine | NEC boot marker, X68000 IPL, CHD track mode | A title catalogue match |
| DAT | Redump logical track match | Universal agreement across DAT families |
| Human | Reviewer confirms a prototype | An automated parser fact |
| Launch | Game starts in FS-UAE | Proof of original dump identity |
| Community | Several hash-bound users agree | A replacement for local verification |

Effective identity is a careful presentation of these lanes and their limits.
It is not a vote where five weak signals defeat one explicit conflict.

## Museum mode (planned)

The DAP Museum is a future optional view, not a requirement for ordinary
library management. It could provide historical context alongside a practical
play button:

- platform history, manufacturer and release year;
- hardware generations and regional variants;
- media formats and preservation oddities;
- collection statistics and timelines;
- notable releases, prototypes, debug builds and demos;
- why an item is historically important even when it is not launch-ready.

Museum data should be additive. A user who wants to organise files should not
have to read an essay about a chipset before fixing a missing BIOS. Historical
notes should link back to the evidence and clearly distinguish sourced fact,
human annotation and editorial context.

## Large-collection triage for weird media

When a scan finds an unfamiliar object:

1. Keep it in an “unreviewed preservation” area.
2. Compute a stable hash and record size/mtime without changing the file.
3. Identify the outer container and list members safely.
4. Search specialist DATs/catalogues by platform family, not by filename alone.
5. Record a human note if the object is known from personal or community
   experience.
6. Only then decide whether to publish it to a playing library.

This workflow protects rare material from accidental conversion or renaming
while still making it discoverable.

## Related chapters

- [Other important systems](83-other-important-systems.md)
- [Internet Archive and public preservation](84-internet-archive-and-public-preservation.md)
- [Preservation projects and community efforts](85-preservation-projects-and-community-efforts.md)
- [The future of emulation](88-future-of-emulation.md)
- [Glossary](92-glossary.md)
