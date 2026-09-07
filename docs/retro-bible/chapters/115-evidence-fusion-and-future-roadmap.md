# Evidence Fusion and the Road Ahead

An effective identity is assembled from independent observations. It is not a
single magical score and it is not a majority vote.

## Five evidence streams

**Machine evidence** includes container signatures, boot sectors, filesystem
markers, tape timing, disc topology and bounded structural reads.

**DAT evidence** includes exact hashes, set relationships and source revision.

**Human evidence** can record a person’s label, test or verification, bound to
the exact content and date.

**Launch evidence** records that a particular profile and emulator successfully
opened the selected content. It can prove practical compatibility without
rewriting the DAT identity.

**Community evidence** is a future opt-in stream: hash-bound observations shared
without ROM bytes or private paths.

Keep every stream inspectable. A file may have strong PC-98 machine evidence,
no exact DAT match, and a human-confirmed title. That is a rich result, not a
contradiction. A different file may have a perfect filename, weak structure and
no DAT match; its name should remain a hint.

## Planned roadmap, honestly labelled

The following are useful directions, not promises that a current build ships:

- **Local Human Ground Truth:** editable USER_LABELLED, USER_TESTED and
  USER_VERIFIED records with correction and retraction.
- **Evidence Fusion:** a readable view that keeps machine, DAT, human and launch
  observations separate while presenting a practical effective identity.
- **Community Evidence Pool:** opt-in, privacy-preserving hash-bound reports for
  homebrew, demos, prototypes and uncatalogued systems.
- **Real-corpus benchmark runner:** repeatable platform, identity, boundary,
  false-positive and performance reports over a lawful fixture corpus.
- **Preservation enrichment:** more optical, tape, Japanese computer and obscure
  platform evidence without extension-only guesses.
- **GUI ambiguity workflow:** evidence cards, missing-decisive-evidence hints,
  manual confirmation and safe follow-up actions.
- **Cheat reconciliation view:** semantic versus raw duplicates, conflicts and
  provenance without an automatic winner.
- **Non-retail classification:** prototype, debug, kiosk, SDK and preservation-
  only categories distinct from “bad dump”.
- **Museum Mode:** optional historical context and collection timelines.

The discipline is as important as the features: a roadmap label must remain a
roadmap label until a tested user workflow exists.

## A compact philosophy

Preserve first. Identify from evidence. Explain decisions. Mutate explicitly.
Make changes reversible. Prefer Unknown to Wrong. Automate repetitive inspection
and ask a human when the human has information the machine cannot derive. Keep
the normal interface simple and let experts drill down. Never punish a person
for having a messy real-world collection.

That philosophy scales from one floppy disk to a home server full of optical
images. It also gives future contributors a useful boundary: add evidence and
safe review before adding automation that can destroy provenance.

## A useful user-facing summary

When several streams agree, say why: “exact logical-track match; platform
confirmed by boot evidence; launch tested with this profile.” When they do not,
say that too: “container valid; two possible platforms; no decisive boot
evidence.” The summary should be short enough for a library row and detailed
enough to expand into an audit trail. This is how a forensic engine becomes a
tool that ordinary collectors can trust.

The same summary can drive a rename preview, a frontend projection and a
support request without copying the underlying bytes. It also leaves room for
new evidence: a later DAT import or a human test can add confidence without
rewriting the earlier observation. That is the durable advantage of keeping
evidence additive and provenance-rich.
