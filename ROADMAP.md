# profilectl Roadmap

## Direction

`profilectl` is restarting from the `carlosferreyra/rust-template` scaffold. The
current implementation can be discarded in favor of a clearer architecture:
profile files describe desired machine state, the core engine builds an explicit
plan, and commands either display, check, or execute that plan.

The first product milestone is replacing personal zsh/bootstrap scripts. Chezmoi
competition is a later pressure: profilectl should differentiate through typed
plans, tool installation, profiles, and an approachable TUI rather than chasing
feature parity immediately.

## Design Principles

- CLI is canonical; the future TUI is a guided frontend over the same engine.
- Profile application is one-way: profile repository to machine.
- `plan`, `status`, `check`, and `apply` all share the same planner.
- Machine changes are typed operations, not hidden shell side effects.
- Migration helpers generate reports or draft profiles, but never auto-apply.
- Previous examples are retained as migration fixtures, not active source.

## Target Command Surface

```sh
profilectl
profilectl init [repo] [--bundles a,b,c] [--force]
profilectl plan   [--scope links|tools|shell|tasks|all]
profilectl apply  [--scope links|tools|shell|tasks|all] [--pull] [--force] [--strict]
profilectl status [--scope links|tools|shell|tasks|all]
profilectl check  [--scope links|tools|shell|tasks|all]
profilectl scan   [--output path]
profilectl profile list
profilectl profile show [name]
profilectl profile use <name>
profilectl publish [url]
profilectl uninstall [--purge]
```

## Architecture Roadmap

### Phase 1: Scaffold Reset

- [x] Generate the project from `gh:carlosferreyra/rust-template`.
- [x] Adopt template defaults: edition 2024, resolver 3, stable toolchain,
  workspace lints, CI, `cargo xtask`, `nextest`, `deny`, and `typos`.
- [x] Preserve old bundles, profiles, config files, and `profilectl.zsh` under
  `fixtures/migration/current/`.
- [x] Scaffold `profilectl-cli`, `profilectl-core`, `profilectl-config`,
  `profilectl-state`, and `profilectl-adapters`.
- [x] Document the greenfield direction in `ROADMAP.md` and `PLAN.md`.

### Phase 2: Read-Only Core

- [ ] Implement TOML profile parsing and validation in `profilectl-config`.
- [ ] Implement bundle and inheritance resolution with deterministic precedence:
  bundles, then inherited profiles, then local profile fields.
- [ ] Define observed machine state and desired-state lowering.
- [ ] Implement plan generation without mutating the machine.
- [ ] Wire `profilectl plan`, `status`, and `check` to the planner.

### Phase 3: File And Shell Apply

- [ ] Track managed artifacts in `profilectl-state`.
- [ ] Implement link operations with conflict reporting and explicit `--force`.
- [ ] Add shell bootstrap management for zsh, bash, and PowerShell.
- [ ] Add minijinja rendering for template files into profilectl's rendered
  output directory.
- [ ] Implement `uninstall` for managed files and shell bootstrap blocks.

### Phase 4: Tooling Apply

- [ ] Add package-manager adapters beginning with `mise` and `brew`.
- [ ] Add Linux and Windows package manager adapters after the interface is
  stable.
- [ ] Make tool checks idempotent and shared by `plan`, `status`, `check`, and
  `apply`.
- [ ] Preserve continue-on-error by default; make `--strict` fail fast.

### Phase 5: Init, Scan, And Publish

- [ ] Implement `init` for local repos, clone URLs, and local paths.
- [ ] Implement `scan` as a migration report/draft generator.
- [ ] Implement `publish` as optional git remote setup.
- [ ] Build the first-run TUI after CLI behavior is stable.

### Phase 6: Chezmoi-Competitive Features

- [ ] Add controlled profile tasks with `always`, `once`, and `changed`
  triggers.
- [ ] Add richer diffs and review output.
- [ ] Add secret-provider integration only after templates and state are stable.
- [ ] Explore remote profile registries after local-first workflows are solid.
