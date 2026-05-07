# Agent Guidelines

- Read CONTRIBUTING.md for guidelines on tools, workflow, and project structure
- ALWAYS work on a new `claude/<short-description>` branch — never commit directly to `main`
- ALWAYS open a Pull Request to `main` when work is complete, even for small changes
- ALWAYS use Conventional Commits: `feat:`, `fix:`, `chore:`, `refactor:`, `test:`, `docs:`,
  `build:`

## Environment bootstrap

If `cargo` is not available (e.g. ephemeral CI or claude.ai/code sessions):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

The toolchain version is pinned in the repo — do not override it:

- `rust-toolchain.toml` — pins the exact toolchain channel (`rustup` reads this automatically)

## Dev tasks

All routine checks run through xtask:

```sh
cargo xtask check   # cargo check + clippy -D warnings
cargo xtask build   # build all workspace crates
cargo xtask test    # full test suite via cargo nextest
```

Use these as gates during a session:

| Checkpoint          | When                        |
| ------------------- | --------------------------- |
| `cargo xtask check` | after every meaningful edit |
| `cargo xtask test`  | before committing           |
| `cargo xtask build` | before pushing              |

- Use `cargo run -p profilectl -- <args>` to run the binary locally during development
- Use `cargo nextest run -p <crate>` to run tests for a specific crate
- NEVER perform builds with the release profile unless asked or reproducing a performance issue
- NEVER update all dependencies at once — use `cargo update --precise <crate> <version>` for
  targeted lockfile changes
- NEVER assume clippy warnings are pre-existing; treat every warning as something to fix
- When making changes that need to work on Windows, use `cargo xwin clippy` to check
  cross-compilation

## Code style

- PREFER running specific tests over the full test suite
- PREFER integration tests under `tests/` over unit tests; unit tests live inline in the same file
  as the code they test
- ALWAYS add or update a test case for any changed behavior
- ALWAYS read and copy the style of nearby tests when adding new cases
- AVOID `panic!`, `unreachable!`, `.unwrap()`, unsafe code, and clippy rule ignores
- PREFER `if let` and let-chains (`if let … && …`) over `.unwrap()` or nested `if let`
- PREFER `#[expect()]` over `#[allow()]` when a clippy lint must be suppressed
- PREFER top-level imports over local imports or fully qualified names
- AVOID shortening variable names — use `profile_name` not `pn`, `config_path` not `cp`
