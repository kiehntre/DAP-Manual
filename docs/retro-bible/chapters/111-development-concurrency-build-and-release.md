# Development, Concurrency and Release Hygiene

Large preservation tools are often developed while another build, experiment or
physical QA session is running. The repository is shared state. Treat it like a
collection: inspect first, mutate deliberately, preserve recovery paths.

## Worktrees and ownership

Before editing, verify branch, HEAD, status and worktrees. A dirty file may be
another person’s active lane. Do not use `git add .`, broad resets, checkout of
unrelated files, or a cleanup command just to make the tree look tidy. Stage
only the files and hunks your task owns.

When several agents or editors work in one repository:

- state the exact target files;
- check whether a worktree owns them;
- inspect ancestry before assuming a commit is missing;
- preserve unrelated dirty files unchanged;
- review the staged diff before committing.

If authority moves while QA is running, rerun the final HEAD and status checks.
A report from yesterday is not proof about today’s tree.

## Cargo contention and isolated targets

Two Rust builds sharing a target directory can waste time, corrupt assumptions
about freshness, or obscure which lane owns a failure. Check for active
`cargo`/`rustc` processes. Use limited jobs and an isolated `CARGO_TARGET_DIR`
for a focused lane when the shared target is busy. Run narrow tests first, then
the package check, then a release build only when ownership is clear.

The reliable minimum is focused tests, `cargo check`, targeted formatting and
`git diff --check`. Full workspace tests are useful at milestones, but launching
several enormous builds at once usually creates noise rather than confidence.

The same principle applies outside Rust. A Docker build, a GUI session and a
physical package install can all mutate caches or ports. Record which process
owns a service, use disposable homes and target directories, and do not restart
the capture or review service merely to make a smoke test convenient. If a
display is available through a real desktop, test there; if the sandbox cannot
reach it, report the sandbox X11 limitation rather than declaring the desktop
absent.

## Recovering Git work safely

Back up before cleanup. Mixed stashes can contain unrelated hunks; export useful
patches, inspect them, and recover only the exact files or hunks needed. Use
`git log`, `git merge-base --is-ancestor` and the reflog to prove whether work
was superseded. Drop a stash only after its useful contents are present in a
reviewed commit or patch.

Clean-tree milestones and remote checkpoints are cheaper than reconstructing a
week of archaeology. Never sacrifice an unknown dirty file for cosmetic
cleanliness.

## Packaging is a second product

An AppImage, Debian package and RPM exercise different assumptions. Validate
desktop files, AppStream metadata, icons, dependencies, binary linkage and
install/uninstall behavior in disposable distro containers. Test the CLI help
and version output. A successful local `cargo build` does not prove that a
package has the right runtime dependencies.

The current EmuWiz packaging work is **implemented/partial**: real packaging
paths, AppImage verification and Debian/RPM QA exist, while distro/toolchain
availability and some external integrations remain environment-dependent.
Use the pinned Rust toolchain where the packaging policy requires it. Keep
AppImage extraction as a fallback when FUSE is unavailable, and report that as
an environment limitation rather than an application failure.

## Release checklist

1. Confirm clean ownership and current HEAD.
2. Run focused regressions and false-positive tests.
3. Run GUI smoke at compact and normal viewports.
4. Verify version consistency and packaging metadata.
5. Build artifacts in disposable environments.
6. Install, run a safe help/version check, and uninstall.
7. Record SHA-256 hashes and remaining blockers.
8. Only then bump version, tag and publish.

Do not version-bump early. A version number should identify a release artifact,
not a hopeful work-in-progress.

## What a release report should contain

Record the authority HEAD, branch, build command, toolchain, artifact path,
timestamp, size and SHA-256. Say which tests were run and which were skipped
because of an environment limitation. For a physical GUI pass, record display,
viewport sizes and screenshot paths. For package QA, record install and
uninstall in a disposable image, not merely the fact that `cargo build` passed.

This discipline prevents a common failure: a later report repeats an old
“blocked by E0027” or “no display” note after the underlying state has changed.
Evidence-backed status is part of the release artifact.
