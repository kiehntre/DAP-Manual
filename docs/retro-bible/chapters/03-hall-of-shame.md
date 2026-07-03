# Hall of Shame

The Hall of Shame records failures that damaged access, preservation, ownership, documentation or trust.

It is not here for cheap outrage. It is here because preservation improves when people remember what went wrong. Lost services, abandoned stores, broken account systems, vanishing patches and hostile policies all teach lessons.

A Hall of Shame entry must be factual, careful and useful. The target is not personal abuse. The target is bad practice.

## Selection Criteria

A subject belongs in the Hall of Shame when it shows one or more of these problems:

- games becoming inaccessible after a service closure;
- patches, DLC or updates becoming difficult to obtain;
- online-only design breaking long-term access;
- poor documentation of ownership or entitlement;
- destructive DRM or account dependency;
- abandoned storefronts;
- hardware or software decisions that make preservation unnecessarily difficult;
- companies failing to provide migration paths for customers.

The point is not to sneer. The point is to learn.

## Digital Store Closures

**Category:** Storefront and access failure.

**What happened:** Digital stores can close, reduce functionality or remove purchasing access. Even when previously purchased software remains downloadable for a time, the surrounding ecosystem often weakens.

**Why it matters:** Digital distribution changed game access. It made buying and downloading games easier, but it also moved ownership into account systems, licences and server-side infrastructure. When that infrastructure changes, the user's library can become harder to preserve.

**Preservation impact:** Store closures can affect:

- base games;
- patches;
- DLC;
- demos;
- manuals;
- trailers;
- themes;
- avatars;
- free promotional content;
- metadata and screenshots;
- purchase records.

**Lesson:** A digital purchase should not be treated as a preservation copy. If a platform matters, document its store, update system, account model and shutdown history while the information is still available.

## Online-Only Single-Player Games

**Category:** Design and access risk.

**What happened:** Some games require online authentication or server access even when the main experience is single-player or mostly local.

**Why it matters:** Online requirements can turn a preserved executable into a locked door. If the server is unavailable, the game may fail to launch, fail to save, lose content or become permanently incomplete.

**Preservation impact:** Online dependency can make long-term access depend on:

- account servers;
- licence checks;
- matchmaking services;
- cloud save systems;
- content delivery networks;
- daily challenge systems;
- anti-cheat or DRM services.

**Lesson:** If a game needs a server, the server behaviour is part of the preservation problem.

## Vanishing Patches

**Category:** Update preservation failure.

**What happened:** Many games ship in one state and become substantially different after updates. When patch access disappears, the best-known version of the game may disappear too.

**Why it matters:** Patches can fix crashes, restore performance, change balance, add content, remove bugs or even make a game playable. A disc or cartridge may not represent the final or most historically relevant state.

**Preservation impact:** Without patch preservation, future users may be left with:

- broken launch versions;
- missing DLC compatibility;
- unfixed save corruption;
- removed online compatibility;
- worse performance;
- missing localisation fixes;
- incomplete feature sets.

**Lesson:** Preserve updates alongside base releases. Treat patches as part of the historical record, not disposable afterthoughts.

## DRM That Outlives Its Purpose

**Category:** Access and ownership failure.

**What happened:** DRM systems are often introduced to protect launch-window sales. Years later, the commercial moment has passed, but the access restriction remains.

**Why it matters:** Old DRM can make legitimate copies harder to run than unauthorised copies. It can depend on activation servers, drivers, account systems or platform clients that may not survive.

**Preservation impact:** DRM can create problems such as:

- broken installation on modern systems;
- activation limits;
- unavailable authentication servers;
- blocked offline play;
- compatibility issues with Wine or Proton;
- dependence on dead clients.

**Lesson:** Long-term access should be part of software design. If DRM is kept, there should be a sunset plan.

## Lost Manuals and Documentation

**Category:** Context loss.

**What happened:** Many games relied on manuals, maps, code wheels, reference cards, keyboard overlays or printed control sheets. Digital re-releases often omit them.

**Why it matters:** Some older games expected the player to read the manual. Controls, copy protection, story context and mechanical explanation were often outside the game itself.

**Preservation impact:** Without documentation, a game can become playable but confusing, technically functional but historically incomplete.

**Lesson:** Manuals, inserts, maps and packaging are not decoration. They are part of the software's original interface.

## Abandoned Multiplayer Services

**Category:** Online service loss.

**What happened:** Multiplayer services are regularly shut down when games age, companies restructure or infrastructure costs no longer seem justified.

**Why it matters:** Multiplayer modes are not optional trivia for many games. They may be the main reason the game mattered.

**Preservation impact:** Losing multiplayer can remove:

- matchmaking;
- leaderboards;
- clans and guilds;
- downloadable ghosts;
- player-created levels;
- online co-op;
- competitive history;
- seasonal content.

**Lesson:** If the community cannot document or reimplement the service, a major part of the game may vanish.

## Bad Metadata as Fake Order

**Category:** Library management failure.

**What happened:** A frontend can make a messy collection look polished by attaching artwork and descriptions to files that are wrongly named, duplicated, corrupt or unverified.

**Why it matters:** Presentation can hide structural problems. A beautiful frontend is not proof of a healthy library.

**Preservation impact:** Bad metadata can lead to:

- duplicated entries;
- wrong regional information;
- incorrect release years;
- mismatched artwork;
- confusion between hacks, translations and original releases;
- broken launch commands;
- false confidence.

**Lesson:** Metadata should describe a clean library, not camouflage a bad one.

## Mystery BIOS Folders

**Category:** Configuration failure.

**What happened:** BIOS files are often copied into many emulator, frontend and RetroArch folders with little documentation.

**Why it matters:** When something breaks, the user no longer knows which file is being used, where it came from or whether it matches the expected checksum.

**Preservation impact:** Poor BIOS management causes:

- inconsistent emulator behaviour;
- region confusion;
- impossible troubleshooting;
- accidental deletion;
- duplicated files;
- unclear legality and provenance.

**Lesson:** Keep BIOS files in one controlled source folder and link or copy them deliberately only when needed.

## Disposable Launchers

**Category:** Platform dependency risk.

**What happened:** Some PC games depend on launchers that may change, merge, become unsupported or disappear.

**Why it matters:** A launcher can become the gatekeeper for games that would otherwise run locally. If the launcher fails, the game may fail too.

**Preservation impact:** Launcher dependency can affect:

- authentication;
- updates;
- cloud saves;
- DLC entitlement;
- command-line launching;
- offline mode;
- Wine and Proton compatibility.

**Lesson:** Document how a game launches, what it depends on and whether an offline path exists.

## How to Write Future Entries

Future Hall of Shame entries should follow this pattern:

```markdown
## Name

**Category:** Company, service, policy, product or incident.

**What happened:** A factual description.

**Why it matters:** The preservation or user impact.

**Evidence:** Official sources, archived pages or respected reporting.

**Lesson:** What future projects should learn.
```

Every entry should be fair enough that it could be read by someone who disagrees and still recognise the facts.

## Key Points

- The Hall of Shame is about lessons, not tantrums.
- Digital ownership depends on infrastructure.
- Store closures can remove more than games.
- Patches and DLC are part of preservation.
- Online services can be part of the game itself.
- DRM needs a sunset plan.
- Metadata can hide a bad library.
- BIOS organisation must be deliberate.

## What Comes Next

After naming the failures, the book turns to the tools that help prevent them. The next chapter introduces the Preservation Toolkit: the projects, formats and verification workflows that keep retro libraries understandable.