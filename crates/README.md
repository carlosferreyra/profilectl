# Crates

## [profilectl](./profilectl)

The binary entry point. Contains `bin/profilectl.rs` which parses CLI args and dispatches to either
`profilectl-cli` (subcommand mode) or `profilectl-interactive` (no-args TUI mode). This crate has no
library surface — it is purely the thin binary glue.

## [profilectl-cli](./profilectl-cli)

All `clap`-based subcommand definitions and their dispatch logic. Each command lives in
`src/commands/<name>.rs` and exposes a `run(args) -> Result<()>` function. Adding a new subcommand
means adding one file here and one variant to `args::Command` — no changes needed in other crates.

**Subcommands:** `init` · `apply` · `publish` · `status` · `check` · `uninstall` · `scan` · `profile list` · `profile show` · `profile use`

## [profilectl-config](./profilectl-config)

Profile schema (`Profile`, `Link`, `ToolSet`) and the loader that reads `profiles/<name>.toml`,
resolves `extends` chains, and merges parent fields into child profiles. This is the source of truth
for what a machine should look like.

## [profilectl-interactive](./profilectl-interactive)

The beginner-friendly interactive TUI, powered by `ratatui` + `crossterm`. On first run (no
`~/.config/profilectl/config.toml`) it launches a setup wizard; afterwards it shows a 4-entry
main menu: **Apply**, **Status**, **Profiles**, **Exit**. The TUI is a strict subset of the CLI —
every effect it produces is reachable via a CLI invocation.

## [profilectl-types](./profilectl-types)

Shared primitives with no business logic: `Platform` enum (macos/linux/windows), `MachineInfo`
(runtime OS/arch/package-manager detection), `PackageManager` enum, and `ProfilectlError`
(thiserror-based). Exists to prevent circular dependencies — every other crate may depend on this
one, but this crate depends on nothing internal.
