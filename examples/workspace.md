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
