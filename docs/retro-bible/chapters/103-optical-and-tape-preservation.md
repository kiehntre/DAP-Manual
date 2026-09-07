# Optical and Tape Preservation

Optical discs and cassette tapes are physical protocols captured into files.
They are not ordinary “ROM files with a different extension”. The file format
may preserve layout, timing, error information, and gaps that an emulator needs
even when the visible game data is identical.

This chapter explains how to preserve those layers without confusing a
container, a logical program, and an exact catalogue identity.

## Optical media: preserve the map, not only the bytes

### Common representations

| Representation | What it is good at | What can be lost or hidden |
| --- | --- | --- |
| ISO | A readable data filesystem | Audio tracks, unusual sessions, subchannel detail |
| BIN/CUE | One or more raw track payloads plus topology | Fidelity depends on the reader and CUE description |
| Raw sectors | Exact sector stream at a chosen size | Track metadata may live in a separate description |
| CHD | Compressed, seekable optical/container representation | The CHD hash is not automatically a Redump logical match |
| GDI | Dreamcast GD-ROM track layout | Multiple files must remain together |
| CDI | Dreamcast-oriented disc image | Tool-specific variants and imperfect dumps exist |

Mixed-mode discs commonly have a data track followed by audio tracks. A game
can boot successfully while its music, pregaps, or index points are wrong. A
preservation workflow therefore records track number, track type, sector mode,
start/end, pregap and index information when available.

### Pregaps, indexes, and subchannels

A **pregap** is intentional timing or sector space before a track. An **index**
marks a position within a track. The two are not interchangeable. Subchannel
data can carry disc-control information, CD+G-style material, or copy-control
signals; a data-only rip may legitimately have no subchannel bytes.

Record whether subchannel data is present, absent, unavailable, or unknown. Do
not turn “not read” into “not present”. If a tool cannot preserve an unusual
layout, keep the original image and label the conversion as lossy or
unsupported.

### CHD is a container, Redump is a logical expectation

CHD compression gives a compact, seekable container. Redump-style verification
usually describes expected logical tracks and their hashes. These are different
questions:

```text
CHD container SHA-1  !=  Redump per-track logical identity
```

Two CHDs can have different container hashes but decode to the same logical
tracks. Conversely, a CHD can have a familiar title and a valid header while a
track, pregap, mode, or audio payload differs. A safe verifier compares bounded
logical tracks, modes, lengths and available hashes, preserving outcomes such
as **MATCH**, **MISMATCH**, **UNVERIFIED**, **UNSUPPORTED**, or **INCOMPLETE**.

EmuWiz's current CHD/Redump work is deliberately bounded. It can verify proven
logical tracks without rewriting the CHD or loading a complete disc into memory.
Unusual specialist layouts, non-zero pregap seeking, and unavailable
subchannel bytes remain explicit limitations rather than invented matches.

### CUE/BIN and CHD parity

Logical parity is strongest when both representations expose the same track
number, mode, sector size, payload length and hashable data. A CUE can describe
more topology than a flat BIN; a CHD may retain details unavailable through the
current reader. Report “logical match” only for the fields actually compared.
Do not call two images identical because their game title or outer filename
matches.

### Families already represented in EmuWiz

The current evidence layer has specialist or platform paths for Saturn, Sega
CD/Mega CD, PC Engine/TurboGrafx-CD, 3DO, Neo Geo CD, Dreamcast, CD-i and
PlayStation-related media, alongside shared ISO/CUE/raw/CHD infrastructure.
Coverage depth differs. A boot signature can establish a platform candidate;
Redump or another authoritative DAT remains the route to exact release
identity.

Preservation-safe optical checklist:

1. Keep the original dump read-only and record its checksum.
2. Keep every CUE, GDI, M3U, CCD, subchannel file, and sidecar with its image.
3. Validate track topology before compressing or reorganising.
4. Keep a lossless source if producing CHD/RVZ/other derivatives.
5. Verify logical tracks after conversion rather than trusting the output name.
6. Never “repair” a CUE by guessing a track mode or pregap.

## Tape preservation: timing is data

Cassette formats sit on a spectrum from decoded records to sampled audio:

