# profilectl

Profile-based dotfiles and developer machine automation.

`profilectl` is being rebuilt from the
[`carlosferreyra/rust-template`](https://github.com/carlosferreyra/rust-template)
scaffold. The product goal is to replace personal shell bootstrap scripts first,
then grow into a focused chezmoi competitor with first-class planning, machine
profiles, and tool installation.

## Current Status

This branch is a greenfield reset:

- the template scaffold is in place;
- the core crate layout is present;
- the CLI command surface is modeled but intentionally stubbed;
- the previous dotfile examples live in `fixtures/migration/current/`;
- implementation direction is documented in `ROADMAP.md` and `PLAN.md`.

## Workspace

```text
crates/profilectl/          # public entrypoint and process dispatch
crates/profilectl-cli/      # Clap command model
crates/profilectl-core/     # desired-state and plan domain model
crates/profilectl-config/   # profile schema and resolution
crates/profilectl-state/    # local managed-artifact inventory
crates/profilectl-adapters/ # filesystem, shell, git, package manager adapters
crates/xtask/               # development automation
```

## Development

```sh
cargo xtask check
cargo xtask test
cargo xtask build
```

The template pins the local toolchain with `rust-toolchain.toml`, uses edition
2024, and keeps routine workflow under `cargo xtask`.
