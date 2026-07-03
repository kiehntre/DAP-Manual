# DAP Retro Bible Style Guide

This document is the editorial constitution for the DAP Retro Bible.

The DAP Retro Bible is a Linux-first retro gaming, emulation, streaming and preservation book. It should read like a professional technical reference, not a pile of notes glued together with hope and caffeine.

Every future chapter should follow this guide unless the project deliberately revises the standard.

## Core Principles

- Write in UK English.
- Treat the book as a publication, not a README.
- Prefer accuracy over speed.
- Prefer tested instructions over repeated internet folklore.
- Explain why something matters, not only what to click.
- Keep Linux as the primary platform.
- Mention Windows, macOS, Android or handhelds only where useful.
- Keep legal, technical and historical topics clearly separated.
- Use open source tools where practical.
- Record real-world behaviour, including failures and workarounds.
- Do not recommend flavour-of-the-month tools without context.

## Audience

Write for readers who may be:

- new to emulation;
- comfortable with Linux but new to retro preservation;
- experienced retro users who want cleaner structure;
- homelab users building shared libraries;
- frontend users trying to understand what sits underneath;
- preservation-minded readers who care about naming, checksums and long-term access.

Do not assume the reader knows emulator jargon. Explain it once, then use the correct term consistently.

## Voice and Tone

The tone should be:

- clear;
- practical;
- historically aware;
- confident without pretending every answer is final;
- friendly, but not jokey for the sake of it;
- suitable for a book someone may read years from now.

Avoid throwaway hype such as "ultimate", "perfect", "best ever" or "just works" unless the claim is carefully explained.

## Markdown Conventions

- Use one H1 per file.
- Use H2 for main sections.
- Use H3 for subsections.
- Do not skip heading levels.
- Keep paragraphs short enough for web reading.
- Use fenced code blocks with a language tag.
- Use tables for comparisons, compatibility and decision matrices.
- Prefer plain Markdown that works cleanly with MkDocs.
- Avoid raw HTML unless there is a clear reason.

Example:

```markdown
## BIOS Requirements

| System | Required | Common filenames | Notes |
| --- | --- | --- | --- |
| Sega Saturn | Yes | `sega_101.bin`, `mpr-17933.bin` | Region matters. |
```

## File and Path Style

Use Linux paths by default.

Preferred examples:

```text
/mnt/games/roms/gamecube/
/mnt/games/bios/
/home/davedap/retrodeck/bios/
```

Rules:

- Use lowercase system folder names unless a tool requires otherwise.
- Use kebab-case for documentation filenames.
- Put chapters in `docs/retro-bible/chapters/`.
- Put appendices in `docs/retro-bible/appendices/`.
- Put images in `docs/retro-bible/assets/`.
- Do not hard-code private tokens, keys or credentials.

## Image Naming

Use lowercase kebab-case filenames.

Preferred pattern:

```text
<chapter-number>-<subject>-<short-description>.<extension>
```

Examples:

```text
01-folder-layout-diagram.png
04-dolphin-controller-settings.png
10-redump-verification-flow.svg
```

Every image should have:

- useful alt text;
- a short caption where needed;
- enough surrounding explanation to make sense without the image.

Screenshots should be cropped to the relevant area. Do not include private paths, tokens, personal email addresses or API keys.

## Code Blocks

Use code blocks for commands, configuration files and command output.

Shell commands:

```bash
mkdir -p /mnt/games/{bios,roms,saves,states,texture-packs,patches,metadata}
```

Configuration snippets:

```yaml
site_name: DAP Retro Bible
```

Command output:

```text
BIOS check passed: sega_101.bin
```

Do not include a shell prompt unless the prompt itself matters.

## Tables

Use tables for:

- emulator comparisons;
- archive format choices;
- BIOS requirements;
- frontend feature comparisons;
- preservation project summaries;
- compatibility notes.

Keep table headings short. If a cell needs a paragraph, the table is probably trying to do too much.

## DAP Callouts

Use consistent callouts so the book develops its own readable rhythm.

