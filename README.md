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
profilectl sync       Apply the active profile (links + tools)
profilectl install    Install declared tools via the appropriate package manager
profilectl link       Create symlinks defined in the profile
profilectl unlink     Remove managed symlinks
profilectl scan       Detect installed tools and compare against the profile
profilectl diff       Show what would change if the profile were applied
profilectl check      Verify all symlinks and tools are in the expected state
profilectl profiles   List available profiles
profilectl status     Show current profile and system state
```

### Selecting a profile

```bash
profilectl --profile work sync
```

Or set the environment variable:

```bash
export PCTL_PROFILE=work
profilectl sync
```

## Profiles

Profiles live in `profiles/<name>.toml` and describe what a machine should look like:

```toml
name = "default"

[[links]]
src = "config/zsh/.zshrc"
dst = "~/.zshrc"

[[links]]
src = "config/git/.gitconfig"
dst = "~/.gitconfig"

[tools]
brew = ["starship", "ripgrep", "bat", "eza"]
cargo = ["cargo-nextest"]
```

Profiles support inheritance via `extends`:

```toml
name = "work"
extends = "default"

[[links]]
src = "config/zsh/.zshrc.work"
dst = "~/.zshrc"

[tools]
brew = ["awscli", "terraform"]
```

The `work` profile inherits all links and tools from `default`, then adds or overrides its own.

## Project Structure

```
dotfiles/
├── profiles/
│   └── default.toml       # Default profile (symlinks + tools)
├── config/
│   ├── zsh/               # Zsh config files
│   ├── git/               # Git config files
│   └── shell/             # Starship prompt config
├── crates/
│   ├── profilectl/        # Binary entry point
│   ├── profilectl-cli/    # Clap subcommands
│   ├── profilectl-config/ # Profile schema and loader
│   ├── profilectl-interactive/ # Inquire TUI
│   └── profilectl-types/  # Shared types and errors
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

**Made with ❤️ by Carlos Ferreyra**
