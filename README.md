# profilectl

Cross-platform dotfiles automation for macOS, Linux, and Windows, powered by Rust.

Manage your development environment through profiles — declarative TOML files that describe your
symlinks, tools, and environment per machine. Works as a direct CLI or an interactive TUI.

## Quick Start

### Install via cargo

```bash
cargo install profilectl
```

### Install via shell script (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/carlosferreyra/profilectl/releases/latest/download/profilectl-installer.sh | sh
```

### Install via PowerShell (Windows)

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/carlosferreyra/profilectl/releases/latest/download/profilectl-installer.ps1 | iex"
```

## Usage

### Interactive mode (no arguments)

```bash
profilectl
```

Launches a guided TUI to select a profile, preview changes, and apply them.

### CLI subcommands

```
profilectl init [<repo>] [--bundles a,b,c] [--force] [--non-interactive]
profilectl apply        [--scope tools|links|all] [--pull] [--force] [--strict]
profilectl publish      [<url>]
profilectl status       [--scope tools|links|all]
profilectl check        [--scope tools|links|all]
profilectl uninstall    [--purge]
profilectl scan         [--output <path>]
profilectl profile list
profilectl profile show [<name>]
profilectl profile use  <name>
```

### Global flags

| Flag        | Env            | Description                                          |
|-------------|----------------|------------------------------------------------------|
| `--profile` | `PCTL_PROFILE` | Active profile name. Defaults to `default`.          |
| `--home`    | `PCTL_HOME`    | Path to the dotfiles repo. Defaults to `~/.dotfiles`.|
| `--verbose` |                | Enable debug-level tracing.                          |
| `--dry-run` |                | Show what would happen without making changes.       |

### Selecting a profile

```bash
profilectl --profile work apply
```

Or set the environment variable:

```bash
export PCTL_PROFILE=work
profilectl apply
```

## Profiles

Profiles live in `profiles/<name>.toml` and describe what a machine should look like:

```toml
name = "default"
bundles = ["zsh", "git"]

[[links]]
src = "config/zsh/.zshrc"
dst = "~/.zshrc"

[[links]]
src = "config/git/.gitconfig"
dst = "~/.gitconfig"

[tools]
mise = ["ripgrep", "bat"]
brew = ["eza"]
```

Profiles support inheritance via `extends` and pre-built `bundles`:

```toml
name = "work"
extends = "default"
bundles = ["docker", "go"]

[[links]]
src = "config/zsh/.zshrc.work"
dst = "~/.zshrc"

[tools]
brew = ["awscli", "terraform"]
```

The `work` profile inherits all links and tools from `default`, then adds or overrides its own.
Merge order: **bundles → extends parent → own definition**. Own values win on conflict.

### Bundles

Nine pre-built bundles are embedded in the binary:

| Bundle   | Description                                                  |
|----------|--------------------------------------------------------------|
| `mise`   | Polyglot version & tool manager (replaces nvm, pyenv, asdf) |
| `uv`     | Python tooling (uv, ruff, mypy, pytest)                      |
| `rustup` | Full Rust toolchain + nextest, bacon, sccache                |
| `bun`    | JavaScript / TypeScript runtime and tooling                  |
| `go`     | Go toolchain + golangci-lint, air                            |
| `docker` | Container runtime (Docker Desktop, docker-compose)           |
| `zsh`    | Modern shell stack (zsh, starship, zoxide, fzf, eza)         |
| `git`    | Git workflow upgrades (git-lfs, lazygit, delta)              |
| `vscode` | VS Code CLI integrations                                     |

## Project Structure

```
profilectl/
├── bundles/               # Embedded bundle TOML fragments (mise, git, zsh, …)
├── crates/
│   ├── profilectl/            # Binary entry point (thin glue)
│   ├── profilectl-cli/        # Clap subcommands and dispatch
│   ├── profilectl-config/     # Profile schema, loader, bundle resolver
│   ├── profilectl-interactive/ # ratatui + crossterm TUI
│   └── profilectl-types/      # Shared types (Platform, MachineInfo, ProfilectlError)
├── scripts/
│   └── release_pypi.py    # PyPI thin-wrapper publish script
└── Cargo.toml             # Workspace root
```

See [crates/README.md](crates/README.md) for a description of each crate.

## Release Pipeline

Releases are fully automated:

1. `cargo release patch --execute` on `main`
2. git-cliff generates `CHANGELOG.md`, cargo-release commits and tags
3. cargo-dist builds binaries for macOS (aarch64 + x86_64), Linux (x86_64), and Windows (x86_64)
4. GitHub Release is created with shell/PowerShell installers
5. PyPI thin-wrapper package is published via OIDC trusted publishing

## Contributing

1. Fork the repository
2. Create a feature branch from `main`
3. Open a pull request — CI runs `cargo fmt`, `cargo clippy`, and `cargo test`

## License

MIT — see [LICENSE](LICENSE).

---

**Made with ❤️ by [Carlos Ferreyra](https://github.com/carlosferreyra)**