### DAP Tip

Use for practical advice that saves time or avoids confusion.

```markdown
> **DAP Tip**
>
> Keep BIOS files in one controlled folder and symlink them into frontend-specific paths only when needed.
```

### DAP Warning

Use for risks involving data loss, broken libraries, account issues, legal concerns or major wasted time.

```markdown
> **DAP Warning**
>
> Do not bulk-convert a verified set until you have tested the target emulator and kept a backup of the original files.
```

### DAP Deep Dive

Use for optional technical detail.

```markdown
> **DAP Deep Dive**
>
> CHD stores disc data in a compressed format originally associated with MAME. Emulator support varies by system.
```

### DAP History

Use for historical context.

```markdown
> **DAP History**
>
> The Sega Dreamcast was one of the first consoles to make online play part of its identity, even though regional support varied heavily.
```

### DAP Myth

Use for common misunderstandings.

```markdown
> **DAP Myth**
>
> Bigger texture packs are not automatically better. Some are inconsistent, incomplete or much heavier than the hardware needs.
```

## Hall of Fame Format

Hall of Fame entries should celebrate projects, people, hardware or services that made preservation, access or understanding better.

Use this structure:

```markdown
## Name

**Category:** Emulator, preservation project, frontend, hardware, service or community.

**Why it matters:** Explain the contribution clearly.

**Historical context:** Explain when it appeared and what problem it solved.

**Legacy:** Explain why it still matters.

**Further reading:** Link to official or primary sources where possible.
```

Keep praise grounded. No statue-polishing unless the statue earned it.

## Hall of Shame Format

Hall of Shame entries must be factual, careful and evidence-based.

Use this structure:

```markdown
## Name

**Category:** Company, service, product, policy or incident.

**What happened:** Describe the issue plainly.

**Why it matters:** Explain the preservation or user impact.

**Evidence:** Prefer primary sources, official statements, archived material or respected reporting.

**Lesson:** Explain what future projects can learn.
```

Rules:

- Do not turn Hall of Shame entries into rants.
- Avoid personal insults.
- Focus on decisions, outcomes and evidence.
- If evidence is uncertain, say so.

## Citation Style

Prefer primary and durable sources:

1. Official project documentation.
2. Source code repositories.
3. Preservation project documentation.
4. Archived official pages.
5. Books, interviews and developer talks.
6. Reputable reporting.

Citation rules:

- Cite historical claims that are not common knowledge.
- Cite technical claims when they depend on a specific tool version or project policy.
- Cite legal or store-shutdown claims carefully.
- Prefer archived links when documenting lost services.
- Do not cite random forum posts as fact unless clearly labelled as anecdotal evidence.

## Terminology

Use consistent terms:

| Term | Use |
| --- | --- |
| ROM image | Cartridge or memory dump. |
| Disc image | CD, DVD, GD-ROM, Blu-ray or similar optical media dump. |
| BIOS | Low-level system firmware needed by some emulators. |
| Firmware | Broader system software used by consoles and devices. |
| Frontend | A launcher or library interface such as ES-DE, LaunchBox or Pegasus. |
| Emulator | Software that recreates another system. |
| Core | A libretro emulator module used by RetroArch or similar tools. |
| Preservation set | A verified or organised collection based on a known standard. |
| Metadata | Titles, artwork, descriptions, release dates and related library data. |

## Preservation Toolkit Coverage

The Preservation Toolkit section should explain what each project does, what it does not do and where it fits.

Planned coverage includes:

- Redump;
- No-Intro;
- TOSEC;
- MAME;
- ScreenScraper;
- SteamGridDB;
- Wii U Downloader;
- NoPayStation-style workflows where legally appropriate;
- DAT verification;
- checksums;
- archive formats including CHD, RVZ, WUA, CSO, ZIP and 7z.

Do not blur preservation databases, metadata scrapers and download tools into one bucket. They solve different problems.

## Emulator and Frontend Coverage

When covering an emulator, include:

