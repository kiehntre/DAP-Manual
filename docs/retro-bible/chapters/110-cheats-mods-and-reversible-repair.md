# Cheats, Mods and Repair Without Destruction

Cheats and mods are powerful because they change behavior. They are dangerous
when a tool quietly guesses semantics or writes over a configuration that was
never backed up.

## Cheat formats are not interchangeable

EmuWiz has implemented support for selected safe forms of RetroArch `.cht`,
PCSX2 PNACH, Dolphin Action Replay, Gecko/Ocarina, Dolphin OnFrame and
Nintendo DS Action Replay. A neutral representation can retain operations such
as writes and explicit execution policy, but it does not make every Action
Replay, GameShark or CodeBreaker variant equivalent.

OnFrame every-frame semantics must not be collapsed into an ordinary write.
Unsupported operations remain visible as unsupported. A malformed line may be
accepted with a warning when its meaning is unambiguous; ambiguous encrypted or
version-dependent codes should remain browse-only.

The safe flow is `recognise → parse → validate → warn → preview → explicit
apply`. Show source and provenance. For a mixed document, “3 exact, 1
unsupported” means “cannot convert safely”, not “write the three and pretend the
file is complete”. Cross-source reconciliation can report exact semantic
duplicates, raw duplicates and same-name conflicts, but it must never choose a
winner automatically.

## Mods are a compatibility problem

A patch may require an exact game revision, region, emulator, load order or
folder layout. A texture pack, widescreen patch, translation, cheat and total
conversion are different things even if a download site calls all of them
“mods”. Record the base hash, patch version, source and installation location.

Preview the files and keep a backup or transaction journal. If a mod site is
linked, do not imply that its files are redistributed or that its claims have
been independently verified. Broad automatic mod installation remains partial
or future in many environments; the safe guide is to diagnose first and apply
only an exact, reversible plan.

## Repair is a transaction

Repair should mean:

1. diagnose;
2. show the evidence and proposed change;
3. ask for explicit confirmation;
4. journal each operation;
5. apply atomically where possible;
6. offer rollback, resume or quarantine.

Quick Rename recovery should remain usable with dozens of historical entries:
show a bounded summary, keep technical details collapsed, and retain every
rollback action. A clean interface must not hide the recovery record.

Duplicates are a review state, not permission to delete. Exact duplicate hashes
can support a quarantine plan, but the source path, backup policy and user
confirmation still matter. A malformed archive may be worth preserving even if
no emulator can open it today.

## Rename decisions should explain themselves

An evidence-backed rename says why it was proposed: exact DAT match, strong
container evidence, human-confirmed launch, or merely a weak filename hint. Use
states such as **Verified**, **Strong**, **Human verified**, **Ambiguous**,
**Conflict** and **Unknown**. Never auto-rename an ambiguous item or silently
resolve a case-fold collision.

For broader safety principles, see [Translation ROM Hacks and Mods](36-translation-romhacks-and-mods.md), [Preservation Toolkit](04-preservation-toolkit.md) and [Renaming With Evidence](102-evidence-based-identification.md).

## Safe boundaries for future converters

A future converter should use parsers and encoders around a neutral semantic
model, not a web of pairwise text substitutions. The model must preserve width,
address, conditional behavior, button activators, master/init operations and
execution policy. If a target cannot represent one operation, the preview must
say so and retain the unsupported source record.

This is why “Action Replay” is not one universal format and why a GameShark
label is insufficient without a console generation and grammar. Conversion is a
candidate for exact, lossy-with-warning, or refused output; it is never a reason
to silently install a partial file.

## Recovery cards are part of the product

When a rename or cheat installation leaves a transaction interrupted, recovery
must remain legible on a small viewport. Show a bounded summary first, expose
the first few entries, and let the user expand the complete list. Keep technical
details collapsed but never delete the journal. A recovery UI that is technically
complete but pushes the next button below an enormous card stack is still a
failed user workflow.
