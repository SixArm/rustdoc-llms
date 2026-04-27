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
