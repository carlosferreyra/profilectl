# Crates

## [dfiles](./dfiles)

The binary entry point. Contains `bin/dfiles.rs` which parses CLI args and dispatches to either
`dfiles-cli` (subcommand mode) or `dfiles-interactive` (no-args TUI mode). This crate has no library
surface — it is purely the thin binary glue.

## [dfiles-cli](./dfiles-cli)

All `clap`-based subcommand definitions and their dispatch logic. Each command lives in
`src/commands/<name>.rs` and exposes a `run(args) -> Result<()>` function. Adding a new subcommand
means adding one file here and one variant to `args::Command` — no changes needed in other crates.

**Subcommands:** `sync` · `install` · `link` · `unlink` · `scan` · `diff` · `check` · `profiles` ·
`status`

## [dfiles-config](./dfiles-config)

Profile schema (`Profile`, `Link`, `ToolSet`) and the loader that reads `profiles/<name>.toml`,
resolves `extends` chains, and merges parent fields into child profiles. This is the source of truth
for what a machine should look like.

## [dfiles-interactive](./dfiles-interactive)

The beginner-friendly interactive TUI, powered by `inquire` (menus) and `indicatif` (spinners and
progress bars). Presents the same feature set as the CLI subcommands but through a guided,
keyboard-driven interface. The `progress` module exports reusable spinner/bar helpers consumed by
`dfiles-cli` commands too.

## [dfiles-types](./dfiles-types)

Shared primitives with no business logic: `Platform` enum (macos/linux/windows), `DfilesError`
(thiserror-based), and any other types needed across multiple crates. Exists to prevent circular
dependencies — every other crate may depend on this one, but this crate depends on nothing internal.
