# AGENTS.md

Guidance for AI agents working with the `rustdoc-llms` crate. This file is
both for agents *using* the tool to help a user document a Rust crate and
for agents *contributing* to this crate's source.

If you are a human, see [CONTRIBUTING.md](./CONTRIBUTING.md) for the
contributor guide and [README.md](./README.md) for usage.

## Part A — Project at a glance

`rustdoc-llms` is a small Rust CLI that generates an `llms.txt` file for a
Rust crate by shelling out to `cargo doc` (with the unstable JSON output
format) and then `rustdoc-md`. It is intentionally a thin wrapper.

**Key files:**

| Path | Purpose |
| --- | --- |
| `src/main.rs` | Binary entry point and the four public orchestration functions |
| `src/cargo_helpers.rs` | Parses `Cargo.toml` to discover the crate's lib name |
| `help/releasing/index.md` | The release workflow steps |
| `Cargo.toml` | Crate metadata and dependencies |

**Upstream dependencies that matter:**

- `cargo doc` — invoked with `--no-deps` and JSON output format
- `rustdoc-md` — invoked as a separate CLI; must be installed by the user
- The unstable `-Z unstable-options --output-format json` rustdoc flag —
  enabled on stable toolchains via `RUSTC_BOOTSTRAP=1`

**Stability note:** The rustdoc JSON format is unstable and the format
version is recorded in the generated `llms.json`. A `cargo` or
`rustdoc-md` upgrade can change the schema; always verify the generated
file looks reasonable after a toolchain change.

## Part B — Using rustdoc-llms

For agents helping a user document a Rust crate.

### When to suggest the tool

- The user asks for `llms.txt`, "docs for an LLM", or "documentation an
  AI can read" for their Rust crate
- The user wants the [`llms.txt` convention](https://llmstxt.org/) for a
  Rust project
- The user is preparing a release and wants regenerated AI-readable docs

### When NOT to suggest it

- The project is not a Rust crate
- The crate does not currently build (`cargo build` fails first)
- The user already uses a different docs-for-LLMs tool — defer
- The user is in an unusual workspace layout where `Cargo.toml` is not
  the working-directory root

### Prerequisites checklist

Before running, verify:

- [ ] The current directory contains a `Cargo.toml`
- [ ] `cargo build` succeeds
- [ ] `which rustdoc-llms` returns a path (else: `cargo install rustdoc-llms`)
- [ ] `which rustdoc-md` returns a path (else: `cargo install rustdoc-md`)

### Standard invocation

```sh
rustdoc-llms
```

### Expected outputs

- `target/doc/<crate_name>.json` — `<crate_name>` is the lib target name
  with `-` replaced by `_`
- `target/doc/llms.txt` — Markdown LLMS file

### Post-processing patterns

Copy to the crate root for the `llms.txt` convention:

```sh
cp target/doc/<crate_name>.json llms.json
cp target/doc/llms.txt llms.txt
```

Whether to commit the files: yes if they should track release versions;
no if they would create noisy diffs on every change.

### Failure modes and diagnosis

| Symptom | Likely cause | Fix |
| --- | --- | --- |
| `rustdoc-md: command not found` | Runtime dep missing | `cargo install rustdoc-md` |
| `target/doc/<crate>.json` missing | `cargo doc` failed | Run `cargo doc --no-deps` directly and read errors |
| Empty or near-empty `llms.txt` | Crate has no public items | Expected; not a bug |
| Format version mismatch warning | rustdoc JSON schema changed | Upgrade `rustdoc-md`; if still stuck, pin a known-good toolchain |

## Part C — Contributing to rustdoc-llms

For agents working on this crate's source.

### Build and test commands

```sh
cargo build              # Build the binary
cargo test               # Run tests (currently zero — adding tests welcome)
cargo doc --no-deps      # Build rustdoc; warnings should not regress
cargo run                # Build and run against the current crate
```

### Code conventions observed in existing code

- Every public item has a rustdoc comment with an `Example` block
- Path arguments use `impl AsRef<Path>` rather than `&Path` or `&str`
- Shell-out uses `std::process::Command::new(...).args(...).output()`
- Infrastructure failures (process spawn) surface via `.expect("...")`
- Fallible parsing returns `Result<_, Box<dyn std::error::Error>>`
- Examples in `src/main.rs` are marked `rust,ignore` because the crate
  is binary-only; if a lib target is added, drop `,ignore`

### Where logic lives

- `main.rs` orchestrates: top-level `main()` plus four public functions
  for path discovery and command invocation
- `cargo_helpers.rs` parses `Cargo.toml` to discover the crate's lib name

### Release workflow

See [`help/releasing/index.md`](./help/releasing/index.md). Do not
duplicate those steps here — they are the source of truth. Summary:
update version in `Cargo.toml`, verify, regenerate `llms.*`, commit, tag,
push tags, `cargo publish`.

### What to avoid

- Adding heavy dependencies — the crate is intentionally small
- Swallowing errors silently — surface or `.expect` them
- Breaking the existing CLI surface (`rustdoc-llms` with no arguments)
- Premature feature work — track in issues first
- Adding `unwrap()` where `.expect("descriptive message")` would be more
  diagnostic

## Part D — Quick reference

### Cheat sheet

```sh
# Install once
cargo install rustdoc-llms
cargo install rustdoc-md

# Generate
rustdoc-llms

# Outputs
ls target/doc/*.json target/doc/llms.txt

# Copy to crate root
cp target/doc/<crate>.json llms.json
cp target/doc/llms.txt llms.txt

# Verify
head llms.txt
```

### File locations

| Item | Location |
| --- | --- |
| Binary entry point | `src/main.rs` |
| Cargo.toml parser | `src/cargo_helpers.rs` |
| Release process | `help/releasing/index.md` |
| Examples | `examples/` |
| Design spec for this doc effort | `docs/superpowers/specs/2026-04-27-comprehensive-docs-design.md` |
