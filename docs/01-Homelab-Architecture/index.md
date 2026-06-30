# Homelab Architecture

## Purpose

The DAP Manual documents the design, operation, maintenance and evolution of the homelab.

It is the single source of truth for infrastructure, AI, gaming, emulation, media services, networking, development and troubleshooting.

## Design Philosophy

- Diagnose before changing.
- Prefer reproducible configuration.
- Use Docker where it makes sense.
- Keep services documented.
- Use Git for anything important.
- Prefer read-only inspection before repair.
- Keep backups before major maintenance.

## Current High-Level Layout

```text
Internet
  |
Cloudflare
  |
OPNsense
  |
Proxmox VE
  |
  +-- VM101: OPNsense
  +-- VM201: Saltbox / Docker / AI / Docs
  +-- Other Linux and gaming workloads
```

## Core Components

| Area | Main Tools |
|---|---|
| Virtualisation | Proxmox VE |
| Firewall | OPNsense |
| Containers | Docker Compose |
| Media | Saltbox, Plex, Jellyfin, Arrs |
| AI | Open WebUI, Ollama, cloud APIs |
| Gaming | Steam, Proton, Wine |
| Streaming | Sunshine, Moonlight |
| Emulation | ES-DE, RetroDECK, RetroArch, Xemu, RPCS3, PCSX2 |
| Development | Rust, Python, Bash, Git |

## Documentation Rule

!!! tip
    Every time a real problem is solved, it should become either a chapter update, a troubleshooting playbook, or a script.
