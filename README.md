# rustdoc-llms

Rust documentation helper to generate `llms.txt` file for large language models (LLMs).

Thanks to excellent work by the Rust team, rustdoc team, and rustdoc-md team.

## What this does

This implements the help description [here](https://crates.io/crates/rustdoc-md).

Step 1: Generate JSON documentation:

```sh
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
```

Step 2: Convert from JSON into Markdown:

```sh
rustdoc-md --path target/doc/lorem_ipsum.json --output target/doc/lorem_ipsum.md
```

Step 3: Copy from Markdown file into LLMs file:

```sh
cp target/doc/lorem_ipsum.md target/doc/llms.txt
```
