"""Install and configure dotfiles."""

import sys
from pathlib import Path

import typer

from dotfiles.config_manager import ConfigManager
from dotfiles.helpers import command_exists, get_home_dir, get_platform, run_command
from dotfiles.package_manager import PackageManager


def bootstrap_uv() -> bool:
    """
    Ensure uv is installed and accessible.

    Returns:
        True if uv is available
    """
    if command_exists("uv"):
        return True

    typer.echo("⚠ UV not found. Installing UV...")
    platform = get_platform()

    if platform == "macos" or platform == "linux":
        try:
            run_command(
                ["bash", "-c", "curl -LsSf https://astral.sh/uv/install.sh | sh"],
                verbose=True,
            )
            return True
        except Exception as e:
            typer.echo(f"✗ Failed to install UV: {e}", err=True)
            return False

    elif platform == "windows":
        try:
            run_command(
                [
                    "powershell",
                    "-ExecutionPolicy",
                    "ByPass",
                    "-Command",
                    "irm https://astral.sh/uv/install.ps1 | iex",
                ],
                verbose=True,
            )
            return True
        except Exception as e:
            typer.echo(f"✗ Failed to install UV: {e}", err=True)
            return False

    return False


def install(
    force: bool = typer.Option(
        False,
        "--force",
        "-f",
        help="Overwrite existing configuration files",
    ),
    no_packages: bool = typer.Option(
        False,
        "--no-packages",
        help="Skip package manager updates/installs",
    ),
    dry_run: bool = typer.Option(
        False,
        "--dry-run",
        help="Preview changes without executing",
    ),
    verbose: bool = typer.Option(
        True,
        "--verbose",
        "-v",
        help="Print detailed output",
    ),
) -> None:
    """
    Install and configure dotfiles for your system.

    This command will:
    1. Detect your system (macOS, Windows, or Linux)
    2. Install core tools via package managers (Homebrew/winget/apt)
    3. Create symlinks for all configuration files
    4. Backup existing configurations before overwriting
    """
    # Get repo root (parent of dotfiles package)
    repo_root = Path(__file__).parent.parent.parent.parent.resolve()

    typer.echo("\n🚀 Dotfiles Installation")
    typer.echo(f"   Platform: {get_platform()}")
    typer.echo(f"   Home: {get_home_dir()}")
    typer.echo(f"   Repo: {repo_root}\n")

    # Step 1: Ensure UV is available
    typer.echo("1️⃣  Checking UV installation...")
    if not command_exists("uv"):
        typer.echo("   → Installing UV...")
        if not bootstrap_uv():
            typer.echo("✗ Failed to bootstrap UV", err=True)
            raise typer.Exit(1)
    typer.echo("   ✓ UV is available\n")

    # Step 1.5: Ensure Git is available
    typer.echo("1️⃣ .5️⃣  Checking Git installation...")
    if not command_exists("git"):
        platform = get_platform()
        if platform == "windows":
            typer.echo("   ✗ Git not found in PATH")
            typer.echo("   → Windows requires manual Git installation")
            typer.echo("   → Download from: https://git-scm.com/download/win")
            typer.echo("   → Or install via: winget install Git.Git\n")
            raise typer.Exit(1)
        else:
            typer.echo("   ⚠ Git not found (may be needed for full functionality)")
    else:
        typer.echo("   ✓ Git is available\n")

    # Step 2: Ensure Python 3.12+ is available
    typer.echo("2️⃣  Ensuring Python 3.12+ (as system version)...")
    try:
        run_command(
            [
                "uv",
                "python",
                "install",
                "--python-preference",
                "only-system",
                "3.12",
            ],
            verbose=verbose,
            dry_run=dry_run,
        )
        typer.echo("   ✓ Python 3.12+ is available\n")
    except Exception as e:
        typer.echo(f"⚠ Python 3.12 setup: {e}", err=True)
        typer.echo("   (This may be OK if already installed)\n")

    # Step 3: Detect existing configurations
    typer.echo("3️⃣  Detecting existing configurations...")
    config_mgr = ConfigManager(repo_root)
    existing = config_mgr.detect_existing_configs()
    if existing:
        typer.echo(f"   Found {len(existing)} existing config(s):")
        for config_name, config_path in existing.items():
            typer.echo(f"     - {config_name}: {config_path}")

        if not force:
            typer.echo("   (Use --force to overwrite existing configs)\n")
    else:
        typer.echo("   No existing configs detected\n")

    # Step 4: Update system packages (optional)
    if not no_packages:
        typer.echo("4️⃣  Updating system packages...")
        pkg_mgr = PackageManager(dry_run=dry_run, verbose=verbose)
        if pkg_mgr.update_packages():
            typer.echo("   ✓ Packages updated\n")
        else:
            typer.echo("   ⚠ Package update had issues (continuing...)\n")
    else:
        typer.echo("4️⃣  Skipping package updates (--no-packages)\n")

    # Step 5: Create configuration symlinks
    typer.echo("5️⃣  Creating configuration symlinks...")
    successful, failed = config_mgr.symlink_configs(
        force=force,
        verbose=verbose,
    )
    typer.echo(f"   ✓ Successful: {successful}, Failed: {failed}\n")

    if failed > 0 and not force:
        typer.echo("   ⚠ Some symlinks failed. Use --force to overwrite existing files.\n")

    # Step 6: Success summary
    typer.echo("✅ Installation Complete!")
    typer.echo("\n📋 Next Steps:")
    typer.echo("   1. Review created symlinks: ls -la ~/")
    typer.echo("   2. Reload shell: exec zsh")
    typer.echo("   3. Check command availability: which uv bun cargo go")
    typer.echo("   4. For updates later: dotfiles update\n")
