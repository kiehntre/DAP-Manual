# The DAP Retro Bible Master Plan

This is the working production plan for **The DAP Retro Bible**.

The current phase is the **Skeleton Pass**. The goal is to make the full shape of the book visible before expansion, fact-checking, illustration, cross-linking and professional editing.

## Status Key

| Status | Meaning |
| --- | --- |
| Existing draft | Chapter has prose beyond a skeleton and needs expansion/editing. |
| Skeleton draft | Chapter exists as a structural draft and needs full expansion. |
| Planned | Chapter is planned but not yet created. |
| Needs split | Chapter exists but may later become multiple chapters. |

## Priority Key

| Priority | Meaning |
| --- | --- |
| P0 | Core book structure or high-risk factual foundation. |
| P1 | Major chapter required for the first complete edition. |
| P2 | Important supporting material. |
| P3 | Useful reference, appendix or later expansion candidate. |

## Review Status Key

| Review status | Meaning |
| --- | --- |
| Not reviewed | No formal review yet. |
| Needs fact-check | Version-sensitive, legal, historical or technical claims need checking. |
| Needs expansion | Structure exists but prose is thin. |
| Needs screenshots | Visual capture plan required. |
| Needs diagrams | Diagram plan required. |
| Ready for edit | Factually stable enough for prose editing. |

## Chapter Plan

