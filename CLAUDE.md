# SuperTOML — Claude instructions

Rust CLI + library that extracts a table from a TOML file, runs it through plugins, and emits TOML / JSON / dotenv / exports / tfvars.

## Commands

Prefer mise tasks:

| Task | Command |
|---|---|
| Build | `mise run build` |
| Test | `mise run test` |
| Lint | `mise run check` (`cargo check && cargo clippy`) |
| Format | `mise run fmt` |
| Format check | `mise run fmt-check` |
| Clean-env test | `mise run docker-test` |

CI enforces `cargo clippy -- -D warnings` and `cargo fmt --check`. Run `mise run check` and `mise run fmt` before committing.

**Stale task** — `mise run release` creates and pushes a `vX.Y.Z` tag. Do not use it. Releases are tagged automatically by CI on merge to `main`. See the `release` skill.

## Documentation is part of every change

Docs are not optional follow-up. **A code change is incomplete until the docs it affects are updated in the same commit.** Before finishing any change, walk the trigger table below.

### Doc ownership

| File | Owns |
|---|---|
[README.md](README.md) | User-facing: install, quick start, CLI reference, output formats, use cases, examples, troubleshooting |
[REFERENCE.md](REFERENCE.md) | Complete reference: every plugin directive, templating features, meta values, output formats, processing order |
[DEVELOPMENT.md](DEVELOPMENT.md) | Internals: architecture, plugin system, library usage, project structure, testing, dependencies |
[BUILDING.md](BUILDING.md) | Toolchain, mise setup, build tasks, cross-compilation, build troubleshooting |
[CHANGELOG.md](CHANGELOG.md) | Release history only. Written at release time by the `release` skill, not per-commit |

### Change → doc triggers

| Change | Update |
|---|---|
| New or changed plugin in [src/plugins/](src/plugins/) | REFERENCE.md (directive syntax + examples), DEVELOPMENT.md "Built-in Plugins" |
| Plugin execution order in [src/main.rs](src/main.rs) or [src/resolver.rs](src/resolver.rs) | REFERENCE.md "Processing Order", DEVELOPMENT.md "Variable Resolution Order" |
| New output format in [src/formatter.rs](src/formatter.rs) | README "Output Formats", REFERENCE.md "Output Formats", DEVELOPMENT.md "Adding a New Output Format", and [tests/readme_validation.rs](tests/readme_validation.rs) |
| CLI arg or flag in [src/main.rs](src/main.rs) | README "Command Line Reference" |
| New template function or filter in [src/plugins/templating.rs](src/plugins/templating.rs) | REFERENCE.md "SuperTOML-Specific Features", README "Custom Template Functions" |
| New meta value on `_` | REFERENCE.md "Meta Values", DEVELOPMENT.md "Meta Values Implementation" |
| User-visible error text in [src/error.rs](src/error.rs) | README "Error Handling", REFERENCE.md "Error Handling" |
| Public API in [src/lib.rs](src/lib.rs) | DEVELOPMENT.md "Library Usage" |
| Source file added or removed | DEVELOPMENT.md "Project Structure" |
| Dependency change in [Cargo.toml](Cargo.toml) | DEVELOPMENT.md "Dependencies" |
| Tool or task change in [mise.toml](mise.toml) | BUILDING.md |
| Test layout or test-case format change | DEVELOPMENT.md "Testing", [build.rs](build.rs) if generation is affected |
| Workflow change in [.github/workflows/](.github/workflows/) | BUILDING.md if it changes local build steps; the `release` skill if it changes the release flow |
| Release process change | The `release` skill (`.claude/skills/release/SKILL.md`) — never leave it stale |

Also update this file when a convention, command, or doc-ownership boundary changes.

## README is under test

[tests/readme_validation.rs](tests/readme_validation.rs) parses README.md by **exact string markers** — surrounding prose, code-fence languages, and blank lines are part of the match. It extracts the TOML example in "Advanced Features Example" and asserts each documented output block matches real formatter output.

Editing those README sections without updating the markers in `readme_validation.rs` breaks the build. Run `mise run test` after any README edit in that region.

## Tests are generated from files

[build.rs](build.rs) generates one test per file/dir, so adding a test means adding a file — no Rust to write:

- **TOML cases** — drop a `.toml` in [tests/toml_test_cases/](tests/toml_test_cases/). Test data plus expected output live in the same file; see DEVELOPMENT.md "Test Case Format".
- **CLI cases** — add a directory under [tests/cli_test_cases/](tests/cli_test_cases/) with a `run.sh`. Scripts must invoke the binary via `$SUPERTOML_BIN`, never a hardcoded path.

## Conventions

- **No comments** unless the *why* is non-obvious. No docstrings.
- **No speculative abstractions or error handling** for cases that can't occur.
- **Plugins** implement the `Plugin` trait, live one-per-file in [src/plugins/](src/plugins/), are re-exported from [src/plugins/mod.rs](src/plugins/mod.rs), and are registered in the resolver list in [src/main.rs](src/main.rs).
- **Never commit to `main`.** Work on a branch, open a PR — merging to `main` publishes a release.
- **Never hand-edit `version`** in Cargo.toml outside the release skill.
