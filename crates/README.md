# Crates

## [profilectl](./profilectl)

Public library and executable dispatch. This crate owns startup concerns:
argument parsing, tracing setup, command dispatch, and exit codes.

## [profilectl-cli](./profilectl-cli)

Clap command model for the project executable. It defines the user-facing
command surface without implementing machine effects.

## [profilectl-core](./profilectl-core)

Desired-state planning engine. This crate should stay independent from concrete
filesystem, git, shell, and package-manager effects.

## [profilectl-config](./profilectl-config)

Profile schema and resolution. It will load TOML profiles, merge bundles and
inheritance, and lower declarations toward core desired state.

## [profilectl-state](./profilectl-state)

Local state inventory. It tracks managed artifacts, backups, rendered files, and
task run metadata under the platform state directory.

## [profilectl-adapters](./profilectl-adapters)

External system adapters. It isolates filesystem, shell, git, package manager,
template, and command-runner effects from the planner.

## [xtask](./xtask)

Development automation. Not published.

| Command                           | What it does                                      |
| --------------------------------- | ------------------------------------------------- |
| `cargo xtask check`               | Format, compile, and lint                         |
| `cargo xtask test [filter]`       | Check and run tests                               |
| `cargo xtask build`               | Test and build release artifacts                  |
| `cargo xtask add <name>`          | Add a library; reserved `cli` adds the project CLI |
| `cargo xtask coverage`            | Generate HTML coverage                            |
| `cargo xtask publish [--execute]` | Prepare a lockstep workspace release              |
