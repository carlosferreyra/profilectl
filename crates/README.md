# Crates

## [profilectl](./profilectl)

The binary entry point. Contains `bin/profilectl.rs` which parses CLI args and dispatches to either
`profilectl-cli` (subcommand mode) or `profilectl-interactive` (no-args TUI mode). This crate has no library
surface — it is purely the thin binary glue.

## [profilectl-cli](./profilectl-cli)

All `clap`-based subcommand definitions and their dispatch logic. Each command lives in
`src/commands/<name>.rs` and exposes a `run(args) -> Result<()>` function. Adding a new subcommand
means adding one file here and one variant to `args::Command` — no changes needed in other crates.

**Subcommands:** `sync` · `install` · `link` · `unlink` · `scan` · `diff` · `check` · `profiles` ·
`status`

## [profilectl-config](./profilectl-config)

Profile schema (`Profile`, `Link`, `ToolSet`) and the loader that reads `profiles/<name>.toml`,
resolves `extends` chains, and merges parent fields into child profiles. This is the source of truth
for what a machine should look like.

## [profilectl-interactive](./profilectl-interactive)

The beginner-friendly interactive TUI, powered by `inquire` (menus) and `indicatif` (spinners and
progress bars). Presents the same feature set as the CLI subcommands but through a guided,
keyboard-driven interface. The `progress` module exports reusable spinner/bar helpers consumed by
`profilectl-cli` commands too.

## [profilectl-types](./profilectl-types)

Shared primitives with no business logic: `Platform` enum (macos/linux/windows), `ProfilectlError`
(thiserror-based), and any other types needed across multiple crates. Exists to prevent circular
dependencies — every other crate may depend on this one, but this crate depends on nothing internal.
