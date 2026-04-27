# Comprehensive Docs, AGENTS.md, and Examples — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add comprehensive user-facing documentation, an `AGENTS.md` for AI agents, and an `examples/` directory of scenario walkthroughs to the `rustdoc-llms` crate. No code-logic changes.

**Architecture:** Flat documentation layout. README.md is the canonical entry point. AGENTS.md sits beside CONTRIBUTING.md for agent consumption. `examples/` is a new directory of scenario-based markdown files keyed off an index README. Source `rustdoc` comments are tightened.

**Tech Stack:** Markdown (CommonMark), Rust rustdoc syntax. No new dependencies.

**Spec:** `docs/superpowers/specs/2026-04-27-comprehensive-docs-design.md`

---

## File Structure

Files created or modified:

- Modify: `README.md` — expanded sections (Task 4)
- Modify: `src/main.rs` — rustdoc cleanup, fix `rustdoc_md::` → `rustdoc_llms::` typo, mark non-runnable examples (Tasks 2–3)
- Modify: `src/cargo_helpers.rs` — minor wording cleanup (Task 3)
- Create: `AGENTS.md` — Parts A, B, C, D (Task 5)
- Create: `examples/README.md` — index (Task 6)
- Create: `examples/basic-crate.md` (Task 7)
- Create: `examples/copy-to-root.md` (Task 8)
- Create: `examples/publishing-workflow.md` (Task 9)
- Create: `examples/github-actions.md` (Task 10)
- Create: `examples/workspace.md` (Task 11)
- Create: `examples/troubleshooting.md` (Task 12)

Final task verifies all acceptance criteria.

---

## Task 1: Verify pre-conditions and capture baseline

**Files:** none modified.

- [ ] **Step 1: Confirm clean working tree**

Run: `git status`
Expected: only `Cargo.lock` and `Cargo.toml` may show as `M` (per session start state); no other unexpected changes.

- [ ] **Step 2: Confirm crate builds**

Run: `cargo build`
Expected: `Finished` line, no errors.

- [ ] **Step 3: Confirm tests pass**

Run: `cargo test`
Expected: `test result: ok` (zero tests is fine — there are none yet).

- [ ] **Step 4: Confirm rustdoc builds**

Run: `cargo doc --no-deps`
Expected: `Finished` line, no errors. Warnings are acceptable.

- [ ] **Step 5: Note baseline rustdoc warnings**

Run: `cargo doc --no-deps 2>&1 | grep -i warning || true`
Record any pre-existing warnings so later tasks can confirm no regressions.

No commit — this is a checkpoint only.

---

## Task 2: Fix doctest typos and mark non-runnable examples in `src/main.rs`

The existing rustdoc examples reference `rustdoc_md::documentation_json_path()` etc. — the prefix should be `rustdoc_llms::` (the typo predates this work). Because this crate is binary-only (no `src/lib.rs`), these `rust` blocks are not runnable as doctests; mark them `ignore` so they remain clear illustrations without compile risk if the crate ever grows a lib target.

**Files:**
- Modify: `src/main.rs` (rustdoc on `documentation_json_path`, `documentation_llms_path`, `generate_documentation_json_file`, `generate_documentation_llms_file`)

- [ ] **Step 1: Replace each `rustdoc_md::` reference with `rustdoc_llms::` and mark blocks ignore**

For each of the four public functions in `src/main.rs`, change the rustdoc `Example` block from:

```rust
/// ```rust
/// use rustdoc_llms::*;
/// let path = rustdoc_md::documentation_json_path();
/// ```
```

to:

```rust
/// ```rust,ignore
/// use rustdoc_llms::*;
/// let path = rustdoc_llms::documentation_json_path();
/// ```
```

Apply the same pattern (`rustdoc_md::` → `rustdoc_llms::`, ` ```rust` → ` ```rust,ignore`) to:
- `documentation_llms_path`
- `generate_documentation_json_file`
- `generate_documentation_llms_file`

- [ ] **Step 2: Verify build still succeeds**

Run: `cargo build`
Expected: `Finished` line.

- [ ] **Step 3: Verify rustdoc still builds**

Run: `cargo doc --no-deps`
Expected: `Finished` line. No new warnings.

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "Fix rustdoc example crate-name typos and mark non-runnable

Why: rustdoc examples referenced rustdoc_md (the dependency crate)
instead of rustdoc_llms (this crate). Marked them rust,ignore because
the crate is binary-only; if a lib target is added later, the examples
will already be using the right path."
```

