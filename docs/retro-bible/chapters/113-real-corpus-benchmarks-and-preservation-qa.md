# Real-Corpus Validation, Strange Media and the Long View

Synthetic fixtures prove that a decoder can work. Real collections tell you
whether it works without becoming confidently wrong. Benchmarking should count
correct platform, exact identity, family-only identity, unknown, ambiguous,
conflict, malformed and unsupported outcomes separately.

## What to measure

Report by platform, media type and evidence source:

- platform accuracy;
- exact identity accuracy;
- unknown and ambiguity rates;
- false-positive rate;
- checksum or logical-track recovery;
- crash/panic rate;
- reasons for abstention;
- time and I/O cost.

False positives should be punished more than Unknown. “99% supported formats”
is less useful than “99% correct platform identity with a tiny false-positive
rate.” A small class with two reviewed examples is not a mature result.

### Tape validation

Use clean TAP/TZX/UEF/CAS examples alongside noisy WAV captures, resampled
audio, clipped edges, jitter, malformed blocks, standard ROM loaders and custom
turbo loaders. Measure platform correctness, block recovery, loader class,
checksum integrity and refusal of random FSK-like noise. Two WAVs can have
different sample rates and absolute timing yet represent the same logical
program; compare calibrated structure, not raw sample indexes.

Current EmuWiz tape work is **implemented/partial** across Spectrum, C64,
Amstrad CPC, BBC, MSX, Atari 8-bit, Dragon/CoCo and related formats. Named
commercial-loader claims remain deliberately conservative.

### Optical validation

Test ISO, BIN/CUE, CHD, GDI, CDI, mixed-mode discs, audio tracks, pregaps,
indexes and subchannel states. Compare logical track content and topology where
provable. A CHD container digest is not a Redump per-track identity. Specialist
layouts should be refused or marked unverified rather than flattened into a
false match.

## Strange, non-retail and research media

Prototypes, alpha/beta and debug builds, review discs, kiosk media, SDK samples,
diagnostics, magazine coverdiscs, homebrew and preservation-only dumps are not
garbage because they fail a retail DAT. Classification and exact identity are
separate. Keep “historically valuable, exact title unknown” as a useful result.

The same restraint applies to N-Gage, Symbian, J2ME, Palm OS, Pocket PC,
Dreamcast VMU, Neo Geo 64, Satellaview/BS-X, PICO, Beena, V.Smile, Leapster,
StudyBox, Pocket Challenge, Picno, Design Master, Little Jammer, Copera, Omni,
Project EGG and kiosk/distribution media. Some have structural clues; many need
DAT/hash evidence or a human. EmuWiz coverage is mixed and research-oriented,
not a promise of universal exact identity.

## Museum Mode: future context

Museum Mode is a **planned/future** optional view for platform history,
manufacturer, release year, hardware generations, media formats, region and
software timelines, collection statistics and preservation oddities. It should
add context without making ordinary scanning slower or forcing a clean-library
workflow to become a museum catalogue.

## Sunshine, Moonlight and home servers

Streaming changes the failure surface: a game can launch correctly while a
dummy display, GPU passthrough, controller-forwarding path, audio device or
shader cache is wrong. Test server-side emulation locally first, then through
Sunshine/Moonlight at the intended client. Record input latency and audio drift;
do not confuse a streaming problem with an emulator identity problem. Keep
network storage mounts read-only where possible.

## Storage and archive horror stories

If a USB bridge reports a zero-byte disk, an NTFS superblock error appears, or a
copy stalls, stop writing to the source. Try a known-good cable and bridge,
capture a read-only image or copy to a healthy disk, and work on the copy. UAS
quirks, failing power and wrong cables are common enough to deserve suspicion.

RAR files masquerading as ZIPs, split archives, partial downloads, archive bombs,
path traversal and enormous member counts all require bounded inspection. A
safe reader limits decompression, member count, path depth and output size.

Do not “repair” a suspicious archive by extracting it into the source tree. Use
a quarantine directory, record the archive digest and extraction tool version,
and inspect the member list first. If a decompressor reports a CRC error, retain
the original and note which member failed. A partial extraction can be useful as
research evidence, but it must not be mistaken for a complete playable set.

## Performance without recklessness

Incremental indexes, local caches, bounded metadata reads, limited parallelism
and deferred deep hashing make a collection usable on slow HDDs, NAS mounts and
large WAV libraries. Hash everything only when the evidence question requires
it. A faster scan that silently misses a changed source is not an optimisation;
it is an identity bug.

## What not to automate

Do not auto-rename a conflict, invent a missing BIOS, decode an unknown encrypted
cheat, select a community-majority title as proof, convert a destructive format,
or guess a platform from an extension. Automate repetitive inspection and clear
matches. Ask a human when the human actually has information the machine lacks.

The project’s long-term philosophy is simple: preserve first, identify from
evidence, explain decisions, mutate explicitly, make changes reversible, prefer
Unknown to Wrong, automate the boring parts, and never punish a user for having
a messy real-world collection.

## A practical benchmark folder

Keep a small, lawful fixture corpus with a manifest rather than embedding user
media in the test repository. Include one clear example, one near miss, one
malformed file, one ambiguous container and one unsupported specialist layout
per important family. Store expected *evidence outcomes*, not just expected
titles: “CPC platform, checksum valid, filename recovered” is more robust than
one string comparison.

For each run, save tool version, fixture manifest version, sample rate or disc
topology, elapsed time and refusal reasons. Compare runs by category. A decoder
that improves recovery but begins labelling generic noise as a named loader has
regressed, even if its headline success count went up.

## From review to roadmap

The benchmark should produce data needs, not just a score. If advert examples,
weekend programme resumes, multi-disc sets or Japanese hard-disk images are
underrepresented, say so plainly and collect those cases next. Do not turn a
small, convenient corpus into a claim about every Internet Archive collection.

## A failure taxonomy beats a single red number

When a case fails, classify the reason: unsupported container, malformed input,
missing companion, weak platform evidence, conflicting DATs, incomplete set,
missing firmware, executable discovery failure, or genuine parser defect. This
lets a maintainer improve the right layer. Otherwise a benchmark can report a
lower “success” rate while actually improving safety by refusing dangerous
guesses.

The same taxonomy helps users. “Needs review: no DAT record” is a different
task from “Needs repair: archive member is truncated”. A future review queue can
prioritise by confidence, reversibility and likely user value without pretending
that every unknown has the same cause.

## Evidence-preserving test fixtures

Fixtures should include provenance notes, expected platform family, permitted
hashes and why a negative case must remain negative. For tape, retain sample
rate and calibration parameters. For discs, retain track mode, pregap/index and
subchannel expectations. For archives, retain member count and bounded-size
limits. This makes a regression explainable instead of reducing it to a mystery
binary blob.

See [Obscure Platforms and Preservation Context](105-obscure-platforms-and-preservation-context.md), [Optical and Tape Preservation](103-optical-and-tape-preservation.md), and [Troubleshooting Reference](100-troubleshooting-reference.md).
