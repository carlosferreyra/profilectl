# profilectl Tool Workflow — Pilot Proposal

Status: **draft / pilot** — this is the source of truth that the CLI argument
parser (`crates/profilectl-cli/src/args.rs`) and the interactive TUI menu
(`crates/profilectl-interactive/src/menu.rs`) should converge on. The current
implementations are stubs; this document fixes the surface so we can land
behaviour against a stable shape.

The goals, in order:

1. Define the **CLI tree** — every subcommand, every flag, every exit code.
2. Define the **interactive (TUI) tree** — every screen, every keybind, and
   the CLI invocation(s) it composes ("recipes").
3. Spell out the **rules** that bind the two: which is canonical, what may
   diverge, and what must not.

If a behaviour is missing here, it is out of scope for the pilot. Add it to
this file first, then implement.

---

## 1. Audience and modes

| Mode | Audience | Trigger | Interaction model |
|---|---|---|---|
| **CLI** | advanced users, scripts, CI/CD | `profilectl <subcommand> [flags]` | one-shot, exit code-driven, no prompts unless `--interactive` is set |
| **TUI** | beginners, exploratory users | `profilectl` (no subcommand) | menu-driven, ratatui + crossterm, composes CLI subcommands under the hood |

### Binding rules

- The **CLI is canonical**. Every effect the TUI has on the system is reachable
  through one or more CLI invocations.
- The **TUI is a strict subset of CLI capability**. A TUI screen may abstract
  one or more CLI subcommands into a single guided flow ("recipe"), but it may
  not perform an effect that no CLI subcommand could.
- The **CLI may have surface outside the TUI** (advanced or destructive
  commands like `unlink`, `scan`, `sync --tools-only`). The TUI omits these
  intentionally.
- The TUI never prompts for anything that cannot be expressed as flags on the
  underlying CLI invocation. If a TUI screen needs a new input, the
  corresponding CLI subcommand grows a flag first.
- Global flags (`--profile`, `--home`, `--verbose`, `--dry-run`) apply to every
  CLI invocation the TUI emits, taking values from TUI state.

---

## 2. CLI tree

```
profilectl [GLOBAL FLAGS] [SUBCOMMAND]
│
├── (no subcommand)             → launches TUI (see §3)
│
├── init                        first-time setup wizard / write global config
│   ├── --force                 overwrite existing ~/.config/profilectl/config.toml
│   ├── --non-interactive       no prompts; fail if required input is missing
│   └── --from <path|url>       clone/copy a dotfiles repo as the new home
│
├── sync                        link + install in one go
│   ├── --tools-only            install tools, skip symlinks
│   ├── --links-only            create symlinks, skip tool install
│   └── --force                 overwrite existing files when linking
│
├── link                        create or refresh dotfile symlinks only
│   └── --force                 overwrite non-symlink targets
│
├── unlink                      remove all symlinks managed by active profile
│   └── --force                 remove targets even if they are not symlinks
│
├── bootstrap                   shell sourcing + ~/.config/profilectl/rendered/
│   ├── --shell <zsh|bash|fish|pwsh>   target shell (default: detected)
│   └── --remove                strip the bootstrap block we previously wrote
│
├── scan                        write a tools.md snapshot of installed tools
│   ├── --path <path>           output file (default: ./tools.md)
│   └── --format <md|toml>      output format (default: md)
│
├── diff                        show drift between profile and machine
│   ├── --tools-only            only diff tools
│   └── --links-only            only diff symlinks
│
├── check                       exit nonzero if profile is not fully applied
│   ├── --tools-only            only check tools
│   └── --links-only            only check symlinks
│
├── status                      human-readable summary of profile + machine
│
└── profile                     noun-verb group for managing profiles
    ├── list                    list available profiles in the home repo
    ├── show [<name>]           print effective (post-extends) profile as TOML
    └── use <name>              persist <name> as the active profile in global config
```

