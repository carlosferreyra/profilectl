# profilectl Reset Plan

## Goal

Restart profilectl on the same scaffold as `gh:carlosferreyra/rust-template`
while preserving the product intent: replace current zsh/bootstrap scripts first,
then grow into a credible chezmoi alternative.

## Approved Decisions

- Preserve the old `bundles/`, `profiles/`, `config/`, and `tool_workflow.md`
  material as migration fixtures.
- Preserve `profilectl.zsh` as migration context.
- Scaffold the fuller crate layout immediately.
- Adopt template defaults exactly.
- Stop after verified local changes; do not commit or open a PR in this step.

## Completed Reset Work

- Generated a fresh `profilectl` project from the local clone of
  `carlosferreyra/rust-template`.
- Moved previous project assets into `fixtures/migration/current/`.
- Scaffolded these crates with template tooling:
  `profilectl`, `profilectl-cli`, `profilectl-core`, `profilectl-config`,
  `profilectl-state`, `profilectl-adapters`, and `xtask`.
- Replaced the template onboarding README with project-specific documentation.
- Replaced placeholder command modeling with the planned command surface.
- Added first-pass domain types for desired state, plans, profile declarations,
  state inventory, and adapters.

## Next Implementation Steps

1. Make `profilectl-config` parse real profile TOML from fixture data.
2. Add bundle resolution and inheritance precedence tests.
3. Lower resolved profiles into `profilectl-core::DesiredState`.
4. Add observed machine state and read-only planning.
5. Implement `plan`, `status`, and `check` before any mutating `apply` behavior.

## Verification

Use the template gates:

```sh
cargo xtask check
cargo xtask test
cargo xtask build
```

For the scaffold reset, `cargo xtask check` is the required gate.