| Layer | Examples | Preservation question |
| --- | --- | --- |
| Decoded byte container | TAP, CAS | Are records, checksums and gaps represented correctly? |
| Structured pulse container | TZX, UEF | Are timing blocks, pilots and special chunks retained? |
| Sampled capture | WAV/PCM | Can edges and symbols be recovered without inventing bytes? |

Two WAV captures can look very different in amplitude or sample rate while
representing the same logical program. A good decoder calibrates local timing,
tracks provenance, and compares recovered blocks/checksums rather than raw PCM.

### Standard versus turbo/custom loaders

Standard ROM loaders usually have recognizable leaders, sync pulses, bit timing,
framing and checksums. Turbo or custom loaders may change pulse durations,
symbol encoding, stage order or speed. The conservative order is:

1. detect edges and calibrate locally;
2. find a stable pilot/leader;
3. validate sync and byte framing;
4. recover a bounded block;
5. verify checksum or known structure;
6. only then consider a custom stage anchored to the standard evidence.

Timing similarity alone is not a platform identity. Random dual-tone noise must
remain generic or UnknownCustom. EmuWiz's tape work preserves confidence,
ambiguity, sample/time ranges and partial recovery rather than forcing a named
loader.

### The current families

EmuWiz has documented and/or implemented bounded paths for:

- **ZX Spectrum:** TAP/TZX semantics and WAV standard/custom stages;
- **Commodore C64:** standard Datasette waveform and bootstrap-gated custom
  evidence;
- **Amstrad CPC:** standard CPC blocks plus CPC-anchored generic custom stages;
- **BBC Micro:** standard waveform evidence;
- **MSX:** standard waveform/CAS-related evidence with known launch limits;
- **Atari 8-bit:** standard and custom audio work documented by versioned tape
  notes;
- **Dragon/CoCo:** ordinary CAS/WAV scope, with turbo/custom boundaries;
- **UEF:** structured container semantics where the current parser supports
  them; unsupported chunks remain visible as unsupported.

The exact implementation status changes as the project grows. The companion
technical notes live in EmuWiz at
`/home/davedap/archivefs/docs/DEEP_TAPE_AUDIO_V4.md`; this chapter is the
user-facing explanation.

### Carrier, gaps, and checksums

Carrier tone gives a decoder a timing reference. Gaps separate records or
stages. A checksum protects a block but does not identify a commercial loader
or title. Load and execute addresses are metadata recovered from a valid format
header, not guesses based on where a block happened to occur.

For WAV captures, preserve the sample rate, channel choice, capture equipment
notes, polarity if known, and any preprocessing. Do not normalize away clipping
or DC offset and then discard the original. Store derived edge/timing evidence,
not unlimited raw PCM in the library database.

### Tape troubleshooting

**Nothing is detected:** verify the file is PCM WAV, listen for the carrier,
check that the correct channel was captured, and try a known-good player before
changing thresholds.

**A leader appears but no block is valid:** the recording may be truncated, have
speed drift, or contain a custom sync/data stage. Keep it as partial evidence;
do not call it a valid program.

**A block works at one speed only:** inspect local calibration and resampling.
Do not use a global scale when a multi-stage loader changes speed.

**The program loads but the identity is Unknown:** successful playback proves
compatibility, not exact release identity. Add a human note or obtain the right
DAT rather than renaming from the screen title alone.

**A `.cas` file is rejected:** CAS is not one universal format. Atari, MSX and
Dragon/CoCo conventions differ. Keep the source and select the parser matching
the machine family.

### Practical tape checklist

- Keep the original WAV and a checksum.
- Use a lossless format; do not convert a source capture to MP3.
- Preserve sample rate and channel information.
- Record recovered block ranges and checksum state.
- Treat malformed/noisy captures as evidence, not as invitations to fabricate
  bytes.
- Compare logical records across captures when testing equivalence.

## Related chapters

- [Disc images, CHD, RVZ and WUA](22-disc-images-chd-rvz-and-wua.md)
- [Rom dumps and verification](21-rom-dumps-and-verification.md)
- [Computer systems, keyboards and media](38-computer-systems-keyboards-and-media.md)
- [Amstrad CPC](78-amstrad-cpc.md)
- [ZX Spectrum](79-zx-spectrum.md)
- [Commodore systems](76-commodore-systems.md)
- [Amiga](77-amiga.md)
