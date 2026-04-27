# Comprehensive Documentation, AGENTS.md, and Examples — Design

**Date:** 2026-04-27
**Crate:** rustdoc-llms (v0.4.0)
**Scope:** Documentation expansion. No code logic changes.

## Goal

Make `rustdoc-llms` easy to adopt and easy to maintain by:

1. Expanding `README.md` to cover install, usage, mechanics, and troubleshooting
2. Adding a top-level `AGENTS.md` for AI agents both *using* the tool and *contributing* to it
3. Adding an `examples/` directory of scenario-based walkthroughs
4. Improving rustdoc comments in `src/main.rs` and `src/cargo_helpers.rs`

The crate stays "deliberately small and simple" (its own stated scope). No new code dependencies, no CLI surface changes.

## Non-goals

- Restructuring source code
- Adding tests (acknowledged as future work in current README)
- Adding command-line arguments
- Migrating `help/` content
- Generating documentation from a static-site generator

## Approach

**Approach 1 — Minimal footprint** (chosen).

- Flat top-level layout; only one new directory (`examples/`)
- README stays the canonical entry point and grows modestly (~120–150 lines)
- AGENTS.md sits beside CONTRIBUTING.md, focused on agent-readable cues
- Examples are markdown scenario files, not Rust example binaries (this is a CLI tool, not a library-first crate)

Rejected alternatives:

