# Arcade and MAME Sets

Arcade emulation is its own world, and it rewards patience.

This draft chapter will explain why MAME sets behave differently from console folders and how to organise arcade games without losing your mind.

## Historical Context

- Explain arcade boards, ROM chips and shared hardware.
- Introduce MAME as documentation and preservation as well as emulation.
- Cover why version matching matters.

## Concepts

| Concept | Meaning | Draft note |
| --- | --- | --- |
| Parent | Main ROM set. | Clones may depend on it. |
| Clone | Regional or variant set. | May need parent data. |
| BIOS set | Shared system firmware. | Required by some arcade platforms. |
| Device ROM | Shared component data. | Easy to miss. |
| MAME version | Specific emulator release. | Should match the ROM set. |

## Practical Setup

- Choose a MAME version and set type.
- Keep arcade sets separate from console ROMs.
- Avoid manual renaming.
- Test with the intended MAME build.
- Document BIOS and device requirements.

## Recommended Layout

```text
/mnt/games/arcade/mame/
/mnt/games/arcade/fbneo/
/mnt/games/arcade/reports/
```

## Real-World DAP Setup

- Document chosen arcade emulator.
- Record frontend arcade system mapping.
- Note controller and coin/start mappings.

> **DAP Warning**
>
> Arcade files are not normal console ROM folders. Renaming ZIPs by hand is a fast route to broken games.

## Common Mistakes

- Mixing MAME versions.
- Deleting parents because only clones appear in the frontend.
- Extracting arcade ZIPs.
- Treating every non-working game as an emulator fault.

## Troubleshooting

### Game reports missing files

- Check MAME version.
- Check parent and clone relationship.
- Check BIOS and device ROMs.
- Audit against the matching DAT.

### Controls feel wrong

- Check arcade input mapping.
- Check per-game controls.
- Check cabinet type.
- Check frontend controller layer.

## Key Points

- MAME sets are versioned ecosystems.
- Parent, clone and BIOS relationships matter.
- Arcade deserves its own organisation policy.

## Further Reading

- Add MAME documentation.
- Add arcade set management references.
