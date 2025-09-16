## Docs migration PR summary

This change moves long-form documentation into the MkDocs site under `docs/` and updates navigation so the site can be published.

Files moved/added:

- `docs/readme.md` — moved from root `README.md` (long-form content)
- `docs/contributing.md` — moved from `CONTRIBUTING.md`
- `docs/index.md` — existing site index

What changed in this PR:

- Root `README.md` replaced with a short landing that points to the site.
- `mkdocs.yml` updated to include `readme.md` and `contributing.md` in nav.
- CI workflow for docs is already present (`.github/workflows/docs.yml`) and will build & deploy on push to `main` or on tags.

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
