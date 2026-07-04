# FPGA and Hardware Preservation

FPGA systems sit in a fascinating space between original hardware and software emulation.

This chapter is the skeleton for explaining MiSTer-style workflows, hardware recreation and where FPGA belongs in a Linux-first retro book.

## Historical Context

Field-programmable gate arrays let hardware behaviour be described and recreated in configurable logic. In retro gaming, that opened a path for low-latency, hardware-like recreations of older systems.

## Practical Advice

- Treat FPGA as another preservation and play option, not a replacement for all emulation.
- Record core versions.
- Keep BIOS and ROM requirements documented.
- Understand display and controller output.
- Keep Linux storage and verification habits even when the player device is not Linux.

## Preservation Notes

FPGA cores can be excellent for timing-sensitive systems, but their accuracy still depends on implementation, documentation and testing.

> **DAP Tip**
>
> FPGA and software emulation can share the same verified library philosophy. The launch device changes; the care for files does not.

## Common Mistakes

- Assuming FPGA automatically means perfect.
- Forgetting core versions.
- Mixing unverified ROMs into a hardware setup.
- Ignoring display scaling and analogue output differences.

## Troubleshooting

### Game behaves differently from emulator

Check core version, ROM checksum, BIOS, region and display timing.

## DAP Warning

Do not use "hardware accurate" as a blanket claim. Accuracy needs evidence, no matter the method.

## What Comes Next

Next comes the future of emulation: development, maintenance, project risk and how this book should age.
