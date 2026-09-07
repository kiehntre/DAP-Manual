# DATs, Metadata Providers and the Limits of the Web

DATs are powerful because they turn a byte sequence into a catalogue identity,
but they are not a universal truth service. No-Intro, Redump, MAME, FBNeo and
Logiqx-style catalogues describe different preservation communities and often
different objects.

## What a DAT can and cannot prove

An exact hash match can identify a known dump in that DAT. It does not prove the
dump is the only valid version, that a set is complete, or that the DAT’s title
matches your preferred naming policy. A MAME set may describe parent/clone and
BIOS dependencies that a console ROM DAT does not. Redump’s per-track optical
identity is not the same as the SHA-1 of a CHD container.

Keep these distinctions visible:

| Evidence | Strong answer | Not necessarily answered |
| --- | --- | --- |
| No-Intro hash | known cartridge/ROM dump | emulator readiness or set completeness |
| Redump logical track hashes | disc track identity and topology | whether a CHD was built with every preservation detail |
| MAME/FBNeo set record | arcade set relationship | a standalone “game file” identity |
| BIOS DAT | firmware revision | whether an emulator can find it in its configured path |
| Human launch observation | this exact content ran in this environment | universal machine verification |

Do not average disagreement between DATs. Preserve the source, revision and
confidence. A stale DAT and a newer DAT can both be internally correct for their
respective catalogues.

## No-Intro imports and managed snapshots

A practical DAT workflow is to import an official or user-selected pack into a
managed, content-addressed snapshot. Record source URL or file provenance,
import time, format and digest. Browser-assisted acquisition can be useful when
a site requires a normal user download, but the application should not depend on
brittle scraping or a hidden downloader.

Keep games-only filters, BIOS entries, headered/headerless variants and regional
families explicit. “Latest DAT” is not enough information to reproduce a
decision later.

## Metadata enrichment is optional

MobyGames, HLTB, LaunchBox metadata, RomM enrichment and similar services can
make a library nicer. They can also rate-limit, change schemas, disagree, or go
offline. Cache successful results, make the request path explicit, and never
make core identity depend on one website. HLTB durations, for example, are
useful cached RomM metadata, not a reason to make a browsing-time network call.

The same applies to SSRF safety: an arbitrary metadata URL must not become a way
to probe local services. Validate destinations, limit response size and time,
and treat enrichment failure as “metadata unavailable”, not “game unknown”.

## A review loop that scales

1. Choose the DAT family appropriate to the media.
2. Import and pin a snapshot.
3. Run exact matches first.
4. Review headered/headerless and parent/clone distinctions.
5. Keep unmatched and conflicting entries visible.
6. Plan any rename or organisation from the evidence.

Never download a DAT and immediately rename an entire collection. A wrong DAT
family can produce an impressively tidy, entirely incorrect library.

## Provider disagreement is normal

One catalogue may call a release a parent, another may list a standalone dump,
and a third may omit it because it is a BIOS or a bad dump. Keep the catalogue
name and revision beside the result. If two exact hashes have different titles,
show a conflict and let the user investigate region, revision or catalogue
scope. Do not invent a blended title.

The same restraint applies to cover art and playtime services. Artwork is a
presentation asset; it cannot repair a weak identity match. A cached duration is
helpful in Game Details, but it should never determine which file gets renamed.

## The useful maintenance record

For each managed snapshot retain: source/provenance, import date, digest,
parser version, number of records and any import warnings. Pin the snapshot
used by a rename plan. When a new DAT arrives, compare its results rather than
silently rewriting old decisions. This makes a later audit answerable: “which
catalogue caused this name?”

## Choosing the next verification step

If a cartridge has an exact No-Intro match but the emulator reports a missing
BIOS, fix readiness rather than hunting for another title. If an optical image
has no Redump match but has a coherent track layout, retain the logical evidence
and inspect the set or DAT coverage. If an arcade archive has a parent match but
missing clone members, repair set completeness rather than renaming the parent.

This separation saves time: identity questions, completeness questions and
launch questions often need different evidence. A metadata provider may answer
“what is the release year?” while a DAT answers “which bytes are these?” Neither
should be forced to answer the other question.

For practical identity layering, see [Evidence-Based Identification](102-evidence-based-identification.md) and [Rom Managers and DAT Files](23-rom-managers-and-dat-files.md).