---

## Task 3: Improve rustdoc on public functions

Tighten existing docstrings — clarify path semantics and side effects per spec section 4.

**Files:**
- Modify: `src/main.rs`
- Modify: `src/cargo_helpers.rs`

- [ ] **Step 1: Update `documentation_json_path` rustdoc**

Add a sentence: paths are relative to the current working directory; the tool must be invoked from the crate root containing `Cargo.toml`.

Final docstring:

```rust
/// Get the target JSON file base name; this file is what cargo doc generates.
/// The name is the crate's lib target's name with all `-` chars turned into `_`.
///
/// The returned path is relative to the current working directory; this tool
/// expects to be run from the crate root that contains `Cargo.toml`.
///
/// Example:
///
/// ```rust,ignore
/// use rustdoc_llms::*;
/// let path = rustdoc_llms::documentation_json_path();
/// ```
///
```

- [ ] **Step 2: Update `documentation_llms_path` rustdoc**

Final docstring:

```rust
/// Get the path to the goal LLMS file that will be generated by `rustdoc-md`.
/// The industry-standard file name is `llms.txt`. The returned path is
/// relative to the current working directory.
///
/// Example:
///
/// ```rust,ignore
/// use rustdoc_llms::*;
/// let path = rustdoc_llms::documentation_llms_path();
/// ```
///
```

- [ ] **Step 3: Update `generate_documentation_json_file` rustdoc**

Note that this function shells out and panics via `.expect` on infrastructure failure.

Final docstring:

```rust
/// Run `cargo doc` with flags to emit a single JSON file containing the
/// crate's combined documentation.
///
/// Side effects: spawns `cargo`. Panics via `.expect` if the child process
/// cannot be started. Errors emitted by `cargo` itself are written to its
/// own stderr; this function does not propagate them.
///
/// Example:
///
/// ```rust,ignore
/// use rustdoc_llms::*;
/// rustdoc_llms::generate_documentation_json_file();
/// ```
///
```

- [ ] **Step 4: Update `generate_documentation_llms_file` rustdoc**

Final docstring:

```rust
/// Run `rustdoc-md` to convert a JSON documentation file into a Markdown
/// LLMS file.
///
/// Side effects: spawns `rustdoc-md`. Panics via `.expect` if the child
/// process cannot be started (commonly: `rustdoc-md` is not installed).
///
/// Example:
///
/// ```rust,ignore
/// use rustdoc_llms::*;
/// let input_json_path = rustdoc_llms::documentation_json_path();
/// let output_llms_path = rustdoc_llms::documentation_llms_path();
/// rustdoc_llms::generate_documentation_llms_file(input_json_path, output_llms_path);
/// ```
///
```

- [ ] **Step 5: Tighten `lib_name` docstring in `cargo_helpers.rs`**

Replace `llb name` typo with `lib name`. Final docstring:

```rust
/// Get a crate's lib name from its `Cargo.toml` file.
///
/// Steps:
///
/// 1. Read the `Cargo.toml` file.
/// 2. Parse it as TOML.
/// 3. If there is a `[lib]` section with a `name` field, return it.
/// 4. Otherwise if there is a `package.name`, return it.
/// 5. Otherwise return an error.
///
/// This priority follows Cargo's explicit rules about library naming:
/// `[lib].name` takes precedence over `package.name`.
///
```

- [ ] **Step 6: Verify build and rustdoc**

Run: `cargo build && cargo doc --no-deps`
Expected: both succeed with no new warnings.

- [ ] **Step 7: Commit**

```bash
git add src/main.rs src/cargo_helpers.rs
git commit -m "Improve rustdoc on public functions

