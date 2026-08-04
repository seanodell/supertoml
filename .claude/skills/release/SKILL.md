---
name: release
description: Prepare and publish a SuperTOML release — pick the SemVer bump, write the CHANGELOG entry, and merge to main so CI tags and publishes. Use when the user says they are ready to release, cut a release, ship a version, or bump the version.
verified: 2026-08-04
---

# Release

## How releasing actually works

Releases are **automatic on merge to `main`**. There is no manual tagging step.

[.github/workflows/release.yml](../../../.github/workflows/release.yml) runs on every push to `main` and:

1. Reads `version` from [Cargo.toml](../../../Cargo.toml) via `cargo metadata`
2. Builds and tests all 5 targets, uploading zip + tar.gz packages
3. Extracts the `## [<version>]` section from [CHANGELOG.md](../../../CHANGELOG.md) as release notes — **fails the release if that section is missing**
4. Creates tag `v<version>` and a GitHub release with those notes and packages, marked latest

Consequences:

- **Cargo.toml is the source of truth** for the released version.
- **A missing CHANGELOG section aborts the release** after a full build. Always land both together.
- **Merging to `main` without bumping the version** re-runs the release against an existing tag. Avoid it — bundle doc-only changes into the release PR or land them alongside a bump.
- `mise run release` manually creates and pushes a tag. It predates this workflow. **Do not use it.**

## Guidelines

- **NEVER** prepare a release while on `main`
- **ALWAYS** follow SemVer
- **ALWAYS** write changelog entries as user-facing features, fixes, and important changes — not commit summaries
- **ALWAYS** be specific and detailed

## Steps

1. **Confirm branch** — run `git branch --show-current`. If on `main`, stop and ask which branch to use, or create one.
2. **Read current version** from the `[package]` table in [Cargo.toml](../../../Cargo.toml).
3. **Find the last release ref** — locate the remote tag matching that version: `git ls-remote --tags origin | grep "v<version>$"`. Use the remote tag, not a local one.
4. **Review changes since that ref**, excluding it: `git log <tag>..HEAD --oneline` and `git diff <tag>..HEAD`.
5. **Determine the next version** per SemVer from what you found.
6. **Propose it to the user and ask for agreement.** If they disagree, stop and wait.
7. **Bump** `version` in [Cargo.toml](../../../Cargo.toml).
8. **Get today's date** — run `date +%Y-%m-%d`. Never guess.
9. **Add a CHANGELOG entry** at the top of the version list in [CHANGELOG.md](../../../CHANGELOG.md), matching the existing Keep a Changelog format exactly:

   ```
   ## [X.Y.Z] - YYYY-MM-DD

   ### Added
   - ...

   ### Changed
   - ...

   ### Fixed
   - ...
   ```

   The `## [X.Y.Z]` header must match the Cargo.toml version exactly — the workflow greps for it. Omit empty sections.
10. **Re-read your own entry** and edit out redundancy and inaccuracy.
11. **Verify the docs are current** — walk the change → doc trigger table in [CLAUDE.md](../../../CLAUDE.md) against the diff from step 4. Anything missed gets fixed in this release, not after it.
12. **Run checks** — `mise run fmt-check && mise run check && mise run test`. Fix failures before proceeding.
13. **Commit** per the git-workflow skill, then push the branch.
14. **Open a PR to `main`** and tell the user that merging it publishes `v<version>`. Do not merge it yourself unless asked.

## After merge

- `Release` workflow runs on `main`; watch it with `gh run watch` or `gh run list --workflow=release.yml`
- Verify the result: `gh release view v<version>`
- If the release failed on missing notes, the CHANGELOG header did not match the Cargo.toml version. Fix the header and land it on `main`.
