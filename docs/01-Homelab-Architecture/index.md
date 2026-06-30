# Chapter 1 - Homelab Architecture

## Purpose

This handbook documents the design, operation, maintenance, and evolution of my homelab.

It serves as the single source of truth for infrastructure, AI, gaming, development, media services, networking, and troubleshooting. Every solution that is proven to work should be recorded here so it can be reproduced in the future.

---

# Design Philosophy

The homelab follows a few simple principles:

* Infrastructure should be reproducible.
* Automation is preferred over repetitive manual work.
* Every major change should be documented.
* Diagnostics should be performed before making changes.
* Security should never be sacrificed for convenience.
* Backups should always exist before major maintenance.

The aim is to spend less time remembering how something was configured and more time building new capabilities.

---

# Core Infrastructure

## Hypervisor

**Platform**

* Proxmox VE

Responsibilities:

* Virtual machine hosting
* Storage management
* GPU passthrough
* Backup management
* Network bridging

---

# Firewall

**Platform**

* OPNsense

Responsibilities

* Internet gateway
* Firewall
* VLAN routing
* DNS
* DHCP
* VPN
* Network security

---

# Primary Linux Server

**Platform**

Ubuntu Server

Responsibilities

* Docker
* Saltbox
* AI services
* Media services
* Documentation
* Development
* Game streaming support

This server is the heart of the homelab.

---

# Container Platform

Docker is used wherever practical.

Reasons:

* Easy deployment
* Easy upgrades
* Easy backups
* Isolation between services
* Consistent configuration

Compose files are stored in version control wherever possible.

---

# AI Platform

Primary components:

* Open WebUI
* Ollama
* Cloud AI providers
* Local models
* AI engineering workflows

The long-term goal is to build a personal AI systems engineer capable of assisting with administration, diagnostics, documentation, and software development.

---

# Gaming Platform

Linux is the preferred gaming platform.

Main technologies include:

* Steam
* Proton
* Proton-GE
* Wine
* Wine-GE
* Heroic Games Launcher
* Lutris
* Bottles

Performance and compatibility notes are documented throughout the handbook.

---

# Streaming Platform

Game streaming is built around:

* Sunshine
* Moonlight

Objectives:

* Low latency
* High image quality
* Reliable controller support
* HDR where supported
* Minimal maintenance

---

# Emulation Platform

Supported systems include:

* RetroArch
* ES-DE
* RetroDECK
* Xemu
* PCSX2
* RPCS3
* Dolphin
* DuckStation
* PPSSPP
* MAME
* ScummVM
* DOSBox

Each emulator has its own chapter covering installation, optimisation, controller configuration, BIOS management, and troubleshooting.

---

# Media Platform

The media platform is designed around Docker services and automated workflows.

Topics covered later include:

* Saltbox
* Plex
* Jellyfin
* Arr applications
* Download automation
* Metadata
* Library management
* Backup strategy

---

# Development Environment

Primary languages:

* Rust
* Python
* Bash

Development tools include Git, Docker, AI-assisted coding tools, and modern editors.

---

# Documentation Philosophy

Every solution included in this handbook should be:

* Tested
* Reproducible
* Clearly explained
* Easy to maintain

The goal is not simply to collect commands, but to understand why each solution works.

---

# Roadmap

Future chapters will expand this architecture into detailed implementation guides covering every major component of the homelab.

This document is intended to grow continuously as the homelab evolves, becoming the definitive operational reference for the entire environment.
