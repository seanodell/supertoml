# Submit PR

Submit a PR to the GitHub repo for the changes according to the guidelines and steps defined below.

## Guidelines

**ALWAYS** use `mise exec --` to run `gh`.
**ALWAYS** write the body of the PR to a temporary file in /tmp
**NEVER** pass the body of the PR on the command-line to gh

## Steps

1. Review all changes between the current HEAD and the remote `main` branch
2. Look at the actual code differences
3. Review the most recent entry in CHANGELOG.md for context
4. Determine a good PR title that summarizes all changes in one succinct line
5. Review your own PR title to eliminate Conventional Commits features
6. Write a summary of the changes in a little more technical detail than found in CHANGELOG.md and include any other code changes
7. Review your own summary for redundancies and inaccuracies, and keep it simple and succinct
8. Open the PR body file and allow me to edit it before you continue
9. Create a PR to merge from the current branch to main
10. Print a link to the new PR as a clickable URL
