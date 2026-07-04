# Docker for Retro Gaming

Docker has a useful place in a retro gaming stack, but it should not be treated as the answer to every problem.

It is excellent for services that run in the background: library managers, web interfaces, metadata tools, documentation sites and supporting databases. It is usually less suitable for normal desktop emulators, where direct access to controllers, audio, display sessions and graphics hardware matters.

This chapter explains where Docker fits in the DAP Retro Bible.

## The Simple Rule

Use Docker for services.

Use native packages, Flatpak or AppImage for most desktop emulators.

That rule is not absolute, but it is a good starting point. Containers are brilliant when an application behaves like a server. They become more awkward when the application behaves like a desktop program.

## Good Docker Candidates

Docker is a strong fit for:

- RomM;
- web dashboards;
- documentation sites;
- metadata helpers;
- library scanners;
- database-backed services;
- small helper tools that run on a schedule.

These tools benefit from repeatable configuration, persistent storage and easy redeployment.

## Poor Docker Candidates

Docker is usually not the simplest first choice for:

- Dolphin;
- PCSX2;
- RPCS3;
- PPSSPP;
- xemu;
- desktop frontends;
- anything requiring low-latency controller, display and audio behaviour.

These applications usually work better as normal desktop applications.

## Paths Are the Main Trap

Docker containers see their own paths.

The host may store the library under a normal Linux mount such as `/mnt/games/`, while a container may see the same content under a shorter internal path such as `/roms`.

Both paths can be correct. The important thing is to document which path belongs to which layer.

> **DAP Tip**
>
> When troubleshooting a Docker service, ask two questions: where is the file on the host, and where does the container see it?

## Read-Only Library Access

Most services do not need permission to change the source game library.

A library browser may need to read ROM folders, but it should not necessarily be able to rename or delete source files. A metadata service may need write access to its own database or cache, not to the main collection.

Good practice:

- keep source game folders protected where practical;
- keep service configuration separate;
- keep databases persistent;
- separate cache data from source files;
- avoid giving every service write access to everything.

## Persistent State

Containers can be recreated. Service data must survive.

Important persistent data includes:

- service configuration;
- databases;
- uploaded artwork;
- metadata cache;
- user accounts;
- custom settings;
- scan results.

A service that took hours to scan and organise a library should not lose its memory because a container was rebuilt.

## Permissions

Permission problems are common.

They may appear as:

- empty libraries;
- failed scans;
- artwork not saving;
- database errors;
- files created with unexpected ownership.

The fix is not always to loosen permissions everywhere. The better fix is to understand which user the service runs as, which folders it needs to read and which folders it needs to write.

## Networking

Docker services may be reached in several ways:

- local host and port;
- an internal container name;
- a reverse proxy;
- a friendly domain name.

Each access route adds its own possible failure points. Document the intended route so future troubleshooting starts in the right place.

## Backups

A Docker-based retro service needs backups just like any other important service.

Back up:

- compose files or deployment notes;
- environment files;
- configuration folders;
- databases;
- artwork and metadata;
- custom scripts;
- notes about mounted paths.

The container image is usually replaceable. The service state is the valuable part.

## Real-World DAP Setup

A DAP-style Docker setup should keep three things separate:

| Layer | Purpose |
| --- | --- |
| Source library | ROMs, disc images, BIOS notes and structured storage. |
| Service config | Settings for the containerised application. |
| Service state | Databases, metadata, cache and generated files. |

The service should point at the source library without taking ownership of it.

## Common Mistakes

Common Docker mistakes include:

- confusing host paths and container paths;
- storing important state inside disposable containers;
- giving services unnecessary write access;
- forgetting permissions;
- failing to back up databases;
- assuming Docker fixes a poor storage layout;
- trying to containerise every desktop emulator.

## Troubleshooting

### The service cannot see the library

Check:

- the host folder exists;
- the container has the folder mounted;
- the application is configured to use the container path;
- permissions allow the service to read the files.

### Metadata is not saved

Check:

- the configuration path is persistent;
- the database path is writable;
- the service has permission to write;
- the storage location is not temporary.

### The service works locally but not through the browser

Check:

- the service is running;
- the intended port or proxy route is correct;
- the application expects a base path;
- authentication or headers are not blocking access.

## Key Points

- Docker is best for services, not most desktop emulators.
- Path mapping must be documented clearly.
- Source libraries should not be casually writable by every service.
- Databases and metadata must be persistent.
- Permission problems often look like application bugs.
- Backups should include service state, not only deployment files.

## What Comes Next

Next comes Flatpak vs Native Packages: how installation choices affect emulator paths, permissions, updates and long-term maintenance.