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
