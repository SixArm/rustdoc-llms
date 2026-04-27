# GitHub Actions

Regenerate `llms.txt` automatically on every release tag and upload it
as a workflow artifact.

## When to use this

You release via tags (`vX.Y.Z`) and want CI to ship a fresh `llms.txt`
without anyone running `rustdoc-llms` locally.

## Prerequisites

- The crate builds in CI today
- You can add a workflow file under `.github/workflows/`

## Steps

1. Create `.github/workflows/llms.yml`:

    ```yaml
    name: Generate llms.txt

    on:
      push:
        tags:
          - 'v*'

    jobs:
      generate:
        runs-on: ubuntu-latest
        steps:
          - uses: actions/checkout@v4

          - name: Install Rust
            uses: dtolnay/rust-toolchain@stable

          - name: Cache cargo
            uses: Swatinem/rust-cache@v2

          - name: Install rustdoc-md
            run: cargo install rustdoc-md

          - name: Install rustdoc-llms
            run: cargo install rustdoc-llms

          - name: Generate llms files
            run: rustdoc-llms

          - name: Copy to repo root
            run: |
              CRATE_NAME=$(grep '^name' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/' | tr '-' '_')
              cp "target/doc/${CRATE_NAME}.json" llms.json
              cp target/doc/llms.txt llms.txt

          - name: Upload as artifact
            uses: actions/upload-artifact@v4
            with:
              name: llms-files
              path: |
                llms.txt
                llms.json
    ```

2. Commit and push the workflow:

    ```sh
    git add .github/workflows/llms.yml
    git commit -m "Add CI workflow to generate llms.txt on release tags"
    git push
    ```

3. Cut a release tag to trigger it:

    ```sh
    git tag v0.1.0
    git push --tags
    ```

## Verify

In the GitHub Actions UI, open the run for the new tag and confirm:

- All steps succeeded
- The `llms-files` artifact contains both `llms.txt` and `llms.json`

## Gotchas

- `cargo install` in CI is slow; the `rust-cache` action helps, but the
  first run will still take a few minutes.
- If your crate has private dependencies, add an authentication step
  before `cargo install`.
- To **commit** the regenerated files back to the repo (instead of
  uploading as artifact), replace the upload step with a commit-and-push
  step using a token with write access. Be careful: commits triggering
  more workflows can cause loops; use `[skip ci]` in the message.
