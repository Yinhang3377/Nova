CSV audit reports for GitHub Actions failing runs

Files:
- failing_runs.csv — consolidated export of failing workflow runs
- failing_runs_non_synthetic.csv — likely real failures (not workflow-file parse issues)
- failing_runs_synthetic.csv — synthetic failures caused by invalid workflow files

Usage:
- These files are kept for audit and review. Reviewers can download from the PR or clone the repo.
