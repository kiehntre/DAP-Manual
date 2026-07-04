# Compression Reference

Compression should make a library easier to store, not harder to understand.

This chapter is the skeleton for a quick reference covering CHD, RVZ, WUA, CSO, ZIP, 7z and related formats.

## Format Table Skeleton

| Format | Best use | Watch for |
| --- | --- | --- |
| ZIP | Arcade sets, small ROMs | Do not extract MAME sets blindly. |
| 7z | Archival compression | Frontend support varies. |
| CHD | Many disc systems | Confirm emulator support. |
| RVZ | GameCube and Wii | Dolphin-focused. |
| WUA | Wii U | Cemu-focused. |
| CSO | PSP | Test load performance. |

## Practical Advice

- Keep verified sources before conversion.
- Test one game before bulk converting.
- Record conversion commands.
- Keep multi-disc structures clear.

> **DAP Tip**
>
> Compression is a policy decision. Write the policy down before the library grows around it.

## Common Mistakes

- Compressing everything the same way.
- Deleting originals too early.
- Using unsupported formats in frontends.
- Breaking cue/bin relationships.

## What Comes Next

Next comes controller reference, where input rules can be gathered for quick setup.
