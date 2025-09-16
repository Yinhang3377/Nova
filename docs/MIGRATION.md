## Docs migration PR summary

This change moves long-form documentation into the MkDocs site under `docs/` and updates navigation so the site can be published.

Files moved/added:


What changed in this PR:


Validation steps for reviewers:

1. Review moved pages for correct links and relative paths.
2. Merge PR and ensure Actions run `docs: build & deploy` completes successfully.
3. If Pages are not yet enabled, enable Pages in repository Settings and set Source to `gh-pages` branch.

Local preview:

Install mkdocs & material theme and run:

```powershell
pip install mkdocs mkdocs-material
mkdocs serve
```


## Admin checklist (action items before or after merge)

