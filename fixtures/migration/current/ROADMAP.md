# profilectl Roadmap

## Design Decisions

### Workspace structure

Cargo workspace under `crates/` with a consistent `profilectl-` prefix, mirroring the uv repo layout:

- `crates/profilectl` — binary entry point
- `crates/profilectl-cli` — argument parser + command dispatch
- `crates/profilectl-config` — profile loader, schema, bundle resolver
- `crates/profilectl-interactive` — ratatui TUI
- `crates/profilectl-types` — shared types (`ProfilectlError`, `Platform`)

### Global config location

`~/.config/profilectl/config.toml` — XDG convention. Absence = first-time setup. Presence = normal mode.

### `profilectl` (no args) → TUI main menu

- 4 entries: **Apply**, **Status**, **Profiles**, **Exit**
- Backed by `ratatui` + `crossterm` (replaced `inquire`)
- Destructive/CI-only verbs (`uninstall`, `check`, `scan`, `publish`, `init`) are CLI-only and deliberately absent from the main menu
- TUI is a strict subset of the CLI — every effect is reachable via a CLI invocation

See `tool_workflow.md` for the full TUI→CLI recipe table.

### CLI surface

```sh
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

### Profile schema

```rust
pub struct Profile {
    pub name: String,
    pub extends: Option<String>,   // inherit from another profile
    pub bundles: Vec<String>,      // baked-in TOML fragments, merged first
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

Merge order: **bundles → extends parent → own definition**. Own values win on conflict.
Per-OS tool lists are filtered at apply time, never at parse time.

### Bundles

9 MVP fragments under `bundles/`, to be embedded in the binary via `include_str!`:
`mise`, `uv`, `rustup`, `bun`, `go`, `docker`, `zsh`, `git`, `vscode`.

`mise` is the universal tool installer; per-OS lists handle everything else.

### Storage model

- `init` always starts local (`git init` in `$PCTL_HOME`, default `~/.dotfiles`)
- Sync is **unidirectional**: profile → machine. No reverse capture.
- `scan` is the only escape hatch and never auto-modifies a profile.
- `publish` is the opt-in remote setup verb. Once published, `apply --pull` is the convenience wrapper for multi-machine use.

### Shell config sourcing

profilectl appends a one-time bootstrap block to the user's shell config on first `apply`:

```zsh
# --- profilectl managed (do not edit) ---
for f in ~/.config/profilectl/rendered/*.zsh; do
  [ -r "$f" ] && source "$f"
done
# --- end profilectl ---
```

Shell config targets: macOS → `~/.zshrc`, Linux → `~/.bashrc`, Windows → `~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1`.

### Templating

- Engine: `minijinja` (Jinja2 syntax), opt-in via `.j2` extension
- Rendered output at `~/.config/profilectl/rendered/`
- Static files → symlink from `rendered/` to repo source (edits in repo are instant)
- Templated files → rendered copy, re-rendered on `apply`

### Env vars (`PCTL_` prefix)

- `PCTL_HOME` — path to dotfiles repo (default `~/.dotfiles`)
- `PCTL_PROFILE` — active profile name (default `default`)

### Idempotency

Every `apply` checks each tool individually via `which::which` before invoking the package manager. Continue-on-error by default; `--strict` flips to fail-fast for CI.

---

## Implementation Plan

### Phase 1 — Foundations ✅

- [x] Workspace migrated to `crates/*` layout
- [x] `PCTL_HOME` / `PCTL_PROFILE` env vars wired through `args.rs` + `loader.rs`
- [x] `GlobalConfig` struct — reads/writes `~/.config/profilectl/config.toml`
- [x] `minijinja` in workspace deps
- [x] `ProfilectlError` type in `profilectl-types`

### Phase 2 — CLI/TUI surface ✅

- [x] Full CLI argument parser (`clap`) matching `tool_workflow.md`
- [x] Legacy verbs removed (`sync`, `link`, `unlink`, `install`, `diff`, `bootstrap`)
- [x] TUI rewritten with `ratatui` + `crossterm` (4-entry main menu)
- [x] `tool_workflow.md` added as source of truth
- [x] 9 bundle TOML fragments under `bundles/`
- [x] Profile schema updated (`ToolSet`, `bundles`, `extends`, `platforms`)
- [x] All command files are stubs with clear `not yet implemented` messages

### Phase 3 — `init`

- [ ] Auto-detect OS, arch, package managers present on PATH
- [ ] TUI first-run wizard: repo → bundle selection → preview → confirm → write config
- [ ] `git init` / `git clone` into `$PCTL_HOME`
- [ ] Write `~/.config/profilectl/config.toml`

### Phase 4 — `apply`

- [ ] Shell sourcing block ensure (idempotent)
- [ ] Symlink materialization from `links`
- [ ] Tool installation via `mise` + per-OS managers
- [ ] `--scope tools|links|all`, `--pull`, `--force`, `--strict`

### Phase 5 — `status` and `check`

- [ ] Drift report: declared tools vs installed, declared links vs present
- [ ] `status` — always exits 0, human-readable output
- [ ] `check` — exits 1 on drift, CI gate

### Phase 6 — Bundle embedding

- [ ] Embed `bundles/*.toml` into the binary via `include_str!`
- [ ] Wire `resolve_bundle()` in `profilectl-config`

### Phase 7 — `profile` group

- [ ] `profile list` — list available profiles in `$PCTL_HOME/profiles/`
- [ ] `profile show [<name>]` — print resolved profile TOML
- [ ] `profile use <name>` — write active profile to global config

### Phase 8 — `publish`, `uninstall`, `scan`

- [ ] `publish [<url>]` — `git remote add` + `git push -u`, optional `gh repo create`
- [ ] `uninstall [--purge]` — remove sourcing block and managed symlinks
- [ ] `scan [--output <path>]` — walk installed tools, write manifest (`.md` or `.toml`)

### Phase 9 — Templating

- [ ] `minijinja` rendering pipeline for `.j2` files
- [ ] Rendered output at `~/.config/profilectl/rendered/`
- [ ] Template context: `machine.*`, `profile.*`, `env.*`

### Phase 10 — Remote profile registry

- [ ] Fetch pre-built profiles from remote, versioned per profilectl release
