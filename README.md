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
