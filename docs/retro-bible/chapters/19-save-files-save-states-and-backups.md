# Save Files, Save States and Backups

Saves are the part of a retro setup people only think about after something goes wrong.

ROMs can often be restored from a clean source. Emulator configs can be rebuilt. Artwork can be scraped again. But a 40-hour RPG save, a completed career mode or a carefully built memory card is personal history. Lose that, and the library suddenly feels less like a museum and more like a crime scene.

This chapter explains how to treat saves, save states and backups as first-class parts of the setup.

## Save Files vs Save States

Save files and save states are different things.

| Type | Meaning |
| --- | --- |
| Save file | The game's normal saved progress. |
| Memory card | A system-specific save container, common on disc consoles. |
| Save state | Emulator snapshot of the machine at a moment in time. |
| Config backup | Emulator, frontend and controller settings. |

Normal saves are usually safer long term. Save states are convenient, but often depend heavily on emulator version and exact game state.

## Normal Saves

Normal saves are created by the game itself.

They may appear as:

- SRAM files;
- EEPROM files;
- memory card images;
- system save folders;
- virtual memory units;
- emulator-specific save data.

These should be backed up before any major emulator migration.

## Save States

Save states are emulator snapshots.

They are useful for:

- difficult sections;
- testing;
- quick resume;
- practice;
- games without friendly save systems.

But they can be fragile. A save state may break after an emulator update, core change or game file change.

> **DAP Warning**
>
> Do not rely only on save states for long-term progress. Use in-game saves too. Save states are a ladder, not the floor.

## Memory Cards

Systems such as PlayStation, PlayStation 2, Dreamcast and GameCube may use memory card concepts.

A memory card can contain multiple saves. Losing it can affect several games at once.

Good practice:

- back up memory card files;
- name them clearly;
- avoid overwriting without a copy;
- document per-game or shared card policy;
- keep region quirks in mind.

## Where Saves Live

Save paths vary by emulator and install method.

They may live under:

```text
~/.config/
~/.local/share/
~/.var/app/
/mnt/games/saves/
```

Flatpak, AppImage and native builds may use different paths for the same emulator. Frontends may also launch a different build than the one configured manually.

## The Backup Rule

Back up anything you would be angry to lose.

That includes:

- normal saves;
- memory cards;
- save states;
- controller profiles;
- emulator configs;
- frontend metadata;
- shader presets;
- patch notes;
- Steam shortcuts where useful;
- RomM or other service databases.

A backup that only contains games is incomplete.

## Versioned Backups

A single backup is better than none. Versioned backups are better still.

Versioned backups help when:

- a save becomes corrupted;
- an emulator writes bad data;
- a sync tool deletes files;
- a user accidentally overwrites progress;
- a migration goes wrong.

If storage allows, keep several backup generations.

## Cloud Sync Caution

Cloud sync can help, but it can also spread mistakes quickly.

Risks include:

- corrupted save synced everywhere;
- deleted files removed from all devices;
- conflicts between machines;
- emulator path differences;
- save states copied across incompatible versions.

Use cloud sync carefully. It is not the same as a backup.

## Real-World DAP Setup

A DAP-style setup should aim for predictable save handling.

Useful structure:

```text
/mnt/games/saves/
/mnt/games/states/
/mnt/games/backups/
```

Not every emulator can be forced into this layout cleanly, but the goal should be to document where saves actually live.

For each major emulator, record:

- save path;
- state path;
- memory card path;
- config path;
- backup method;
- restore notes.

## Common Mistakes

Common mistakes include:

- backing up ROMs but not saves;
- relying only on save states;
- migrating emulators without copying memory cards;
- not knowing which build created the save;
- syncing saves without version history;
- deleting emulator config folders during cleanup;
- forgetting Docker service databases and frontend metadata.

## Troubleshooting

### Save is missing after changing emulator build

Check:

- old config path;
- new config path;
- Flatpak or native difference;
- frontend launch target;
- memory card folder;
- game region or filename change.

### Save state no longer loads

Check:

- emulator version;
- core version;
- game file checksum;
- BIOS change;
- save state slot;
- whether an in-game save still works.

### Saves vanish after streaming setup change

Check:

- which user launches the emulator;
- Steam shortcut target;
- Sunshine launch command;
- emulator profile;
- path permissions.

## Key Points

- Saves are personal data.
- Normal saves are usually safer than save states.
- Memory cards can contain progress for many games.
- Install method affects save paths.
- Cloud sync is not a proper backup by itself.
- Backups should include configs, metadata and controller profiles.
- Test restore, not only backup.

## What Comes Next

Next comes BIOS Organisation and Verification: turning the most mysterious folder in emulation into something sane, documented and recoverable.