### 2.1 Global flags

| Flag | Type | Default | Env | Description |
|---|---|---|---|---|
| `--profile, -p <name>` | string | `default` | `PCTL_PROFILE` | Profile to operate on. |
| `--home <path>` | path | autodetect | `PCTL_HOME` | Path to the dotfiles repo. |
| `--verbose, -v` | flag | off | — | Debug-level tracing output. |
| `--dry-run` | flag | off | — | Print the effects that would happen, take none. |

`--dry-run` applies to every mutating subcommand. Read-only subcommands
(`status`, `diff`, `check`, `scan`, `profile list`, `profile show`) ignore it.

### 2.2 Per-subcommand reference

#### `profilectl init`

Writes `~/.config/profilectl/config.toml` and (optionally) clones a dotfiles
repo. Designed to be the entry point for new machines. Works headless via
`--non-interactive`.

| Flag | Default | Description |
|---|---|---|
| `--force` | false | Overwrite an existing global config. |
| `--non-interactive` | false | Refuse to prompt; fail if any required field is missing. |
| `--from <path-or-url>` | — | Clone (URL) or symlink (local path) a dotfiles repo into the chosen home. |

Exit codes: `0` ok, `1` already initialized (use `--force`), `2` missing input
in `--non-interactive` mode.

Mutates: `~/.config/profilectl/config.toml`, optionally creates `$PCTL_HOME`.

#### `profilectl sync`

Apply the active profile end-to-end: create symlinks, then install tools.
This is the headline command.

| Flag | Default | Description |
|---|---|---|
| `--tools-only` | false | Skip the link phase. Mutually exclusive with `--links-only`. |
| `--links-only` | false | Skip the install phase. Mutually exclusive with `--tools-only`. |
| `--force` | false | Pass `--force` through to the link phase. Ignored under `--tools-only`. |

Exit codes: `0` ok, `1` partial failure (one phase succeeded, one failed),
`2` invalid flag combination.

Mutates: filesystem (symlinks), package manager state (brew/cargo/uv/npm/bun).

#### `profilectl link`

Create or refresh dotfile symlinks for the active profile. Does not touch
tools.

| Flag | Default | Description |
|---|---|---|
| `--force` | false | Overwrite existing files at link targets. |

Exit codes: `0` ok, `1` one or more targets failed.

Mutates: filesystem (symlinks).

#### `profilectl unlink`

Remove every symlink the active profile manages. **Destructive**, deliberately
absent from the TUI top level.

| Flag | Default | Description |
|---|---|---|
| `--force` | false | Remove targets even if they are not symlinks (i.e. user replaced them). |

Exit codes: `0` ok, `1` one or more targets failed.

Mutates: filesystem.

#### `profilectl bootstrap`

