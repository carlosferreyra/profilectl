"""Scan installed tools and write a summary file."""

import subprocess
from pathlib import Path

import typer


def which(cmd: str) -> bool:
    return subprocess.run(["which", cmd], capture_output=True).returncode == 0


def _run(cmd: list[str]) -> str:
    try:
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.stdout.strip() if result.returncode == 0 else f"(error: {result.stderr.strip()})"
    except FileNotFoundError:
        return f"({cmd[0]} not found)"


SECTIONS: list[tuple[str, list[str]]] = [
    ("Brew packages", ["brew", "ls"]),
    ("Cargo tools", ["cargo", "install", "--list"]),
    ("UV tools", ["uv", "tool", "list"]),
    ("NPM global packages", ["npm", "list", "-g", "--depth=0"]),
    ("Bun global packages", ["bun", "pm", "ls", "-g"]),
]


def _build_report() -> str:
    lines: list[str] = []
    for title, cmd in SECTIONS:
        lines.append(f"# {title}\n")
        lines.append(_run(cmd) if which(cmd[0]) else f"({cmd[0]} not found)")
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def _resolve_output(path: Path) -> Path:
    if path.is_dir():
        return path / "tools.md"
    path.parent.mkdir(parents=True, exist_ok=True)
    return path


def scan(
    path: Path = typer.Option(
        Path("."),
        "--path",
        "-p",
        help="Output directory or file path (default: ./tools.md)",
    ),
) -> None:
    """Scan installed tools (brew, cargo, uv, npm, bun) and write a report."""
    out = _resolve_output(path)
    report = _build_report()
    out.write_text(report)
    typer.echo(f"Written to {out}")
