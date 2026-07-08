# profilectl Context

profilectl is a profile-based dotfiles and developer machine automation tool.
The current branch is a greenfield reset from `gh:carlosferreyra/rust-template`.

The product should replace personal zsh/bootstrap scripts first. The longer-term
competitive frame is chezmoi, but the MVP should stay smaller: typed desired
state, explicit plans, tool installation, profile resolution, and safe migration
from existing shell setup.

Previous project examples are preserved under `fixtures/migration/current/` so
future work can mine them for realistic bundles, profiles, shell snippets, and
migration behavior.
