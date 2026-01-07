# dotfiles 🚀

Cross-platform dotfiles automation for macOS and Windows, powered by Python.

Manage your development environment configurations with a single repository. Install once, sync
everywhere.

## Features

- **🐍 Pure Python**: No shell script complexity—clean, typed Python code
- **🔄 Cross-Platform**: Supports macOS (Homebrew), Windows (winget), and Linux (apt)
- **📦 Package Management**: Automated installation and updates for all development tools
- **🔗 Smart Symlinks**: Idempotent symlink creation with backup support
- **🛠️ Multi-Language Support**: Rust, Python, Go, JavaScript/TypeScript, C/C++, Java
- **⚡ Tool Updates**: One command to update everything (brew, cargo, uv, bun, npm, etc.)
- **🔐 Configuration Detection**: Automatically detects and preserves existing configs

## Supported Tools

### Package Managers

- **macOS**: Homebrew (`brew`)
- **Windows**: Windows Package Manager (`winget`)
- **Linux**: APT (`apt-get`)

### Development Tools

- **Python**: `uv` (package manager), Python 3.10/3.11/3.12
- **Rust**: `rustup`, `cargo`
- **JavaScript/TypeScript**: `bun`, `nvm` (Node version manager)
- **Go**: `go`
- **C/C++**: LLVM, clang, gcc

### Shell & Terminal

- **Shell**: `zsh`, `bash`
- **Prompt**: `starship`
- **Multiplexer**: `tmux`
- **Utilities**: `jq`, `ripgrep`, `fd`, `bat`, `eza`, `tldr`

## Quick Start

### Bootstrap Instructions

#### macOS & Linux

1. **Install UV** (Python package manager):

   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

2. **Install Python 3.12**:

   ```bash
   uv python install 3.12
   ```

3. **Clone and install dotfiles**:
   ```bash
   git clone https://github.com/carlosferreyra/dotfiles.git ~/.dotfiles
   cd ~/.dotfiles
   uv run install.py
   ```

#### Windows (PowerShell)

1. **Install UV**:

   ```powershell
   powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
   ```

2. **Install Python 3.12**:

   ```powershell
   uv python install 3.12
   ```

3. **Clone and install dotfiles**:
   ```powershell
   git clone https://github.com/carlosferreyra/dotfiles.git $env:USERPROFILE\.dotfiles
   cd $env:USERPROFILE\.dotfiles
   uv run install.py
   ```

## Usage

### Installation

Full installation with all system packages:

```bash
uv run install.py
```

Skip package manager updates (faster):

```bash
uv run install.py --no-packages
```

Overwrite existing configurations:

```bash
uv run install.py --force
```

Preview changes without executing:

```bash
uv run install.py --dry-run
```

### Updates

Update everything (system packages + tools):

```bash
uv run update.py
```

Update only development tools (skip system packages):

```bash
uv run update.py --tools-only
```

Preview updates without executing:

```bash
uv run update.py --dry-run
```

## Configuration

### Project Structure

```
dotfiles/
├── install.py                 # Main installation script
├── update.py                  # Tool and package updater
├── pyproject.toml            # Python project config
├── config/                    # Configuration files
│   ├── zsh/
│   │   ├── .zshrc            # Main zsh config
│   │   ├── .zshenv           # Zsh environment variables
│   │   ├── .zprofile         # Zsh profile
│   │   └── config/           # Modular zsh configs
│   │       ├── 00_environment.zsh
│   │       ├── 01_plugins.zsh
│   │       ├── 02_options.zsh
│   │       ├── 03_aliases.zsh
│   │       ├── 04_functions.zsh
│   │       ├── 05_completions.zsh
│   │       └── 06_init.zsh
│   ├── git/
│   │   ├── .gitconfig
│   │   └── .gitignore_global
│   └── shell/
│       └── starship.toml
├── manifests/
│   ├── links.json            # Symlink configuration
│   ├── Brewfile              # macOS packages
│   └── packages.json         # Windows packages
└── src/dotfiles/
    ├── __init__.py
    ├── helpers.py            # Cross-platform utilities
    ├── config_manager.py     # Symlink management
    └── package_manager.py    # Package installation
```

### Symlink Manifest

The `manifests/links.json` defines which files get symlinked:

