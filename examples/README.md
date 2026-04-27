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
