# ROM Managers and DAT Files

ROM managers are not glamorous, but they are how a library grows up.

This draft chapter will explain DAT files, auditing and rebuilding without making the reader feel like they joined a secret spreadsheet cult.

## Historical Context

- Explain why preservation projects publish database files.
- Cover the old habit of naming sets by release group or tool.
- Explain why structured auditing beats manual guessing.

## Concepts

| Term | Meaning | Draft note |
| --- | --- | --- |
| DAT | Machine-readable file list and checksums. | Used for auditing. |
| Audit | Compare files to a reference. | Finds missing, extra or wrong files. |
| Rebuild | Create a set from available files. | Can rename or reorganise files. |
| Split set | Parent and clone structure. | Common in arcade collections. |
| Merged set | Parent and clone data bundled. | Useful in some setups. |

## Practical Setup

- Choose the correct DAT for the target set.
- Work on a copy first.
- Audit before rebuilding.
- Save reports.
- Keep manual hacks and translations out of the clean audit folder.

> **DAP Warning**
>
> ROM managers can rename, move or delete a lot of files quickly. Never point a new rebuild job at your only copy.

## Recommended Layout

```text
/mnt/games/audit/incoming/
/mnt/games/audit/rebuilt/
/mnt/games/audit/reports/
/mnt/games/dat/
```

## Real-World DAP Setup

- Decide where audit work happens.
- Record which tools were used.
- Record which folders are considered canonical.

## Common Mistakes

- Using a DAT for the wrong region or version.
- Rebuilding arcade sets without understanding parents and clones.
- Mixing scraped metadata names with verified filenames.
- Treating a rebuild report as boring noise.

## Troubleshooting

### Files are renamed unexpectedly

- Check DAT naming rules.
- Check tool profile options.
- Check whether the job was set to rebuild or fix.

### Many arcade games fail after rebuild

- Check MAME version.
- Check merged, split or non-merged setting.
- Check BIOS and device ROM requirements.

## Key Points

- DAT files give the library a reference point.
- Audit work should happen on copies.
- Arcade sets need special care.

## Further Reading

- Add DAT format references.
- Add ROM manager documentation.
- Add MAME set documentation.
