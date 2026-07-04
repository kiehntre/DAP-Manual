# GPU Passthrough and Virtual Machines

GPU passthrough is the point where a retro setup starts borrowing ideas from serious homelab work.

This chapter is a skeleton for when virtual machines are useful, what they cost and why they should not be the first answer to every compatibility problem.

## Historical Context

Virtual machines began as a way to share hardware and isolate operating systems. For gaming, PCI passthrough made it possible for a Windows VM to use a real GPU directly. That opened doors for awkward Windows games, launchers and tools.

## Practical Advice

- Use passthrough only when Wine, Proton or native Linux are not enough.
- Plan IOMMU groups before buying hardware.
- Keep host and guest storage boundaries clear.
- Back up VM definitions and virtual disks.
- Treat GPU driver updates carefully on both host and guest.

## Linux-First Recommendations

Passthrough belongs to advanced setups. For most emulators, native Linux is easier. Use a VM when the target software truly needs Windows or isolation.

## DAP Gold Standard Setup

```text
/mnt/games/vms/
/mnt/games/vms/windows-gaming/
/mnt/games/backups/vm-definitions/
```

> **DAP Warning**
>
> GPU passthrough can eat weekends. Do not build it just to avoid learning a normal emulator's Linux settings.

## Common Mistakes

- Buying hardware before checking IOMMU layout.
- Passing through shared storage without permission planning.
- Forgetting VM backups.
- Assuming anti-cheat or DRM will accept a VM.

## Troubleshooting

### VM boots but GPU is not available

Check IOMMU, vfio binding, guest drivers, host display ownership and motherboard settings.

### Performance is poor

Check CPU pinning, storage path, hugepages, GPU driver and whether the workload actually benefits from passthrough.

## What Comes Next

Next comes homelabs and NAS storage: the quieter infrastructure that keeps a large retro library available and backed up.
