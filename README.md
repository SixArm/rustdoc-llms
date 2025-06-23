# rustdoc-llms

Rust documentation helper to generate `llms.txt` file for large language models (LLMs).

Thanks to excellent work by the Rust team, rustdoc team, and rustdoc-md team.

## Scope

This Rust crate is deliberately small and simple. The purpose is to make it
slightly-easier to generate `llms` based on what's currently easy and available
for Rust crate developers.

Currently this tool calls existing command line interfaces, using `cargo` and
`rustdoc-md` with specialized compiler flags. If this tool is useful in
practice, then I intend to iterate to add command line arguments, options,
tests, and the like.

## Contributing

Pull requests are welcome. Issues are welcome.

If you want to help in other ways, or provide constructive criticism, or ask
questions directly, then please feel free to contact me. I'm Joel Parker
Henderson and my email is <joel@joelparkerhenderson.com>.

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