| No. | Chapter | File | Status | Priority | Word target | Dependencies | Notes | Screenshot requirements | Tables required | Diagrams required | Review status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 00 | Preface | `chapters/00-preface.md` | Existing draft | P0 | 1,500 | None | Establishes the promise of the book. | None | None | Book structure overview optional. | Needs expansion |
| 01A | Foundations and Philosophy | `chapters/01-foundations-and-philosophy.md` | Existing draft | P0 | 3,000 | Preface | Duplicate numbering with preservation chapter needs editorial decision. | Folder examples optional. | Layer table | DAP stack diagram | Needs fact-check |
| 01B | Why Preservation Matters | `chapters/01-why-preservation-matters.md` | Existing draft | P0 | 3,000 | Preface | Should become the emotional and historical foundation. | Store closure examples if cited. | Preservation risk table | Preservation layers diagram | Needs fact-check |
| 02 | Hall of Fame | `chapters/02-hall-of-fame.md` | Existing draft | P1 | 4,000 | Preservation toolkit | Needs citations and careful attribution. | Project screenshots optional. | Entry summary table | None | Needs fact-check |
| 03 | Hall of Shame | `chapters/03-hall-of-shame.md` | Existing draft | P1 | 4,000 | Legal landscape | Must stay factual and evidence-based. | Avoid unless sourced. | Incident summary table | Timeline optional | Needs fact-check |
| 04 | Preservation Toolkit | `chapters/04-preservation-toolkit.md` | Existing draft | P0 | 4,500 | Foundations | Core terminology chapter. | Tool screenshots optional. | Tool category table | Preservation workflow diagram | Needs fact-check |
| 05 | Metadata vs Storage | `chapters/05-metadata-vs-storage.md` | Existing draft | P0 | 3,500 | Storage architecture | Key source-of-truth chapter. | Frontend metadata examples. | Metadata vs storage table | Library layer diagram | Needs expansion |
| 06 | Archive Formats and Compression | `chapters/06-archive-formats-and-compression.md` | Existing draft | P0 | 5,000 | Verification, storage | Needs format support checked. | File browser examples optional. | Format comparison table | Conversion flow diagram | Needs fact-check |
| 07 | BIOS and Firmware | `chapters/07-bios-and-firmware.md` | Existing draft | P0 | 4,000 | Legal landscape | Must avoid copyrighted material. | Emulator BIOS settings only. | BIOS concept table | BIOS path diagram | Needs fact-check |
| 08 | Choosing an Emulator | `chapters/08-choosing-an-emulator.md` | Existing draft | P0 | 3,500 | System chapters | Should define recommendation philosophy. | Emulator UI examples optional. | Decision matrix | Emulator selection flow | Needs fact-check |
| 09 | Front Ends | `chapters/09-front-ends.md` | Existing draft | P0 | 3,500 | Metadata, storage | Broad frontend overview. | ES-DE, RomM, LaunchBox examples. | Frontend comparison table | Frontend/storage relationship | Needs expansion |
| 10 | Storage Architecture | `chapters/10-storage-architecture.md` | Existing draft | P0 | 4,500 | Foundations | Core Linux path chapter. | Folder tree screenshot optional. | Storage policy table | Folder layout diagram | Needs expansion |
| 11 | Linux Foundations | `chapters/11-linux-foundations.md` | Existing draft | P0 | 4,000 | None | Needs distro-neutral clarity. | Terminal examples optional. | Package type table | Linux stack diagram | Needs expansion |
| 12 | Docker for Retro Gaming | `chapters/12-docker-for-retro-gaming.md` | Existing draft | P1 | 3,500 | Linux foundations | Connect to RomM and services. | Docker compose examples. | Container role table | Bind mount diagram | Needs fact-check |
| 13 | Flatpak vs Native Packages | `chapters/13-flatpak-vs-native-packages.md` | Existing draft | P1 | 3,000 | Linux foundations | Needs path examples verified. | Flatseal screenshot. | Packaging comparison table | Sandbox access diagram | Needs fact-check |
| 14 | Steam, Proton and Non-Steam Games | `chapters/14-steam-proton-and-non-steam-games.md` | Existing draft | P1 | 4,000 | Wine, Steam Input | Needs current Proton guidance. | Steam shortcut examples. | Proton version table | Launch path diagram | Needs fact-check |
| 15 | Sunshine and Moonlight | `chapters/15-sunshine-and-moonlight.md` | Existing draft | P1 | 4,000 | Remote gaming | Split with host/client chapters if needed. | Sunshine and Moonlight screens. | Streaming settings table | Host/client diagram | Needs fact-check |
| 16 | Emulator Configuration Philosophy | `chapters/16-emulator-configuration-philosophy.md` | Existing draft | P0 | 3,000 | Choosing emulator | Defines per-system setup style. | Emulator settings examples. | Config policy table | Settings inheritance diagram | Needs expansion |
| 17 | Controller Configuration | `chapters/17-controller-configuration.md` | Existing draft | P0 | 3,500 | Controller reference | Needs hotkey policy. | Controller mapping screens. | Controller mapping table | Input layer diagram | Needs expansion |
| 18 | Shaders, Filters and CRT Simulation | `chapters/18-shaders-filters-and-crt-simulation.md` | Existing draft | P1 | 3,500 | System chapters | Needs visual examples. | Shader comparison shots. | Shader type table | Display pipeline diagram | Needs screenshots |
| 19 | Save Files, Save States and Backups | `chapters/19-save-files-save-states-and-backups.md` | Existing draft | P0 | 3,500 | Storage architecture | Core user-data protection chapter. | Save path examples optional. | Save type table | Backup flow diagram | Needs expansion |
| 20 | BIOS Organisation and Verification | `chapters/20-bios-organisation-and-verification.md` | Skeleton draft | P0 | 3,500 | BIOS and firmware | Expand with checksum workflow. | BIOS settings examples only. | BIOS audit table | BIOS verification flow | Needs fact-check |
| 21 | ROM Dumps and Verification | `chapters/21-rom-dumps-and-verification.md` | Skeleton draft | P0 | 4,000 | Preservation toolkit | Core verification workflow. | DAT tool screenshots. | Verification result table | Dump verification flow | Needs fact-check |
| 22 | Disc Images, CHD, RVZ and WUA | `chapters/22-disc-images-chd-rvz-and-wua.md` | Skeleton draft | P1 | 4,000 | Archive formats | Needs tested commands later. | Conversion examples optional. | Disc format table | Conversion decision tree | Needs fact-check |
| 23 | ROM Managers and DAT Files | `chapters/23-rom-managers-and-dat-files.md` | Skeleton draft | P1 | 3,500 | Verification | Needs tool selection. | ROM manager screenshots. | DAT workflow table | Audit/rebuild flow | Needs fact-check |
| 24 | RomM Library Management | `chapters/24-romm-library-management.md` | Skeleton draft | P1 | 3,500 | Docker, metadata | Needs DAP Docker notes. | RomM library views. | RomM role table | Container/storage diagram | Needs screenshots |
| 25 | ES-DE in Practice | `chapters/25-es-de-in-practice.md` | Skeleton draft | P1 | 4,000 | Frontends | Needs real config paths. | ES-DE system and game views. | ES-DE config table | Frontend launch flow | Needs screenshots |
| 26 | LaunchBox and Big Box on Linux | `chapters/26-launchbox-and-big-box-on-linux.md` | Skeleton draft | P2 | 3,000 | Wine | Linux use may need careful testing. | LaunchBox under Wine if used. | Path mapping table | Wine path diagram | Needs fact-check |
| 27 | Pegasus and Alternative Frontends | `chapters/27-pegasus-and-alternative-frontends.md` | Skeleton draft | P2 | 3,000 | Frontends | May remain comparison chapter. | Pegasus examples optional. | Frontend comparison table | None | Needs expansion |
| 28 | Controller Profiles and Input Mapping | `chapters/28-controller-profiles-and-input-mapping.md` | Skeleton draft | P0 | 3,500 | Controller config | Should align with controller appendix. | Input settings screens. | Mapping table | Input stack diagram | Needs expansion |
| 29 | Steam Input and Shortcuts | `chapters/29-steam-input-and-shortcuts.md` | Skeleton draft | P1 | 3,000 | Steam | Needs Steam UI screenshots. | Steam Input and shortcut screens. | Shortcut field table | Steam launch flow | Needs screenshots |
| 30 | Sunshine Host Configuration | `chapters/30-sunshine-host-configuration.md` | Skeleton draft | P1 | 3,500 | Sunshine | Host-focused expansion. | Sunshine app settings. | Encoder/settings table | Host pipeline diagram | Needs fact-check |
| 31 | Moonlight Clients and Handhelds | `chapters/31-moonlight-clients-and-handhelds.md` | Skeleton draft | P1 | 3,500 | Sunshine host | Client-focused expansion. | Moonlight client settings. | Client profile table | Network/client diagram | Needs screenshots |
| 32 | Network Storage and Permissions | `chapters/32-network-storage-and-permissions.md` | Skeleton draft | P0 | 3,500 | Storage, Docker | Needs Linux permission examples. | Terminal examples optional. | UID/GID table | Permission flow diagram | Needs fact-check |
| 33 | Docker Services for Retro Libraries | `chapters/33-docker-services-for-retro-libraries.md` | Skeleton draft | P1 | 3,500 | Docker | Expand with compose examples. | Portainer or compose examples optional. | Service table | Container topology | Needs fact-check |
| 34 | HD Texture Packs | `chapters/34-hd-texture-packs.md` | Skeleton draft | P2 | 3,000 | Enhancements | Needs examples by system. | Before/after texture shots. | Texture pack table | Asset load diagram | Needs screenshots |
| 35 | Widescreen Hacks and Patches | `chapters/35-widescreen-hacks-and-patches.md` | Skeleton draft | P2 | 3,000 | Shaders, patches | Needs visual comparison. | 4:3 vs widescreen examples. | Patch compatibility table | Aspect workflow | Needs screenshots |
| 36 | Translations, ROM Hacks and Mods | `chapters/36-translation-romhacks-and-mods.md` | Skeleton draft | P2 | 3,500 | Verification | Needs patch workflow examples. | Patch tool screenshots optional. | Patch format table | Patch workflow diagram | Needs fact-check |
| 37 | Arcade and MAME Sets | `chapters/37-arcade-and-mame-sets.md` | Skeleton draft | P1 | 4,000 | Arcade systems | Needs MAME version detail. | MAME UI/log examples. | Parent/clone table | Set relationship diagram | Needs fact-check |
| 38 | Computer Systems, Keyboards and Media | `chapters/38-computer-systems-keyboards-and-media.md` | Skeleton draft | P1 | 3,500 | Computer chapters | Broad bridge chapter. | Keyboard mapping examples. | Media type table | Computer launch flow | Needs expansion |
| 39 | Digital Stores, Lost Services and Delisting | `chapters/39-digital-stores-lost-services-and-delisting.md` | Skeleton draft | P1 | 4,000 | Legal landscape | Needs citations. | Archived store pages where allowed. | Store lifecycle table | Service dependency diagram | Needs fact-check |
| 40 | Maintenance, Audits and the Long Game | `chapters/40-maintenance-audits-and-the-long-game.md` | Skeleton draft | P0 | 3,500 | Backups, verification | Should become operational close of setup parts. | Audit report examples. | Maintenance cadence table | Audit cycle diagram | Needs expansion |
| 41 | Ubuntu for Retro Gaming | `chapters/41-ubuntu-for-retro-gaming.md` | Skeleton draft | P1 | 2,500 | Linux foundations | Distro-specific chapter. | Ubuntu software/settings optional. | Package source table | Install source diagram | Needs fact-check |
| 42 | Fedora, Nobara and Modern Linux Gaming | `chapters/42-fedora-nobara-and-modern-linux-gaming.md` | Skeleton draft | P1 | 2,500 | Linux foundations | Needs current Nobara wording checked. | Desktop/package examples optional. | Distro comparison table | None | Needs fact-check |
| 43 | Arch and Rolling Release Setups | `chapters/43-arch-and-rolling-release-setups.md` | Skeleton draft | P2 | 2,500 | Linux foundations | Keep pragmatic, not tribal. | Pacman/log examples optional. | Rolling maintenance table | Update workflow | Needs fact-check |
| 44 | AppImage and Portable Builds | `chapters/44-appimage-and-portable-builds.md` | Skeleton draft | P2 | 2,500 | Packaging | Needs examples of stable launcher paths. | File manager example optional. | Packaging comparison table | Portable app path diagram | Needs expansion |
| 45 | Wine Outside Steam | `chapters/45-wine-outside-steam.md` | Skeleton draft | P1 | 3,000 | Linux, Windows preservation | Needs Wine prefix examples. | Wine/Bottles screenshots optional. | Prefix policy table | Prefix layout diagram | Needs fact-check |
| 46 | Remote Gaming Design | `chapters/46-remote-gaming-design.md` | Skeleton draft | P1 | 3,000 | Sunshine, Moonlight | Broad streaming design chapter. | Client examples optional. | Client profile table | Whole-home streaming diagram | Needs expansion |
| 47 | GPU Passthrough and Virtual Machines | `chapters/47-gpu-passthrough-and-virtual-machines.md` | Skeleton draft | P2 | 3,000 | Linux foundations | Advanced topic; avoid overpromising. | VM settings optional. | Hardware requirement table | Passthrough architecture | Needs fact-check |
| 48 | Homelabs, NAS and Storage Servers | `chapters/48-homelabs-nas-and-storage-servers.md` | Skeleton draft | P1 | 3,500 | Storage, permissions | Connects DAP homelab identity. | NAS/share examples optional. | Storage role table | NAS/library topology | Needs expansion |
| 49 | Arcade Systems | `chapters/49-arcade-systems.md` | Skeleton draft | P1 | 3,500 | MAME sets | Platform-family intro. | Arcade frontend examples. | Arcade emulator table | Cabinet/control diagram | Needs expansion |
| 50 | Nintendo Entertainment System | `chapters/50-nintendo-entertainment-system.md` | Skeleton draft | P1 | 3,000 | Emulator choice | First Nintendo system chapter. | Emulator setting screenshot optional. | Emulator/file table | Cartridge workflow | Needs fact-check |
| 51 | Super Nintendo Entertainment System | `chapters/51-super-nintendo-entertainment-system.md` | Skeleton draft | P1 | 3,000 | NES, patching | Needs enhancement chip detail. | Shader examples optional. | Chip/support table | Patch workflow optional | Needs fact-check |
| 52 | Nintendo 64 | `chapters/52-nintendo-64.md` | Skeleton draft | P1 | 3,500 | Controller config | Needs emulator landscape checked. | Controller and video settings. | Emulator comparison table | Input mapping diagram | Needs fact-check |
| 53 | Nintendo GameCube | `chapters/53-nintendo-gamecube.md` | Skeleton draft | P1 | 3,500 | Dolphin, RVZ | Dolphin reference chapter. | Dolphin settings. | Format/settings table | RVZ workflow diagram | Needs screenshots |
| 54 | Nintendo Wii | `chapters/54-nintendo-wii.md` | Skeleton draft | P1 | 4,000 | Dolphin, motion controls | Needs motion input detail. | Dolphin Wii settings. | Controller profile table | Motion control diagram | Needs fact-check |
| 55 | Nintendo Wii U | `chapters/55-nintendo-wii-u.md` | Skeleton draft | P1 | 4,000 | WUA, legal landscape | Needs Cemu current status. | Cemu settings. | Content grouping table | Base/update/DLC diagram | Needs fact-check |
| 56 | Nintendo Switch | `chapters/56-nintendo-switch.md` | Skeleton draft | P1 | 4,000 | Legal landscape | Highest legal/current-platform caution. | Avoid screenshots until reviewed. | Content/version table | Legal workflow diagram optional | Needs fact-check |
| 57 | Sega Master System | `chapters/57-sega-master-system.md` | Skeleton draft | P2 | 2,500 | Sega overview | Needs regional history. | Optional emulator shot. | Region table | None | Needs expansion |
| 58 | Sega Mega Drive and Genesis | `chapters/58-sega-mega-drive-and-genesis.md` | Skeleton draft | P1 | 3,000 | Master System | Needs naming policy. | Emulator settings optional. | Add-on table | Sega family diagram | Needs fact-check |
| 59 | Sega 32X | `chapters/59-sega-32x.md` | Skeleton draft | P2 | 2,500 | Mega Drive | Needs BIOS and emulator checks. | Optional. | BIOS/emulator table | Add-on stack diagram | Needs fact-check |
| 60 | Sega Saturn | `chapters/60-sega-saturn.md` | Skeleton draft | P1 | 4,000 | BIOS, CHD | Needs detailed BIOS and format notes. | Emulator settings. | BIOS/format table | Disc/BIOS flow | Needs fact-check |
| 61 | Sega Dreamcast | `chapters/61-sega-dreamcast.md` | Skeleton draft | P1 | 4,000 | GDI, CHD | Needs online service notes. | Flycast settings. | GDI/CDI/CHD table | VMU/save diagram | Needs fact-check |
| 62 | Sega Game Gear | `chapters/62-sega-game-gear.md` | Skeleton draft | P2 | 2,500 | Master System | Handheld display focus. | LCD shader examples. | Emulator table | None | Needs expansion |
| 63 | Sony PlayStation | `chapters/63-sony-playstation.md` | Skeleton draft | P1 | 4,000 | BIOS, CHD | Needs multi-disc workflow. | DuckStation settings. | BIOS/format table | M3U disc flow | Needs fact-check |
| 64 | Sony PlayStation 2 | `chapters/64-sony-playstation-2.md` | Skeleton draft | P1 | 4,500 | BIOS, PCSX2 | Major platform chapter. | PCSX2 settings. | BIOS/format/settings table | Memory card diagram | Needs fact-check |
| 65 | Sony PlayStation 3 | `chapters/65-sony-playstation-3.md` | Skeleton draft | P1 | 4,500 | RPCS3, firmware | Needs careful current guidance. | RPCS3 settings. | Firmware/content table | PS3 content model | Needs fact-check |
| 66 | Sony PSP | `chapters/66-sony-psp.md` | Skeleton draft | P1 | 3,000 | PPSSPP | Needs CSO/ISO guidance. | PPSSPP settings. | Format/enhancement table | None | Needs expansion |
| 67 | Sony PS Vita | `chapters/67-sony-ps-vita.md` | Skeleton draft | P2 | 3,500 | Legal landscape | Needs current emulator facts. | Avoid until reviewed. | Content grouping table | None | Needs fact-check |
| 68 | Microsoft Xbox | `chapters/68-microsoft-xbox.md` | Skeleton draft | P1 | 3,500 | Legal, XISO | Needs system file caution. | xemu settings. | XISO/system table | Xbox content diagram | Needs fact-check |
| 69 | Microsoft Xbox 360 | `chapters/69-microsoft-xbox-360.md` | Skeleton draft | P2 | 3,500 | Legal, Xenia | Needs Linux compatibility check. | Emulator screenshots optional. | Content/update table | None | Needs fact-check |
| 70 | Nintendo Game Boy | `chapters/70-nintendo-game-boy.md` | Skeleton draft | P1 | 2,500 | Handheld shaders | Needs palette examples. | Palette/shader shots. | Emulator table | None | Needs screenshots |
| 71 | Nintendo Game Boy Color | `chapters/71-nintendo-game-boy-color.md` | Skeleton draft | P1 | 2,500 | Game Boy | Needs enhanced vs colour-only detail. | Colour mode examples. | Compatibility table | None | Needs expansion |
| 72 | Nintendo Game Boy Advance | `chapters/72-nintendo-game-boy-advance.md` | Skeleton draft | P1 | 3,000 | BIOS, mGBA | Needs colour correction guidance. | mGBA settings. | BIOS/save table | None | Needs fact-check |
| 73 | Nintendo DS | `chapters/73-nintendo-ds.md` | Skeleton draft | P1 | 3,500 | Touch input | Needs layout examples. | Screen layout screenshots. | Control/layout table | Dual-screen diagram | Needs screenshots |
| 74 | Nintendo 3DS | `chapters/74-nintendo-3ds.md` | Skeleton draft | P1 | 4,000 | Legal, emulator status | Needs current emulator landscape. | Avoid until reviewed. | Content/layout table | Content flow optional | Needs fact-check |
| 75 | Atari Systems | `chapters/75-atari-systems.md` | Needs split | P2 | 4,000 | Computer systems | May split into 2600, ST, Lynx, Jaguar later. | Emulator examples optional. | Atari family table | Atari timeline | Needs expansion |
| 76 | Commodore Systems | `chapters/76-commodore-systems.md` | Needs split | P2 | 4,000 | Computer systems | C64 may deserve standalone chapter. | VICE screenshots. | Media/ROM table | Load workflow | Needs fact-check |
| 77 | Amiga | `chapters/77-amiga.md` | Skeleton draft | P1 | 4,500 | Kickstart, computer media | Major computer chapter. | FS-UAE settings. | Kickstart/model table | Amiga profile diagram | Needs fact-check |
| 78 | Amstrad CPC | `chapters/78-amstrad-cpc.md` | Skeleton draft | P2 | 3,000 | Computer systems | Needs emulator selection. | Emulator examples optional. | Media/model table | None | Needs fact-check |
| 79 | ZX Spectrum | `chapters/79-zx-spectrum.md` | Skeleton draft | P2 | 3,000 | Computer systems | Needs tape/snapshot distinction. | Emulator examples optional. | Format/model table | Tape workflow | Needs expansion |
| 80 | MS-DOS Gaming | `chapters/80-ms-dos-gaming.md` | Skeleton draft | P1 | 4,500 | Windows preservation | Major PC preservation chapter. | DOSBox settings. | Sound card table | DOS launch diagram | Needs fact-check |
| 81 | Windows Gaming Preservation | `chapters/81-windows-gaming-preservation.md` | Skeleton draft | P1 | 4,500 | Wine, Proton | Needs DRM/legal caution. | Wine/Proton examples optional. | Dependency table | Prefix/install flow | Needs fact-check |
| 82 | ScummVM | `chapters/82-scummvm.md` | Skeleton draft | P1 | 3,000 | Adventure/PC chapters | Needs supported game workflow. | ScummVM launcher. | Data file table | Engine/data diagram | Needs screenshots |
| 83 | Other Important Systems | `chapters/83-other-important-systems.md` | Needs split | P2 | 5,000 | System chapters | Candidate incubator chapter. | Optional. | Candidate system table | System family map | Needs expansion |
| 84 | Internet Archive and Public Preservation | `chapters/84-internet-archive-and-public-preservation.md` | Skeleton draft | P1 | 3,500 | Legal landscape | Needs citations and careful wording. | Archived page examples. | Source type table | Public preservation flow | Needs fact-check |
| 85 | Preservation Projects and Community Efforts | `chapters/85-preservation-projects-and-community-efforts.md` | Skeleton draft | P1 | 3,500 | Toolkit | Needs project attribution. | Project pages optional. | Project category table | Community ecosystem diagram | Needs fact-check |
| 86 | Legal Landscape for Preservation | `chapters/86-legal-landscape-for-preservation.md` | Skeleton draft | P0 | 4,500 | None | Must be reviewed carefully; not legal advice. | None | Legal distinction table | Rights boundary diagram optional | Needs fact-check |
| 87 | FPGA and Hardware Preservation | `chapters/87-fpga-and-hardware-preservation.md` | Skeleton draft | P2 | 3,000 | Emulation future | Needs MiSTer-style examples. | FPGA UI examples optional. | FPGA vs emulation table | Hardware recreation diagram | Needs fact-check |
| 88 | Future of Emulation | `chapters/88-future-of-emulation.md` | Skeleton draft | P2 | 3,000 | All emulator chapters | Needs careful current framing. | None | Project risk table | Emulator lifecycle diagram | Needs expansion |
| 89 | AI Restoration and Upscaling | `chapters/89-ai-restoration-and-upscaling.md` | Skeleton draft | P2 | 3,000 | Texture packs | Needs restraint and examples. | Before/after examples. | AI use-case table | Asset pipeline diagram | Needs fact-check |
| 90 | Audio Restoration and Music Preservation | `chapters/90-audio-restoration-and-music-preservation.md` | Skeleton draft | P2 | 3,000 | DOS, PC, disc formats | Needs sound hardware examples. | Audio settings screenshots optional. | Audio format/device table | Audio path diagram | Needs expansion |
| 91 | Community Tools and Utilities | `chapters/91-community-tools-and-utilities.md` | Skeleton draft | P2 | 3,000 | Toolkit | Useful support chapter. | Tool examples optional. | Utility category table | Tool workflow diagram | Needs expansion |
| 92 | Glossary | `chapters/92-glossary.md` | Skeleton draft | P3 | 4,000 | All chapters | Should be updated continuously. | None | Term table | None | Needs expansion |
| 93 | Reference Tables | `chapters/93-reference-tables.md` | Skeleton draft | P3 | 3,000 | All chapters | Must be dated/versioned. | None | Many reference tables | None | Needs fact-check |
| 94 | BIOS Reference | `chapters/94-bios-reference.md` | Skeleton draft | P0 | 4,000 | BIOS chapters | High-value reference; high legal caution. | None or emulator settings only. | BIOS reference table | BIOS folder diagram | Needs fact-check |
| 95 | Compression Reference | `chapters/95-compression-reference.md` | Skeleton draft | P1 | 3,000 | Archive chapters | Quick lookup appendix. | None | Format reference table | Conversion decision tree | Needs fact-check |
| 96 | Controller Reference | `chapters/96-controller-reference.md` | Skeleton draft | P1 | 3,000 | Controller chapters | Needs final DAP hotkey policy. | Controller diagrams/screens. | Hotkey/reference table | Input layer diagram | Needs expansion |
| 97 | Recommended Folder Structures | `chapters/97-recommended-folder-structures.md` | Skeleton draft | P0 | 3,000 | Storage architecture | Core appendix. | Folder examples optional. | Folder policy table | Master folder diagram | Needs expansion |
| 98 | Recommended Hardware | `chapters/98-recommended-hardware.md` | Skeleton draft | P2 | 3,500 | System requirements | Recommendations age quickly. | Hardware photos optional. | Hardware tier table | Setup topology diagram | Needs fact-check |
| 99 | Recommended Linux Distributions | `chapters/99-recommended-linux-distributions.md` | Skeleton draft | P2 | 3,000 | Linux chapters | Needs dated advice. | None | Distro comparison table | None | Needs fact-check |
| 100 | Troubleshooting Reference | `chapters/100-troubleshooting-reference.md` | Skeleton draft | P1 | 4,000 | All practical chapters | Should link to deeper chapters. | Log examples optional. | Symptom/fix table | Troubleshooting flowchart | Needs expansion |
| 101 | Chapter Index and Editorial Roadmap | `chapters/101-chapter-index-and-editorial-roadmap.md` | Skeleton draft | P3 | 2,500 | Master plan | May merge with this file later. | None | Editorial status table | Production workflow | Needs expansion |