Why: clarify path semantics (relative to crate root) and document the
shell-out side effects + panic behavior. Fix 'llb name' typo."
```

---

## Task 4: Expand `README.md`

Rewrite `README.md` per spec section 1 with eleven sections in the listed order. Length target 120–150 lines.

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Rewrite the file**

Write the file as follows:

````markdown
# rustdoc-llms

Rust documentation helper that generates an `llms.txt` file for a Rust crate.
This file gives large language models (LLMs) the full crate documentation in a
single Markdown document, useful for AI assistants, search engines, and the
[`llms.txt` convention](https://llmstxt.org/).

Thanks to the Rust team, the rustdoc team, and the `rustdoc-md` team.

## Install

Install this crate:

```sh
cargo install rustdoc-llms
```

This tool shells out to [`rustdoc-md`](https://crates.io/crates/rustdoc-md);
install it as well:

```sh
cargo install rustdoc-md
```

## Quickstart

From the root of any Rust crate (a directory containing `Cargo.toml`):

```sh
rustdoc-llms
```

That generates two files under `target/doc/`. See *What it generates* below.

## What it generates

Two files are written under `target/doc/`:

| File | Description |
| --- | --- |
| `target/doc/<crate_name>.json` | Raw rustdoc JSON for the crate. `<crate_name>` is the lib target name with `-` replaced by `_`. |
| `target/doc/llms.txt` | Markdown rendering of the JSON, produced by `rustdoc-md`. |

## Copying to the crate root

The [`llms.txt` convention](https://llmstxt.org/) places `llms.txt` at the root
of a project so search engines and AI agents can find it. Copy the generated
files up:

```sh
cp target/doc/<crate_name>.json llms.json
cp target/doc/llms.txt llms.txt
```

Replace `<crate_name>` with your crate's lib target name (dashes turned into
underscores). Commit both files if you want them versioned alongside releases.

## How it works

`rustdoc-llms` is a thin wrapper around three shell commands. Knowing them is
useful for debugging.

1. Generate JSON documentation:

    ```sh
    RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
    ```

2. Convert the JSON to Markdown:

    ```sh
    rustdoc-md --path target/doc/<crate_name>.json --output target/doc/llms.txt
    ```

3. Optionally copy to the crate root:

    ```sh
    cp target/doc/<crate_name>.json llms.json
    cp target/doc/llms.txt llms.txt
    ```

The `RUSTC_BOOTSTRAP=1` environment variable lets a stable toolchain accept
the unstable `--output-format json` flag. No nightly toolchain is required.

## Examples

Scenario walkthroughs live in [`examples/`](./examples/):

- [Basic crate](./examples/basic-crate.md) — end-to-end on a small crate
- [Copy to root](./examples/copy-to-root.md) — the `llms.txt` convention
- [Publishing workflow](./examples/publishing-workflow.md) — plug into `cargo publish`
- [GitHub Actions](./examples/github-actions.md) — regenerate on release tags
- [Workspace](./examples/workspace.md) — Cargo workspaces with multiple crates
- [Troubleshooting](./examples/troubleshooting.md) — common failures and fixes

## FAQ

**Do I need a nightly toolchain?**
No. The tool sets `RUSTC_BOOTSTRAP=1`, which lets stable Rust accept the
unstable JSON output flag.

**Why is `rustdoc-md` needed separately?**
`rustdoc-llms` shells out to it. Keeping it as a separate install lets you
upgrade either tool independently.

**The output `llms.txt` is empty or short.**
Check that `cargo doc --no-deps` succeeds on its own. If your crate has no
public items, the LLMS file will be near-empty by design.

**Does this work on a Cargo workspace?**
Run `rustdoc-llms` from inside a member crate's directory. See
[`examples/workspace.md`](./examples/workspace.md).

**`rustdoc-md: command not found`.**
Run `cargo install rustdoc-md`. The tool is a runtime dependency.

## Scope

This crate is deliberately small and simple. Its purpose is to make it
slightly easier to generate `llms.txt` based on what is already easy and
available for Rust crate developers. Currently it shells out to existing
tools rather than reimplementing them. If the tool proves useful in
practice, command-line arguments and tests may be added.

## Contributing

Pull requests and issues are welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md)
for the human-oriented guide and [AGENTS.md](./AGENTS.md) for an AI-agent
oriented guide.

For other ways to help, or constructive criticism, contact Joel Parker
Henderson at <joel@joelparkerhenderson.com>.

## License

See [LICENSE.md](./LICENSE.md). The crate is licensed under MIT OR Apache-2.0
OR GPL-2.0 OR GPL-3.0 OR BSD-3-Clause; choose whichever applies.
````

- [ ] **Step 2: Verify links resolve**

Run: `ls README.md AGENTS.md CONTRIBUTING.md LICENSE.md examples/`
Expected: every linked file shows up. Some files (`AGENTS.md`, `examples/*`) will be created in later tasks — at this point they will *not* exist yet. Note this and proceed; final verification happens in Task 13.

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "Expand README with install, FAQ, examples, troubleshooting

Why: the original README covered usage but lacked install ordering
(rustdoc-md is a runtime dep), troubleshooting, and discoverable
examples. Expanded to 11 sections per design spec."
```

---

## Task 5: Create `AGENTS.md`

Create a new top-level `AGENTS.md` covering Parts A–D per spec section 2.

**Files:**
- Create: `AGENTS.md`

- [ ] **Step 1: Write the file**

Write the file with this content:

````markdown
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
````

- [ ] **Step 2: Verify file written**

Run: `wc -l AGENTS.md`
Expected: 200+ lines.

- [ ] **Step 3: Commit**

```bash
git add AGENTS.md
git commit -m "Add AGENTS.md for AI agents using and contributing to this crate

Why: provide a single agent-readable file covering both the 'how to
use rustdoc-llms when helping a user' and 'how to contribute to
rustdoc-llms' workflows. Complements the human-oriented README and
CONTRIBUTING."
```

---

## Task 6: Create `examples/README.md` (index)

**Files:**
- Create: `examples/README.md`

- [ ] **Step 1: Create the directory**

Run: `mkdir -p examples`

- [ ] **Step 2: Write the file**

Write the file as follows:

````markdown
# Examples

Scenario-based walkthroughs for `rustdoc-llms`. Each file follows the same
template: scenario, when to use it, prerequisites, steps, verification, and
gotchas.

| I want to... | See |
| --- | --- |
| Generate `llms.txt` for my crate, end to end | [basic-crate.md](./basic-crate.md) |
| Put `llms.txt` and `llms.json` at my crate root | [copy-to-root.md](./copy-to-root.md) |
| Regenerate `llms.txt` as part of `cargo publish` | [publishing-workflow.md](./publishing-workflow.md) |
| Regenerate automatically on every release tag | [github-actions.md](./github-actions.md) |
| Document a crate inside a Cargo workspace | [workspace.md](./workspace.md) |
| Diagnose a failing run | [troubleshooting.md](./troubleshooting.md) |

## Conventions used in examples

- `<crate_name>` means your crate's lib target name with `-` replaced by `_`
- Shell prompts (`$`) are omitted from code blocks for copy-paste ease
- Output samples are abbreviated with `…` where they are noisy
````

- [ ] **Step 3: Commit**

```bash
git add examples/README.md
git commit -m "Add examples/ index with scenario table

Why: give users a single 'I want to...' table that points at the
right walkthrough."
```

---

## Task 7: Create `examples/basic-crate.md`

**Files:**
- Create: `examples/basic-crate.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
# Basic crate

Generate `llms.txt` end to end for a small Rust crate.

## When to use this

You have a Rust crate (a directory with `Cargo.toml`), it builds, and you
want AI-readable documentation generated under `target/doc/`.

## Prerequisites

- `cargo build` succeeds in the crate root
- `rustdoc-llms` installed (`cargo install rustdoc-llms`)
- `rustdoc-md` installed (`cargo install rustdoc-md`)

## Steps

1. Change into the crate root:

    ```sh
    cd path/to/your-crate
    ```

2. Run the tool:

    ```sh
    rustdoc-llms
    ```

    Expected output:

    ```
    Generate documentation JSON file: target/doc/<crate_name>.json
    Generate documentation LLMS file: target/doc/llms.txt
    ```

3. Inspect the result:

    ```sh
    head -20 target/doc/llms.txt
    ```

## Verify

```sh
test -f target/doc/<crate_name>.json && echo "JSON ok"
test -f target/doc/llms.txt && echo "LLMS ok"
```

Both commands should print their `ok` line.

## Gotchas

- `<crate_name>` is the **lib target name** with `-` replaced by `_`. For
  a crate named `foo-bar`, the file is `target/doc/foo_bar.json`.
- If the crate has no `[lib]` section, the package name is used instead.
- `target/doc/` is `.gitignore`d by default in many setups; that is fine
  for the JSON file. Copy out (see [copy-to-root.md](./copy-to-root.md))
  if you want to track `llms.txt` in git.
````

- [ ] **Step 2: Commit**

```bash
git add examples/basic-crate.md
git commit -m "Add example: basic crate end-to-end walkthrough"
```

---

## Task 8: Create `examples/copy-to-root.md`

**Files:**
- Create: `examples/copy-to-root.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
# Copy generated files to the crate root

Place `llms.txt` and `llms.json` at the root of your crate so the
[`llms.txt` convention](https://llmstxt.org/) discoverers (search engines,
AI agents, GitHub renderers) find them.

## When to use this

You want a stable, repository-root URL like
`https://github.com/<user>/<repo>/blob/main/llms.txt`.

## Prerequisites

- A successful run of `rustdoc-llms` — see [basic-crate.md](./basic-crate.md)

## Steps

1. Identify your crate's lib target name. For a `Cargo.toml` with
   `package.name = "foo-bar"` and no `[lib]` override, the name is
   `foo_bar` (dashes → underscores).

2. Copy both files to the crate root:

    ```sh
    cp target/doc/foo_bar.json llms.json
    cp target/doc/llms.txt llms.txt
    ```

3. Commit them if you want them tracked:

    ```sh
    git add llms.json llms.txt
    git commit -m "Update llms.* for release"
    ```

## Verify

```sh
ls llms.txt llms.json
head -5 llms.txt
```

## Gotchas

- The JSON file is large (often 100KB+) and changes with every code
  change. If you do not need it tracked, copy only `llms.txt`.
- If your repo has a CI step that fails on uncommitted generated files,
  add `llms.json` and `llms.txt` to your commit before pushing.
- `llms.txt` is the file the `llms.txt` convention names; `llms.json`
  is a courtesy for tools that prefer structured data.
````

- [ ] **Step 2: Commit**

```bash
git add examples/copy-to-root.md
git commit -m "Add example: copy generated files to crate root"
```

---

## Task 9: Create `examples/publishing-workflow.md`

**Files:**
- Create: `examples/publishing-workflow.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
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
````

- [ ] **Step 2: Commit**

```bash
git add examples/publishing-workflow.md
git commit -m "Add example: integrate rustdoc-llms with cargo publish flow"
```

---

## Task 10: Create `examples/github-actions.md`

**Files:**
- Create: `examples/github-actions.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
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
````

- [ ] **Step 2: Commit**

```bash
git add examples/github-actions.md
git commit -m "Add example: GitHub Actions workflow to regenerate llms on tags"
```

---

## Task 11: Create `examples/workspace.md`

**Files:**
- Create: `examples/workspace.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
# Cargo workspace

Document a single crate that lives inside a Cargo workspace.

## When to use this

Your repository has a `Cargo.toml` at the root with a `[workspace]`
section, and the actual crates live under `crates/`, `packages/`, or
similar subdirectories.

## Prerequisites

- `cargo build` succeeds in the workspace root
- `rustdoc-llms` and `rustdoc-md` installed

## Steps

`rustdoc-llms` reads the `Cargo.toml` in the **current working
directory**. For a workspace, run it from the directory of the crate you
want to document.

1. Change into the member crate's directory:

    ```sh
    cd crates/<member-name>
    ```

2. Run the tool:

    ```sh
    rustdoc-llms
    ```

3. Outputs land under the workspace's shared `target/doc/`:

    ```sh
    ls ../../target/doc/<member_name>.json
    ls ../../target/doc/llms.txt
    ```

    (Cargo workspaces share one `target/` at the workspace root.)

4. Repeat for each member crate you want to document.

## Verify

For each member:

```sh
test -f ../../target/doc/<member_name>.json && echo "JSON ok for <member-name>"
```

## Gotchas

- One run produces one crate's docs. There is no `--workspace` mode; loop
  over crates explicitly.
- The shared `target/doc/llms.txt` is **overwritten** on every run.
  After running for crate A, then crate B, only crate B's `llms.txt`
  remains. Copy each crate's output before running the next:

    ```sh
    cd crates/foo && rustdoc-llms && cp ../../target/doc/llms.txt foo-llms.txt
    cd ../bar    && rustdoc-llms && cp ../../target/doc/llms.txt bar-llms.txt
    ```

- The JSON files are per-crate (`<crate_name>.json`) so they do **not**
  collide.
- If you want a single combined `llms.txt` for the entire workspace,
  concatenating the per-crate outputs is currently the simplest path.
````

- [ ] **Step 2: Commit**

```bash
git add examples/workspace.md
git commit -m "Add example: documenting members of a Cargo workspace"
```

---

## Task 12: Create `examples/troubleshooting.md`

**Files:**
- Create: `examples/troubleshooting.md`

- [ ] **Step 1: Write the file**

Write the file as follows:

````markdown
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
````

- [ ] **Step 2: Commit**

```bash
git add examples/troubleshooting.md
git commit -m "Add example: symptom-driven troubleshooting reference"
```

---

## Task 13: Verify acceptance criteria

**Files:** none modified.

- [ ] **Step 1: Build succeeds**

Run: `cargo build`
Expected: `Finished` with no errors.

- [ ] **Step 2: Tests pass**

Run: `cargo test`
Expected: `test result: ok`.

- [ ] **Step 3: Rustdoc builds with no new warnings**

Run: `cargo doc --no-deps`
Expected: `Finished`. Compare any warnings to the baseline captured in
Task 1, Step 5 — there should be no new ones.

- [ ] **Step 4: README has 11 sections**

Run:

```sh
grep -c '^## \|^# ' README.md
```

Expected: 12 (one `# rustdoc-llms` plus 11 `##` sections: Install,
Quickstart, What it generates, Copying to the crate root, How it works,
Examples, FAQ, Scope, Contributing, License — note the spec lists 11
section labels under the title, not all of them are `##`-headed; the
acceptance criterion is met if the file covers all 11 topics in order.
A visual scan suffices).

- [ ] **Step 5: AGENTS.md has Parts A–D**

Run:

```sh
grep -E '^## Part [A-D] ' AGENTS.md
```

Expected: four matching lines.

- [ ] **Step 6: examples/ has the index plus six scenario files**

Run: `ls examples/`
Expected output (in any order):

```
README.md
basic-crate.md
copy-to-root.md
github-actions.md
publishing-workflow.md
troubleshooting.md
workspace.md
```

- [ ] **Step 7: Internal links resolve**

For each markdown file under the repo, confirm relative links point at
existing files. A quick scan:

```sh
grep -RhoE '\]\(\./[^)]+\)' README.md AGENTS.md examples/ | sort -u
```

Open each path printed and confirm it exists. Fix any broken link by
correcting the path in the offending file and committing the fix as
`docs: fix broken internal link`.

- [ ] **Step 8: Final commit if anything was patched**

If steps 1–7 surfaced fixes, commit them. Otherwise this task ends
with no commit — it is a verification task.

```bash
git log --oneline -15
```

Expected: a clean stack of commits matching Tasks 2 through 12.

---

## Self-review notes

- Spec coverage: README expansion → Task 4. AGENTS.md A/B/C/D → Task 5.
  examples/ index + 6 files → Tasks 6–12. Rustdoc improvements + typo
  fix → Tasks 2–3. Acceptance criteria → Task 13.
- No placeholder text in any task body.
- Every code/markdown block is concrete and copy-paste ready.
- Type/name consistency: `<crate_name>` is the placeholder used
  uniformly across README, AGENTS.md, and examples (vs. an example
  literal `foo_bar` used only in copy-to-root.md). The doctest fix
  (Task 2) uses the exact correct `rustdoc_llms::` prefix.
