# Emulator Readiness, BIOSes and Safe Launch Planning

“The emulator is installed” is not the same as “this selected game is ready to
launch.” A reliable launch path resolves identity, checks a compatible profile,
checks firmware, builds an argument vector, and only then executes.

## Readiness is game-specific

RetroArch, PCSX2, DuckStation, Dolphin, PPSSPP, RPCS3, Amiberry and FS-UAE all
have different requirements. A discovered Dolphin binary does not make a PS2
game ready. A SameBoy profile should not be offered for a Game Boy Advance file.
The selected platform and content shape must be part of the candidate.

Typical states are more useful than a generic red cross:

- **Ready** — executable/profile, platform, identity and firmware checks pass;
- **Missing executable** — discovery found no safe binary;
- **Missing BIOS/Kickstart** — required firmware is absent or unverified;
- **Needs profile** — more than one binding is possible or the profile changed;
- **Unsupported content** — the adapter refuses this shape;
- **Stale selection** — the prepared plan belongs to another selected game.

Firmware marked “not required” is a successful result, not an unresolved one.
Unknown firmware must not be silently promoted to Ready.

## Typed commands, not shell strings

The safe launch sequence is:

`identity → readiness → profile → command preview → explicit execute`

The command is an executable plus typed arguments and an optional working
directory. Do not concatenate a shell string. This prevents quoting surprises,
path injection and accidental argument reinterpretation. It also makes the
preview honest: the argv that the user reviews is the argv the executor receives.

Recheck the selected file immediately before spawn. If the user selected Game A,
then clicked Game B while an asynchronous discovery result returned, the old
plan must be rejected rather than launched against B’s page.

## BIOS discipline

A BIOS file has an identity, revision and expected location. “BIOS found” is
only the beginning. Verify a hash where a trusted catalogue exists, record
region/revision, and keep the source and provenance. Do not download a random
“BIOS pack” and place every file in every emulator directory.

The same discipline applies to Amiga Kickstarts, Saturn firmware, PlayStation
firmware, PS2 BIOS files and arcade system ROMs. If several revisions are valid,
show which one was selected and why. If no safe selection exists, remain blocked.

## WHDLoad in practice

WHDLoad packages are not ordinary ROMs. An archive may contain a `.slave`, data
files, documentation and several slaves for different entry points. The slave
binds the package to a game identity and launch contract; the archive filename
may be meaningless.

EmuWiz’s Amiberry and FS-UAE WHDLoad path is **implemented** for verified
targets: it reuses the existing parser, profile selection, Kickstart readiness,
argv planner and shared process executor. The GUI should pass verified identity
and selected slave data through that path, not reparse filenames. ADF/HDF media
must not accidentally route through WHDLoad.

Extraction is often unnecessary when the emulator can launch package-relative
content. If extraction is required for a particular tool, make it a staging
operation with a clear cleanup policy. Never treat “archive opened” as “game
ready”.

## What needs physical QA

Static checks can prove a profile, executable, argv and firmware binding. They
cannot prove that a particular WHDLoad package behaves correctly on real display,
audio and controller paths. Use a disposable test set, capture launch errors,
and record whether the user reached a working game. That launch evidence is
valuable, but it should remain separate from DAT truth.

Read [BIOS Organisation and Verification](20-bios-organisation-and-verification.md), [Choosing an Emulator](08-choosing-an-emulator.md), and [Amiga](77-amiga.md) alongside this chapter.

## Doctor versus Launch

Diagnostics and launch readiness should agree without being the same subsystem.
Doctor can say that a Dolphin executable was found and a profile is valid;
Launch still has to decide whether the selected game is GameCube/Wii content
and whether its particular firmware or title path is usable. Conversely, an
emulator may be perfectly installed while a selected disk format is unsupported.

Keep the blocking reason. “Not ready” is less actionable than “PS2 BIOS missing”
or “selected WHDLoad slave is no longer present”. This also protects against
stale GUI state: when setup changes, recompute the candidate rather than leaving
an old Ready badge visible.

## Server-side and streamed launches

On a Sunshine host, add display, GPU, controller and audio checks to the smoke
plan. A successful process spawn is only one milestone. If Moonlight sees a
black screen, inspect the host display session and renderer before replacing a
known-good game file. Keep the launch plan and streaming diagnosis separate so
that a network symptom does not trigger a destructive library change.

## A launch failure decision tree

If no candidate appears, check platform and content shape first. If a candidate
appears but is not Ready, read the specific blocker: executable, profile,
firmware, identity or stale path. If the command preview is correct but the
process exits immediately, preserve the exit result and inspect emulator logs;
do not rewrite the game. If the emulator opens but cannot find content, compare
the exact argv path and working directory with the reviewed plan. If only remote
play fails, test locally on the host before changing any library state.

A good failure report answers three questions: what was selected, what was
planned, and which prerequisite failed. That makes support reproducible and
keeps “launch failed” from becoming a vague invitation to guess.