```json
{
	"config/zsh/.zshrc": "~/.zshrc",
	"config/zsh/.zshenv": "~/.zshenv",
	"config/git/.gitconfig": "~/.gitconfig"
}
```

Add new symlinks by adding entries to this JSON file.

### Brewfile (macOS)

The `manifests/Brewfile` lists all macOS packages. Customize by editing:

```bash
# Edit Brewfile
code manifests/Brewfile

# Install/sync packages
brew bundle --file=manifests/Brewfile
```

### Packages (Windows)

Windows package configuration is in `manifests/packages.json` (WIP).

## Platform-Specific Configuration

Some configurations are OS-specific. The scripts handle this automatically:

- **macOS**: Uses Homebrew, `.zprofile` for shell init, GPG keychain
- **Windows**: Uses winget, PowerShell profile, GPG4Win (placeholder)
- **Linux**: Uses apt, standard `.bashrc`/.zshrc`, system GPG

## Customization

### Adding New Dotfiles

1. Copy the file to `config/<category>/`
2. Add entry to `manifests/links.json`
3. Run `uv run install.py --force`

### Adding New Tools

**macOS**:

```bash
# Edit Brewfile
code manifests/Brewfile

# Add package and run
uv run install.py
```

**Windows**:

```json
// Edit manifests/packages.json
{
	"packages": ["Git.Git", "JetBrains.Toolbox", "golang.Go"]
}
```

### Python Version

This repo requires **Python 3.12+** (set in `pyproject.toml`). To use a different version:

```bash
uv python install 3.13
uv run install.py
```

## What Gets Installed

### System Packages (Brewfile on macOS)

- Git, Zsh, Starship, Tmux
- Python 3.10/3.11/3.12, UV, Pipx
- Rustup, Go, LLVM, Make, CMake
- Utilities: jq, ripgrep, fd, bat, eza, tldr
- Optional: Docker, PostgreSQL

### Development Tools

- UV self-updates and tool upgrades
- Rust/Cargo tool updates
- Bun upgrades
- npm global updates

## Troubleshooting

### UV not found after installation

```bash
# Ensure UV is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.zshrc
```

### Symlink creation failed

```bash
# Check existing config
ls -la ~/.zshrc

# Use --force to overwrite
uv run install.py --force

# Or manually remove and retry
rm ~/.zshrc
uv run install.py
```

### Homebrew not found (macOS)

```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### winget not found (Windows)

Windows Package Manager comes with Windows 11+. For Windows 10, install from:
https://github.com/microsoft/winget-cli

## Backup & Recovery

The installer automatically backs up existing configurations with `.backup` extension:

- `~/.zshrc.backup` (if `.zshrc` already exists)
- `~/.gitconfig.backup` (if `.gitconfig` already exists)

To restore:

```bash
mv ~/.zshrc.backup ~/.zshrc
```

## Architecture

### Design Principles

1. **Pure Python**: No bash dependencies—single language across platforms
2. **Cross-Platform**: `platform.system()` and conditional logic handle OS differences
3. **Idempotent**: Safe to re-run `install.py` multiple times
4. **Modular**: Each tool/feature in separate Python modules
5. **Declarative**: Configuration in JSON manifests, not code

### Key Modules

- **`helpers.py`**: Cross-platform utilities (run_command, safe_symlink, etc.)
- **`config_manager.py`**: Symlink creation and verification
- **`package_manager.py`**: Abstracts brew/winget/apt

## Future Enhancements

- [ ] Windows package manifest (packages.json)
- [ ] GPG4Win setup wizard for Windows
- [ ] SSH config template with security best practices
- [ ] VSCode profile integration (if sync is disabled)
- [ ] Pre-commit hooks automation
- [ ] Custom functions/aliases installer
- [ ] Automated config backup to cloud storage

## Contributing

This is a personal dotfiles repo, but if you have improvements:

1. Fork the repository
2. Create a branch for your feature
3. Ensure Python 3.12+ compatibility
4. Submit a pull request

## License

MIT License - see LICENSE file for details

## Resources

- **UV Documentation**: https://docs.astral.sh/uv/
- **Homebrew**: https://brew.sh
- **Starship Prompt**: https://starship.rs
- **Dotfiles Community**: https://dotfiles.github.io

---

**Made with ❤️ by Carlos Ferreyra**

Last updated: January 2026
