# profilectl — Tool Workflow

This document is the **source of truth for the user-facing surface** (CLI and
TUI). Behaviour for individual subcommands is implemented in follow-up work;
this file fixes the *shape* so each implementation can land against a stable
contract.

---

## 1. Audience model

- **CLI** is for power users and CI/CD. One-shot, exit-code driven, scriptable.
- **TUI** (ratatui + crossterm) is for beginners and day-to-day local use. It
  composes one or more CLI invocations per screen — the "recipe" model.
- **CLI is canonical.** Every effect the TUI can produce is reachable via the
  CLI. The TUI is a strict subset.

---

## 2. CLI surface

```
profilectl                                          # → TUI

profilectl init [<repo>] [--bundles a,b,c] [--force] [--non-interactive]
profilectl apply         [--scope tools|links|all] [--pull] [--force] [--strict]
profilectl publish       [<url>]
profilectl status        [--scope tools|links|all]
profilectl check         [--scope tools|links|all]
profilectl uninstall     [--purge]
profilectl scan          [--output <path>]
profilectl profile list
profilectl profile show  [<name>]
profilectl profile use   <name>
```

### Globals

| Flag        | Env             | Description                                         |
|-------------|-----------------|-----------------------------------------------------|
| `--profile` | `PCTL_PROFILE`  | Active profile name. Defaults to `default`.         |
| `--home`    | `PCTL_HOME`     | Path to the dotfiles repo. Defaults to `~/.dotfiles`. |
| `--verbose` |                 | Enable debug-level tracing.                         |
| `--dry-run` |                 | Show what would happen without making changes.      |

### Subcommand semantics

#### `init [<repo>]`

| Form                          | Effect                                                            |
|-------------------------------|-------------------------------------------------------------------|
| `init`                        | `git init` in `~/.dotfiles` with bundled starter profiles.        |
| `init <url>`                  | Clone `<url>` into `$PCTL_HOME` (default `~/.dotfiles`).          |
| `init <local-path>`           | Use existing local path as `$PCTL_HOME`.                          |
| `--bundles a,b,c`             | Non-interactive bundle selection. Comma-separated.                |
| `--force`                     | Overwrite an existing dotfiles repo at the target location.       |
| `--non-interactive`           | Skip all prompts; fail if a required answer is missing.           |

#### `apply`

Idempotent. Ensures the shell sourcing block, materializes the rendered tree,
creates symlinks, and installs missing tools.

| Flag                    | Effect                                                            |
|-------------------------|-------------------------------------------------------------------|
| `--scope tools|links|all` | Limit work to one side. Default `all`.                            |
| `--pull`                | Run `git pull --ff-only` in the dotfiles repo first.              |
| `--force`               | Overwrite existing files / symlinks at link targets.              |
| `--strict`              | Fail fast on the first error. Default is continue-on-error.       |

#### `publish [<url>]`

Opt-in remote setup. Detects `gh auth status` to prefill
`https://github.com/<handle>/dotfiles`; prompts when no URL is given.
Runs `git remote add` then `git push -u origin main`. Optional
`gh repo create` if `gh` is on PATH and authed; never required.

#### `status [--scope ...]`

Read-only drift report (folds in what the first PR called `diff`).

#### `check [--scope ...]`

Same logic as `status` but exits **1** on drift. CI gate.

#### `uninstall [--purge]`

Removes the shell sourcing block and all profile-managed symlinks. With
`--purge`, also uninstalls packages the active profile installed.

#### `scan [--output <path>]`

Power-user migration utility. Walks `mise list`, `brew leaves`, `apt list
--installed`, etc. Format inferred from the output extension (`.md`,
`.toml`). Defaults to `./tools.md`. **Never** auto-modifies a profile.

#### `profile { list | show [<name>] | use <name> }`

Noun-verb group for profile management. `use` writes
`~/.config/profilectl/config.toml`; it does not auto-`apply`.

### What got dropped (and why)

| Removed       | Replacement / rationale                                  |
|---------------|----------------------------------------------------------|
| `link`        | `apply --scope=links`                                    |
| `unlink`      | `uninstall` (broader, clearer intent)                    |
| `bootstrap`   | Owned implicitly by `apply` (idempotent shell-block ensure) |
| `diff`        | Folded into `status` (one read-only command, not two)    |
| `install`     | `apply --scope=tools`                                    |
| `--tools-only` / `--links-only` | Single `--scope` enum                       |
| `init --from`  | Repo is the positional argument to `init`               |
| `scan --format` / `scan --path` | Format is inferred from `--output` extension |

---

## 3. TUI surface

### 3.1 Screens

```
First-run (no ~/.config/profilectl/config.toml):
  Wizard auto-launches:
    1. Existing repo? URL / no / local path
    2. (if new) Pick bundles — multi-select checkbox
    3. Preview generated profile TOML
    4. Confirm → init → apply → drop to main menu

Recurring:
  Main Menu (4 entries)
  ├── Apply       → status preview → confirm → apply [--scope ...]
  ├── Status      → status (read-only)
  ├── Profiles    → submenu: list / view / switch / Publish to remote
  └── Exit
```

### 3.2 TUI → CLI recipes

| Screen                            | Recipe                                          |
|-----------------------------------|-------------------------------------------------|
| First-run wizard                  | `init <repo>` then `apply`                      |
| Main → Apply                      | `status` (preview) then `apply [--scope ...]`   |
| Main → Status                     | `status`                                        |
| Profiles → switch `<name>`        | `profile use <name>` then `apply`               |
| Profiles → view `<name>`          | `profile show <name>`                           |
| Profiles → publish to remote      | `publish` (interactive prompts)                 |

