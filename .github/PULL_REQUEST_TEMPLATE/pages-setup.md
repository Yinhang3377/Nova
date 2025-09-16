## Docs: Publish MkDocs site (Pages setup)

This PR prepares the repository to publish the MkDocs-generated site to GitHub Pages.

Checklist for repository owners (please complete before merging):

- [ ] Confirm Pages source is set to the `gh-pages` branch: Repository → Settings → Pages → Source → `gh-pages` branch
- [ ] (Optional) Set a custom domain under Pages settings if you have one and add required DNS records
- [ ] Verify `GitHub Pages` is enabled and the `GITHUB_TOKEN` has permission to create the `gh-pages` branch
- [ ] (Optional) Add a Pages badge to `README.md` pointing to the published site URL
- [ ] Merge PR and wait for the `docs: build & deploy` workflow to run; check Actions logs for success
- [ ] Visit the published URL (https://<owner>.github.io/<repo>/) and sanity-check pages (Home, Storage, Architecture)

If you prefer deployment only on releases/tags, change `.github/workflows/docs.yml` to trigger only on `push.tags`.

Notes for reviewers:

- This PR adds `mkdocs.yml` and `docs/index.md` and a workflow `.github/workflows/docs.yml` to build and deploy the site.
- The workflow uses `peaceiris/actions-gh-pages@v4` which is widely used for this purpose and requires no additional secrets beyond `GITHUB_TOKEN`.
