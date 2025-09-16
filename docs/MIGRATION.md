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

---

## Admin checklist (action items before or after merge)

- [ ] Go to Repository → Settings → Pages and set Source to `gh-pages` branch (or verify the branch after first deployment).
- [ ] If using a custom domain, add DNS records and configure the custom domain in Pages settings.
- [ ] Verify `GITHUB_TOKEN` has repository write permissions in repository settings if your organization policy restricts Actions permissions.
- [ ] After merging, check Actions → docs: build & deploy run logs to confirm `site/` was pushed to `gh-pages`.
- [ ] Visit https://Yinhang3377.github.io/Nova/ to validate content.
