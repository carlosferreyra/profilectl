# profilectl Roadmap

## Design Decisions

### Global config location
`~/.config/profilectl/config.toml` — XDG convention.
Absence of this file = first-time setup. Presence = normal/edit mode.

### `profilectl` (no args) → TUI main menu
- Welcome screen
- Shows current profile + status if initialized
- Options: Init, Sync, Link, Diff, Check, Status, Exit
- Uses inquire (menus) + indicatif (spinners/progress)

### `profilectl init` → first-time wizard
1. Auto-detect: OS, arch, available package managers (brew, cargo, uv, mise, npm, bun, pip...)
2. Remote repo: GitHub default, manual input for GitLab/Bitbucket/self-hosted
3. Profile stack selection: pick from bundled pre-built profiles (rust, bun, uv, c, cpp, node...) that stack via `extends`
4. Custom profile: hybrid — detected tools pre-checked + manual input to add more
5. Write `~/.config/profilectl/config.toml`
6. Preview what will be applied → confirm → apply

### Pre-built profiles
- **MVP:** bundled in the binary as embedded TOML (works offline, no external deps)
- **Future:** fetched from remote registry, versioned per profilectl release
- Profile library: rust, uv, bun, node, c, cpp, python, go — stacked via `extends`

### Multi-OS handling
- OS variation at **profile selection time**, not inside files
- `platforms = ["macos"]` field in profile schema already exists
- Separate profiles with `extends` is the default answer (not template branches inside files)
- minijinja as opt-in for small per-file variations (`.j2` extension)

### Templating
- Engine: `minijinja` (Jinja2 syntax)
- Opt-in: only files with `.j2` extension are rendered
- Rendered output cached to `~/.config/profilectl/rendered/`, then symlinked
- Template context: `profile.name`, `machine.hostname`, `machine.platform`, `env.*`, `profile.env.*`

### VSCode detection
- Detect `$TERM_PROGRAM == "vscode"` or `$VSCODE_INJECTION`
- If detected: enable `code -r` integrations (open files in editor)

### Env vars (`PCTL_` prefix, like `UV_` style)
- `PCTL_HOME` — path to dotfiles repo
- `PCTL_PROFILE` — active profile name

---

## Implementation Plan

### Phase 1 — Foundations
- [ ] Fix env vars: `DFILES_HOME/PROFILE` → `PCTL_HOME/PCTL_PROFILE` in `args.rs` + `loader.rs`
- [ ] Add `GlobalConfig` struct — reads/writes `~/.config/profilectl/config.toml`, detects first-run
- [ ] Add `minijinja` to workspace deps

### Phase 2 — `profilectl init`
- [ ] Auto-detect OS, arch, package managers present on PATH
- [ ] Bundled pre-built profiles (embedded TOML)
- [ ] TUI wizard: repo → profile stack → custom tools → preview → confirm → write config

### Phase 3 — TUI main menu
- [ ] Main menu (inquire) with status summary
- [ ] Routes to subcommand implementations as they land

### Phase 4 — First subcommands
- [ ] `link` — create symlinks from profile
- [ ] `unlink` — remove managed symlinks
- [ ] `profiles` — list available profiles
- [ ] `status` — show current profile and machine state

### Phase 5 — Sync and verification
- [ ] `check` — verify symlinks and tools
- [ ] `diff` — compare profile vs installed state
- [ ] `sync` — link + install in sequence

### Phase 6 — Tool management
- [ ] `install` — invoke brew/cargo/uv/npm/bun per profile tools
- [ ] `scan` — detect installed tools, compare against profile

### Phase 7 — Templating
- [ ] minijinja rendering pipeline for `.j2` files
- [ ] Rendered output cache at `~/.config/profilectl/rendered/`
- [ ] Template context: machine, profile, env

### Phase 8 — Remote profiles registry
- [ ] Fetch pre-built profiles from remote, versioned per profilectl release
- [ ] `profilectl update-profiles` or equivalent
