# Contributing

## Use of AI

See [AI_POLICY.md](https://github.com/carlosferreyra/.github/blob/main/AI_POLICY.md) for our policy
on AI-assisted contributions.

## Project structure

```text
crates/
  profilectl/              # core library (business logic, config, profile resolution)
  profilectl-cli/          # binary entry point, argument parsing
  profilectl-config/       # config file types and parsing
  profilectl-interactive/  # TUI and interactive prompts
  profilectl-types/        # shared types across crates
tests/                     # integration tests (test the binary end-to-end)
```

Unit tests live inline in each crate (`#[cfg(test)]` modules at the bottom of the relevant file).
Integration tests that exercise the compiled binary or cross-crate behavior go in `tests/`.

To visualize the crate dependency graph, install
[cargo-depgraph](https://github.com/jplatte/cargo-depgraph) and graphviz, then run:

```sh
cargo depgraph --dedup-transitive-deps --workspace-only | dot -Tpng > graph.png
```

## Setup

[Rust](https://rustup.rs/) (and a C compiler) are required to build profilectl.

Invoke your development build with:

```sh
cargo run -p profilectl-cli -- <args>
```

## Branching and PRs

- Work on a `claude/<short-description>` branch (agents) or a descriptive feature branch (humans)
- Never commit directly to `main`
- Open a Pull Request for every change, no matter how small
- PRs must pass all CI checks before merging

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat: add profile extends resolution
fix: handle missing config file on first run
chore: bump clap to 4.5.4
refactor: extract profile loader into separate module
test: add integration test for profilectl init
docs: update ROADMAP with shell sourcing design
build: add cargo xwin to CI for Windows cross-check
```

Breaking changes: append `!` after the type (`feat!:`) and add a `BREAKING CHANGE:` footer.

## Before every commit

Run these in order and fix any issues before committing:

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo check --workspace
cargo nextest run
```

For changes that must work on Windows:

```sh
cargo xwin clippy --workspace --all-targets --all-features --locked -- -D warnings
```

For spell checking:

```sh
uvx typos
```

For unused dependency detection:

```sh
cargo shear
```

## Code style

- Avoid `.unwrap()`, `panic!`, `unreachable!`, and `unsafe` — use `if let`, let-chains, or return
  `Result`
- When a clippy lint must be suppressed, prefer `#[expect()]` over `#[allow()]`
- Use full, descriptive variable names: `profile_name` not `pn`, `config_path` not `cp`
- Prefer top-level imports over local imports or fully qualified paths
- No abbreviations in public API names

## Testing

We use [nextest](https://nexte.st/) as the test runner and [insta](https://insta.rs/) for snapshot
testing.

Install them once:

```sh
cargo install cargo-nextest --locked
cargo install cargo-insta --locked
```

Run all tests:

```sh
cargo nextest run
```

Run a specific test by name:

```sh
cargo nextest run -E 'test(test_name)'
```

Run and accept snapshot changes:

```sh
cargo insta test --accept --test-runner nextest
```

Review snapshots interactively:

```sh
cargo insta review
```

Guidelines:

- Add or update a test for every behavior change
- Read nearby tests and copy their style before writing new ones
- Prefer integration tests in `tests/` for user-facing behavior; unit tests for internal logic
- Prefer specific test targets (`cargo nextest run -p profilectl-config`) over running the full
  suite

## Dependencies

- Never run `cargo update` without `--precise` — update one crate at a time:

  ```sh
  cargo update --precise <crate> <version>
  ```

- Justify dependency additions in the PR description

## Windows compatibility

profilectl targets Windows as a future platform. When making changes that touch platform-specific
paths, shell invocations, or file operations, verify cross-compilation passes:

```sh
# Install once
cargo install cargo-xwin --locked
rustup target add x86_64-pc-windows-msvc

# Run on affected changes
cargo xwin clippy --workspace --all-targets --all-features --locked -- -D warnings
```