## Expansion Pass Targets

| Pass | Goal | Output |
| --- | --- | --- |
| Skeleton Pass | Ensure every planned chapter exists. | Complete chapter structure. |
| Expansion Pass 1 | Turn skeletons into readable draft chapters. | Full prose drafts. |
| Technical Pass | Verify commands, paths, emulator names and platform behaviour. | Corrected technical content. |
| Citation Pass | Add official, durable and archived sources. | Source-backed claims. |
| Visual Pass | Add screenshots, diagrams and captions. | Illustrated book draft. |
| Cross-Link Pass | Link related chapters and references. | Navigable web/book structure. |
| Human Edit | Warmth, rhythm, consistency and clarity. | Publication-quality prose. |
| Final Review | Legal, factual and editorial review. | Release candidate. |

## Global Screenshot Requirements

- Capture emulator settings only when they teach something.
- Avoid private paths, account names, tokens, keys and email addresses.
- Prefer cropped, readable screenshots over full desktop captures.
- Include alt text and captions during the visual pass.
- Re-capture screenshots after major UI changes.

## Global Table Requirements

- Every system chapter should eventually include emulator, BIOS, file format and folder layout tables where useful.
- Every reference chapter must include dated tables where recommendations can change.
- Tables must stay short enough to read on the web.

## Global Diagram Requirements

- DAP stack overview.
- Preservation workflow.
- Storage source-of-truth model.
- BIOS verification flow.
- Disc conversion decision tree.
- Frontend launch flow.
- Sunshine/Moonlight host-client flow.
- Backup and restore cycle.
- System content grouping for modern consoles.

## High-Risk Review Areas

- Legal landscape and rights language.
- BIOS, firmware, keys and account-bound content.
- Current and recent platforms.
- Emulator project status.
- Store closures and delisting history.
- Hardware recommendations.
- Version-sensitive compatibility claims.

## Current Editorial Notes

- There are currently two chapter `01` files. A later structural pass should resolve numbering.
- Chapters `75`, `76` and `83` are likely split candidates.
- Appendix-style chapters currently live under `chapters/` because the Skeleton Pass asked for one Markdown chapter file per topic. They may move to `appendices/` later.
- The book should remain Linux-first, practical, warm and historically aware.
