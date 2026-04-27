# Publishing workflow

Regenerate `llms.txt` as part of the `cargo publish` release flow so the
file shipped on crates.io always matches the released version.

## When to use this

You are preparing to release a new version of your crate to crates.io and
you want the `llms.*` files to reflect that exact version.

## Prerequisites

- The release workflow used by this crate is captured in
  [`help/releasing/index.md`](../help/releasing/index.md). The order
  below mirrors it.

## Steps

1. Bump the version in `Cargo.toml`.

2. Verify locally:

    ```sh
    cargo build --release
    cargo test
    cargo doc --no-deps
    ```

3. Regenerate the LLMS files **before** committing:

    ```sh
    rustdoc-llms
    cp target/doc/<crate_name>.json llms.json
    cp target/doc/llms.txt llms.txt
    ```

4. Commit everything together:

    ```sh
    git add Cargo.toml Cargo.lock llms.json llms.txt
    git commit -m "Release vX.Y.Z"
    ```

5. Tag and push:

    ```sh
    git tag vX.Y.Z
    git push --tags
    ```

6. Publish:

    ```sh
    cargo publish
    ```

## Verify

After `cargo publish` succeeds:

```sh
head -3 llms.txt   # Should show the new version near the top
```

Then visit `https://crates.io/crates/<crate_name>` to confirm the new
version is listed.

## Gotchas

- Generate `llms.*` **before** the commit that bumps the version, so the
  recorded version inside `llms.txt` matches the tag.
- If you forget and tag first, run `rustdoc-llms` and amend (or push a
  follow-up commit on the same tag — your call).
- `cargo publish` does not include `llms.json` / `llms.txt` in the
  uploaded crate by default (see `include` in `Cargo.toml`); they live
  in the git repository, not the registry artifact.