- supported systems;
- Linux installation options;
- BIOS or firmware requirements;
- preferred file formats;
- controller notes;
- save and state locations;
- performance notes;
- frontend integration;
- known issues;
- official documentation links.

When covering a frontend, include:

- what it manages;
- what it does not manage;
- folder expectations;
- metadata sources;
- artwork handling;
- emulator launching;
- controller behaviour;
- where configuration lives on Linux.

Frontends are views over the collection. They are not the source of truth.

## Real-World DAP Notes

Use **Real-World DAP Setup** sections when documenting behaviour tested on the DAP environment.

Include only useful details, such as:

- Ubuntu or Nobara host notes;
- Docker services;
- ES-DE paths;
- RetroDECK paths;
- RomM behaviour;
- Sunshine/Moonlight streaming notes;
- symlink strategies;
- network storage considerations.

Example:

```markdown
## Real-World DAP Setup

On the DAP setup, shared ROM storage lives under `/mnt/games/roms/`, while frontend-specific folders use symlinks where practical.
```

## Emoji Usage

Use emoji sparingly.

Rules:

- Do not use emoji in headings.
- Do not use emoji in technical tables.
- Do not use emoji in commands, filenames or configuration.
- Occasional emoji may appear in informal project notes, but not in core reference prose.

The book should age well. Disco lights are for the arcade cabinet, not every paragraph.

## Chapter Template

Use this template for normal chapters:

```markdown
# Chapter Title

Opening paragraph explaining what the chapter solves and why it matters.

## Historical Context

Explain relevant background.

## Concepts

Define the core ideas readers need before following the tutorial.

## Practical Setup

Give tested, ordered instructions.

## Recommended Layout

Show paths, folder structures or configuration patterns.

## Real-World DAP Setup

Document tested behaviour from the DAP environment when relevant.

## Common Mistakes

List common traps and how to avoid them.

## Troubleshooting

Describe symptoms, causes and fixes.

## Key Points

Summarise the chapter.

## Further Reading

List official or durable sources.
```

## Chapter Quality Checklist

Before a chapter is considered ready, check that it:

- has one clear subject;
- follows the chapter template where appropriate;
- uses UK English;
- explains jargon;
- separates fact, opinion and personal experience;
- includes practical Linux paths where useful;
- avoids unsupported legal claims;
- avoids unexplained acronyms;
- has consistent heading levels;
- has useful tables only where they help;
- includes citations for historical or version-sensitive claims;
- does not depend on screenshots alone;
- can be maintained later without guessing what the author meant.

## Git and Repository Rules

For this book:

- One chapter per commit.
- One file per commit unless explicitly agreed otherwise.
- Verify every GitHub action before reporting success.
- Verify every created or modified file exists after committing.
- Never claim a commit succeeded without checking.
- Never claim a file exists without checking.
- Keep commit messages clear and specific.

## Long-Term Publishing Goal

Markdown should remain the single source of truth.

The project should be suitable for:

- GitHub Pages;
- MkDocs;
- searchable web output;
- printable PDF;
- EPUB or other book formats later.

Do not write content that only works in one output format unless there is a deliberate reason.

## Book Structure

The current planned structure is:

### Part I: Foundations

- Preface
- Why Preservation Matters
- Hall of Fame
- Hall of Shame

### Part II: Preservation Toolkit

- Preservation Toolkit
- Redump
- No-Intro
- TOSEC
- MAME
- ScreenScraper
- SteamGridDB
- Metadata vs Storage

### Part III: Systems and Software

- Emulators
- Frontends
- BIOS
- File Formats

### Part IV: Linux Gaming Stack

- Linux
- Docker
- Flatpak
- Steam
- Proton
- Sunshine
- Moonlight

### Part V: Enhancements

- HD Packs
- AI Upscaling
- Widescreen
- Mods

### Part VI: Museum

- Console History
- Digital Stores
- Lost Services
- Preservation Stories

## Final Editorial Rule

Nothing enters the DAP Retro Bible unless it is useful, maintainable and honest enough that we would still be comfortable recommending it years from now.