Owns the shell-sourcing concern described in `ROADMAP.md` (§ "Shell config
sourcing"). Writes the managed bootstrap block into the user's shell rc and
materializes `~/.config/profilectl/rendered/`.

| Flag | Default | Description |
|---|---|---|
| `--shell <zsh\|bash\|fish\|pwsh>` | autodetect | Target shell rc to modify. |
| `--remove` | false | Strip the previously inserted block. Inverse of the default action. |

Idempotent: re-running without flags is a no-op when the block is already
present.

Exit codes: `0` ok, `1` could not write rc.

Mutates: shell rc file, `~/.config/profilectl/rendered/`.

#### `profilectl scan`

Inspect the machine, list every detected tool grouped by package manager,
and write a markdown (default) or TOML snapshot. Useful for seeding new
profiles.

| Flag | Default | Description |
|---|---|---|
| `--path <path>` | `./tools.md` | Output file. |
| `--format <md\|toml>` | `md` | Output format. |

Exit codes: `0` ok, `1` could not write output.

Read-only with respect to the dotfiles repo and machine state.

#### `profilectl diff`

Show drift between the effective profile and the current machine. Read-only.

| Flag | Default | Description |
|---|---|---|
| `--tools-only` | false | Only diff tools. |
| `--links-only` | false | Only diff symlinks. |

Exit codes: `0` always, regardless of drift. (Use `check` for gating.)

#### `profilectl check`

Verify the active profile is fully applied. Designed for CI gates.

| Flag | Default | Description |
|---|---|---|
| `--tools-only` | false | Only check tools. |
| `--links-only` | false | Only check symlinks. |

Exit codes: `0` profile fully applied, `1` drift detected.

#### `profilectl status`

Human-readable report: active profile, home path, OS / arch, package managers
present, last sync time (if recorded). Does not exit nonzero on drift.

No flags.

#### `profilectl profile list`

Print every profile available in `$PCTL_HOME/profiles/`.

#### `profilectl profile show [<name>]`

Print the effective profile (after `extends` resolution) as TOML. Defaults to
the active profile.

#### `profilectl profile use <name>`

Persist `<name>` as the active profile in `~/.config/profilectl/config.toml`.
Equivalent to setting `PCTL_PROFILE=<name>` permanently.

---

## 3. Interactive (TUI) tree

The TUI is launched by `profilectl` with no subcommand. It always opens at the
**Main Menu**. Every screen has a single canonical exit (`q` / `Esc`) that
returns to the main menu, plus a global hard-quit (`Ctrl-C`).

```
TUI
│
├── Main Menu                  (entry; shows active profile + first-run banner)
│   │
│   ├── Init                   → wizard (only highlighted when first-run is detected)
│   │   ├── 1. detect home repo (or prompt to clone)
│   │   ├── 2. pick profile from listed/bundled
│   │   ├── 3. preview effective profile
│   │   └── 4. confirm → runs `init` then `bootstrap`
│   │
│   ├── Sync                   → guided sync
│   │   ├── pick scope: all | links-only | tools-only
│   │   ├── preview (`diff`)
│   │   └── confirm → runs `sync [--scope-flag]`
│   │
│   ├── Link                   → just symlinks
│   │   ├── preview (`diff --links-only`)
│   │   └── confirm → runs `link`
│   │
│   ├── Bootstrap              → shell sourcing
│   │   ├── pick shell (autodetected default)
│   │   └── confirm → runs `bootstrap --shell <picked>`
│   │
│   ├── Status                 → inspect-only, no confirmation
│   │   └── runs `status`, then `check`, then `diff` (sequenced; results stacked)
│   │
│   ├── Profiles               → noun-verb deep menu
│   │   ├── list               → runs `profile list`
│   │   ├── show <name>        → runs `profile show <name>`
│   │   └── switch <name>      → runs `profile use <name>` (refreshes active profile in TUI state)
│   │
│   └── Exit                   → leave TUI (graceful)
│
└── (Ctrl-C, anywhere)         → leave TUI immediately
```

### 3.1 Keybindings (every screen)

| Key | Action |
|---|---|
| `↑` / `k` | move selection up |
| `↓` / `j` | move selection down |
| `Enter` | activate / confirm |
| `Esc` / `q` | back to previous screen (or exit from main menu) |
| `Ctrl-C` | hard-quit |

### 3.2 Recipes — what each TUI screen runs

Each leaf in the TUI tree maps to one or more CLI invocations. This is the
contract: if the recipe diverges from these invocations, fix the TUI, not the
CLI.

| TUI screen | Recipe (CLI invocations, in order) |
|---|---|
| Main Menu | none — pure rendering |
| Init | `init [--from <user-input>]` then `bootstrap` |
| Sync (scope = all) | `diff` (preview), then `sync` |
| Sync (scope = links-only) | `diff --links-only`, then `sync --links-only` |
| Sync (scope = tools-only) | `diff --tools-only`, then `sync --tools-only` |
| Link | `diff --links-only`, then `link` |
| Bootstrap | `bootstrap --shell <picked>` |
| Status | `status`, then `check`, then `diff` |
| Profiles → list | `profile list` |
| Profiles → show `<name>` | `profile show <name>` |
| Profiles → switch `<name>` | `profile use <name>` (TUI then re-reads active profile) |
| Exit | none |

### 3.3 What the TUI deliberately omits

The following CLI surface is **not reachable from the TUI**, by design:

- `unlink` — destructive, requires explicit CLI use.
- `scan` — utility for power users seeding new profiles.
- `sync --force`, `link --force`, `unlink --force` — guard against accidents.
- `init --non-interactive` — meaningless inside an interactive frontend.
- `bootstrap --remove` — undo path, advanced.
- Global `--dry-run` — the TUI always previews via `diff` before mutating.

Beginners should not need any of these. Power users can drop into the CLI.

---

## 4. Rules summary (one-page contract)

1. **Canonical surface lives in `args.rs`.** Subcommand names, flag names,
   defaults, and exit codes are sourced from the clap definitions.
2. **TUI dispatches CLI commands.** No TUI screen reaches into business logic
   directly; every effect goes through `profilectl_cli::commands::*::run`.
3. **TUI inputs map to CLI flags one-for-one.** A TUI prompt that cannot be
   expressed as flags is a missing CLI flag — add the flag first.
4. **Read-only screens never mutate.** `Status` and `Profiles → show` must not
   write. They are pure recipes over read-only subcommands.
5. **Mutating screens preview first.** Any TUI screen that runs a mutating
   recipe runs the corresponding `diff` (or its `--*-only` variant) first.
6. **Destructive flags are CLI-only.** `--force`, `unlink`, `bootstrap
   --remove`, `init --force` are not exposed in the TUI menu.
7. **`--dry-run` is universal on the CLI.** Every mutating subcommand honors
   it, including the ones the TUI itself emits when the user passes the flag
   on the parent invocation.
8. **Exit codes are stable.** `check` is the gate for CI: 0 means clean, 1
   means drift. No other subcommand reuses these semantics.
9. **`PCTL_HOME` and `PCTL_PROFILE` are the only env vars.** Anything else is
   a bug.
10. **This file is the source of truth for surface.** Code that disagrees with
    this file is the side that needs changing.

---

## 5. Migration from current state

This is what the pilot PR changes versus `main`:

| Subcommand | Before | After |
|---|---|---|
| `install` | top-level subcommand stub | **removed**; subsumed by `sync --tools-only` |
| `profiles` | top-level subcommand stub | **removed**; replaced by `profile list/show/use` |
| `init` | TUI-only stub | promoted to top-level CLI subcommand |
| `bootstrap` | not present | new top-level CLI subcommand |
| `sync` | no flags | gains `--tools-only`, `--links-only`, `--force` |
| `diff`, `check` | no flags | gain `--tools-only`, `--links-only` |
| `scan` | `--path` only | gains `--format md\|toml` |

TUI menu before: `init, sync, link, diff, check, status, exit`.
TUI menu after: `init, sync, link, bootstrap, status, profiles, exit`.

Behaviour remains stubbed — the pilot fixes the **shape**, not the logic.

---

## 6. Open questions (track here, resolve in follow-ups)

- Should `profile use <name>` accept `--no-sync`, or is it always purely a
  config write? Default proposed: pure config write, user runs `sync` next.
- Should `bootstrap` auto-detect WSL and treat it as Linux + Windows shells,
  or pick one? Default proposed: pick the shell from `$SHELL`, user can
  override with `--shell`.
- Should `init` and `bootstrap` be fused into one wizard flow in the TUI, or
  remain two separately picker-able actions? Currently fused inside Init,
  separately reachable as Bootstrap.
- `scan` output format: should TOML emit profile-shaped TOML (drop-in
  ready) or a flat manifest? Default proposed: profile-shaped, so the user
  can `cp tools.toml profiles/<name>.toml` and edit.