### 3.3 Keybindings

| Keys                     | Action                  |
|--------------------------|-------------------------|
| `↑` / `↓` or `j` / `k`   | Move selection          |
| `Enter`                  | Activate selection      |
| `Esc` / `q` / `Ctrl-C`   | Exit / back             |
| `Space`                  | Toggle in checkboxes    |

### 3.4 Not in TUI (CLI-only)

`uninstall`, `check`, `scan`, and all of `--force` / `--purge` / `--strict`.
`publish` lives only inside the Profiles submenu, never in the main menu.

---

## 4. Storage model: local default, opt-in publish

- `init` always starts local. `git init` in `$PCTL_HOME` (default `~/.dotfiles`).
- Sync is **unidirectional**: profile → machine. No reverse capture.
  `scan` is the only escape hatch and never auto-modifies a profile.
- `publish` is the opt-in remote setup, separate verb. Once published,
  `git push` / `git pull` are the user's tools; `apply --pull` is the
  convenience wrapper for the multi-machine recurring case.

We deliberately do **not** require a fork of the `profilectl` tool repo as
user storage. That conflates the tool source with user data, creates merge
conflicts on `Cargo.toml` updates, and breaks air-gapped use.

---

## 5. Bundles

`mise` is used as the universal tool installer registry, so we don't have to
maintain our own. **Pattern B**: `mise` auto-checks (and locks) when any other
bundle is picked; it stays unlocked if the user picks only `mise`.

Bootstrap: when first `apply` needs `mise`, we shell out to
`curl https://mise.run | sh` (with `brew` fallback on macOS). The TUI shows
the command and waits for Enter; the CLI requires `--yes` or an interactive
TTY.

### MVP bundles

| Bundle    | Description                                                                  |
|-----------|------------------------------------------------------------------------------|
| `mise`    | polyglot version & tool manager (replaces nvm, pyenv, asdf)                  |
| `uv`      | high-performance tooling for Python development (uv, ruff, mypy, pytest)     |
| `rustup`  | full Rust toolchain — rustup + cargo + clippy + rustfmt, plus nextest, bacon, sccache |
| `bun`     | high-performance runtime & tooling for JavaScript / TypeScript development   |
| `go`      | Go toolchain and common dev helpers (go, golangci-lint, air)                 |
| `docker`  | container runtime (Docker / Docker Desktop, docker-compose)                  |
| `zsh`     | modern shell stack (zsh, starship, zoxide, fzf, eza)                         |
| `git`     | git workflow upgrades (git-lfs, lazygit, delta)                              |
| `vscode`  | VS Code CLI integrations (`code` on PATH, settings sync wiring)              |

Bundle TOML fragments are baked into the binary at `bundles/<name>.toml`.
Loader merge order: **bundles → extends parent → own definition**.

---

## 6. Schema

```rust
pub struct Profile {
    pub name: String,
    pub extends: Option<String>,
    pub bundles: Vec<String>,
    pub description: Option<String>,
    pub links: Vec<Link>,
    pub tools: ToolSet,
    pub env: HashMap<String, String>,
    pub platforms: Vec<Platform>,
}

pub struct ToolSet {
    pub mise:   Vec<String>,    // cross-platform default
    pub brew:   Vec<String>,    // macOS
    pub apt:    Vec<String>,    // Debian / Ubuntu
    pub dnf:    Vec<String>,    // Fedora / RHEL
    pub pacman: Vec<String>,    // Arch
    pub winget: Vec<String>,    // Windows (official)
    pub choco:  Vec<String>,    // Windows (community)
    pub scoop:  Vec<String>,    // Windows (CLI-focused)
    pub other:  HashMap<String, Vec<String>>,
}
```

Per-OS lists are silently skipped on non-matching platforms (filter at apply time).

---

## 7. Idempotency contract

Every apply checks each tool individually before invoking the package manager.

- **Default check**: PATH lookup via `which::which(<tool>)` — no subprocess,
  cross-platform.
- **Curated binary-name lookup**: baked into the binary for the few cases
  where package name ≠ binary (`ripgrep` → `rg`, `fd-find` → `fd`,
  `the-silver-searcher` → `ag`). Bundle TOMLs stay as plain `Vec<String>`.
- The same mechanism powers `check` and `status`: drift is "tools declared
  but not present."
- **Output**: per-tool status lines (`✓ already installed`,
  `↓ installing via mise…`, `✗ failed`), with a summary at the end
  (`3 installed, 5 already present, 0 failed`).

### Failure policy

Continue-on-error by default. Install everything we can, report failures at
the end, exit 1 if any failed. `--strict` flips to fail-fast for CI users
who want immediate abort.

---

## 8. Rules summary (one-page contract)

1. CLI is canonical; TUI composes CLI invocations.
2. `apply` is idempotent and the only state-mutating verb in the TUI main menu.
3. Read-only verbs (`status`, `profile show`, `profile list`) never mutate.
4. `check` exits 1 on drift; `status` always exits 0.
5. `uninstall` only removes what profilectl owns unless `--purge`.
6. `scan` is read-only and never auto-modifies a profile.
7. Sync is unidirectional: profile → machine.
8. Bundles → extends → own values, with own values winning on conflict.
9. Per-OS tool lists are filtered at apply time, never at parse time.
10. Destructive flags (`--force`, `--purge`, `--strict`) are CLI-only.
