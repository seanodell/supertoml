# Release

Prepare the project for the next release according to the guidelines and steps defined below.

## Guidelines

**NEVER** prepare a release while on the `main` branch
**ALWAYS** follow SemVer version number guidelines
**ALWAYS** write changelog entries as user-facing features, bug fixes and important changes
**ALWAYS** be specific and detailed

## Steps

1. Read Cargo.toml and get the version value from the [package] table
2. Find the git ref for the remote (not local) tag that matches the version value
3. Review all code changes (diffs) and git commits since that git ref (but do not include it)
4. Following SemVer guidelines, determine the next appropriate version number for the changes you discovered
5. Share the new number with me and ask if I agree
6. If I agree, update Cargo.toml and apply the new version number; otherwise stop here and wait for further instruction
7. Run the date command to determine today's date
8. Add a new entry to CHANGELOG.md for the new version number, today's date, and considering the changes you discovered
9. Review your own changelog entry and edit to remove redundancies or inaccuracies