- **Structured docs/** directory — over-engineering for a 180-LOC crate
- **Everything in help/** — conflates "release notes" with "usage scenarios"

## Final file layout

```
README.md                     # Expanded
AGENTS.md                     # NEW
CONTRIBUTING.md               # Unchanged
CITATION.cff                  # Unchanged
CODE_OF_CONDUCT.md            # Unchanged
LICENSE.md                    # Unchanged
Cargo.toml                    # Unchanged
src/
  main.rs                     # Improved rustdoc only
  cargo_helpers.rs            # Improved rustdoc only
help/
  releasing/
    index.md                  # Unchanged
examples/                     # NEW
  README.md                   # Index
  basic-crate.md
  copy-to-root.md
  publishing-workflow.md
  github-actions.md
  workspace.md
  troubleshooting.md
docs/
  superpowers/
    specs/
      2026-04-27-comprehensive-docs-design.md  # this file
```

## Section 1 — README.md

Sections, in order:

1. **Title + tagline** (unchanged)
2. **Install** — `cargo install rustdoc-llms`. Note that `rustdoc-md` is a runtime dependency: also install with `cargo install rustdoc-md`
3. **Quickstart** — minimal 3-line example showing invocation + outputs
4. **What it generates** — table or bulleted list of output files and their locations
5. **Copying to the crate root** — the `cp` pattern, with rationale linking to the `llms.txt` proposal so readers understand the convention
6. **How it works** — the three shell commands the tool wraps (verbatim from current README, kept for transparency)
7. **Examples** — link to `examples/` directory with one-line summaries
8. **FAQ / Troubleshooting** — entries for: nightly toolchain not required (uses `RUSTC_BOOTSTRAP=1`), `rustdoc-md` not installed, empty JSON, multi-crate workspace caveats
9. **Scope** — keep existing "deliberately small and simple" paragraph
10. **Contributing** — link to `CONTRIBUTING.md` and `AGENTS.md`
11. **License** — keep existing pointer

Target length: 120–150 lines.

## Section 2 — AGENTS.md

A top-level file an AI agent can scan once to know how to both use and contribute to this crate. Sections:

### Part A — Project at a glance

- What this crate is (2 sentences)
- Key files: `src/main.rs` (orchestration), `src/cargo_helpers.rs` (Cargo.toml parsing), `help/releasing/index.md` (release workflow)
- Upstream dependencies that matter: `cargo doc`, `rustdoc-md`, the nightly JSON format reached via `RUSTC_BOOTSTRAP=1`
- Stability note: the rustdoc JSON format is unstable; the format version is recorded in the generated `llms.json`

### Part B — Using rustdoc-llms

For agents helping a user document a crate.

- When to suggest the tool: user wants to generate `llms.txt` for a Rust crate
- Prerequisites check: both `rustdoc-llms` and `rustdoc-md` installed; current directory is a Cargo crate root with a buildable `Cargo.toml`
- Standard invocation: `rustdoc-llms`
- Expected output paths: `target/doc/<crate_name>.json` and `target/doc/llms.txt`
- Post-processing patterns: copying to crate root, committing the `llms.*` files, regenerating on release
- Failure modes and diagnosis (with the symptom → cause → fix pattern)
- When NOT to suggest: non-Rust projects, crates that don't build, unusual workspace layouts, projects that already use a different docs-for-LLMs tool

### Part C — Contributing to rustdoc-llms

For agents working on this crate.

- Build/test commands: `cargo build`, `cargo test`, `cargo doc`, `cargo run`
- Code conventions observed in the existing code:
  - rustdoc on every `pub` item with an `Example` block
  - path arguments take `impl AsRef<Path>`
  - shell-out via `std::process::Command`
  - errors surfaced with `.expect("...")` on infrastructure calls; `Result<_, Box<dyn Error>>` for fallible parsing
- Where logic lives: `main.rs` orchestrates; `cargo_helpers.rs` parses `Cargo.toml`
- Release workflow: point to `help/releasing/index.md`; do not duplicate steps here
- What to avoid: adding heavy dependencies, swallowing errors silently, breaking the CLI surface, premature feature additions (the crate is intentionally small)

### Part D — Quick reference

- Command cheat sheet (install, generate, copy, verify)
- File locations table

Target length: 200–250 lines.

## Section 3 — examples/ directory

`examples/README.md` is the index. It contains a "I want to..." → file table:

| I want to... | See |
|---|---|
| Generate llms.txt for my crate, end to end | `basic-crate.md` |
| Put `llms.txt` and `llms.json` at my crate root | `copy-to-root.md` |
| Regenerate on every `cargo publish` | `publishing-workflow.md` |
| Regenerate automatically on release tags | `github-actions.md` |
| Document a crate inside a Cargo workspace | `workspace.md` |
| Diagnose a failing run | `troubleshooting.md` |

Each scenario file follows this template:

1. **Scenario** — one sentence
2. **When to use this** — the concrete trigger
3. **Prerequisites** — what must already be true
4. **Steps** — numbered shell blocks; show expected output where useful
5. **Verify** — how to confirm it worked
6. **Variations / gotchas** — edge cases worth knowing

Target per file: 40–80 lines.

### Per-file content sketch

- **basic-crate.md** — Run `rustdoc-llms` in a small crate; verify `target/doc/<crate>.json` and `target/doc/llms.txt` exist; inspect the markdown
- **copy-to-root.md** — `cp target/doc/<crate>.json llms.json && cp target/doc/llms.txt llms.txt`; rationale (discoverability, llms.txt convention); whether to commit
- **publishing-workflow.md** — Hooking regeneration into the release workflow already documented in `help/releasing/index.md`; what order matters (regen → commit → tag → publish)
- **github-actions.md** — A minimal workflow YAML that installs `rustdoc-llms` and `rustdoc-md`, runs the tool, and uploads `llms.txt` as an artifact (or commits it back via a PR)
- **workspace.md** — Cargo workspaces: the tool runs against a single crate's `Cargo.toml`; show running it from a member crate directory; note that the JSON output is per-crate
- **troubleshooting.md** — Symptom-driven entries: `rustdoc-md: command not found`, `target/doc/<crate>.json` missing, empty/short `llms.txt`, format-version mismatch warnings

## Section 4 — rustdoc comments

Improvements only — keep the existing examples and signatures.

- `cargo_helpers::lib_name` already has good content; minor wording cleanup only
- `main.rs::main` — note that this is the binary entry point; it is fine to call the public functions individually from a build script
- Public path-returning functions — clarify that paths are relative to the current working directory (must be the crate root)
- Public `generate_*` functions — document that they shell out and that errors from the spawned commands surface as panics via `.expect`

The existing rustdoc examples in `main.rs` reference `rustdoc_md::documentation_json_path()` etc., which appears to be a typo (should be `rustdoc_llms::...`). Fix these as part of this work — they would not compile if doctests were run.

## Risks and mitigations

- **Risk:** Examples drift out of date if the underlying CLI surface ever changes.
  **Mitigation:** Keep examples shell-driven so they fail loudly when commands change; reference output filenames in one place (the index table).

- **Risk:** AGENTS.md duplicates README content and decays.
  **Mitigation:** AGENTS.md links to the README and `help/releasing/index.md` rather than restating; agents read both.

- **Risk:** The doctest in `src/main.rs` references the wrong crate name (`rustdoc_md` vs `rustdoc_llms`).
  **Mitigation:** Fix as part of the rustdoc-comment pass. Verify with `cargo test --doc` after the change.

## Acceptance criteria

- `cargo build` succeeds
- `cargo test` succeeds (including doctests, after the typo fix)
- `cargo doc --no-deps` succeeds with no new warnings
- `README.md` covers the 11 sections listed above
- `AGENTS.md` exists at repo root with parts A–D
- `examples/` contains an index plus 6 scenario files matching the layout above
- All internal links resolve (README → examples, AGENTS.md → README + help/releasing)
