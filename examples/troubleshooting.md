# Troubleshooting

Symptom-driven entries for the most common `rustdoc-llms` failures.

## `rustdoc-md: command not found`

**Symptom:** `rustdoc-llms` runs the JSON step, then prints something like:

```
thread 'main' panicked at 'command failed to start: …'
```

**Cause:** `rustdoc-md` is a runtime dependency and is not installed.

**Fix:**

```sh
cargo install rustdoc-md
which rustdoc-md   # confirm
rustdoc-llms
```

## `target/doc/<crate>.json` missing

**Symptom:** the JSON step appears to run but no JSON file is written.

**Cause:** `cargo doc` itself failed. `rustdoc-llms` does not surface
that error.

**Fix:** run the underlying command directly to see the real error:

```sh
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
```

Common subcauses:

- A compile error in the crate (fix the code, then retry)
- A toolchain too old to recognize `--output-format json` (upgrade Rust)

## Empty or near-empty `llms.txt`

**Symptom:** the file is created but is only a few lines long.

**Cause:** the crate has no public items, or the rustdoc JSON contained
none.

**Fix:** confirm with `cargo doc --no-deps && open target/doc/<crate>/index.html`.
If the rendered docs are also empty, this is expected — make some items
`pub` and re-run.

## Format-version mismatch

**Symptom:** `rustdoc-md` prints a warning that the JSON format version
differs from the version it expects.

**Cause:** the rustdoc JSON schema is unstable. A `cargo` upgrade may
emit a newer format than the installed `rustdoc-md` understands.

**Fix:**

```sh
cargo install rustdoc-md   # reinstall the latest
```

If that does not help, pin a known-good toolchain:

```sh
rustup toolchain install <known-good-version>
rustup override set <known-good-version>
```

## Wrong file name in `target/doc/`

**Symptom:** you expected `target/doc/foo-bar.json` but the actual file
is `target/doc/foo_bar.json`.

**Cause:** Cargo normalizes lib names by replacing `-` with `_`. This is
documented Cargo behavior, not a bug.

**Fix:** use the underscore form when copying:

```sh
cp target/doc/foo_bar.json llms.json
```

## `Permission denied` writing under `target/doc/`

**Symptom:** the run fails midway with a write error.

**Cause:** another process holds a lock on `target/`, or the directory
was created by a different user (e.g., root via `sudo cargo doc`).

**Fix:**

```sh
ls -la target/doc | head
# If owned by root: sudo chown -R "$USER" target
cargo clean
rustdoc-llms
```
