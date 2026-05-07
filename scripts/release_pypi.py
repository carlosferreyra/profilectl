# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
"""
Generate a PyPI-publishable Python wrapper package from crates/profilectl/Cargo.toml.

The wrapper is a thin Python shim that downloads and runs the pre-built
profilectl binary from GitHub Releases. This lets users install via:

    pip install profilectl
    uvx profilectl

Usage (called by publish-pypi.yml):
    uv run scripts/release_pypi.py

Output:
    .release/python/pyproject.toml
    .release/python/src/profilectl/__init__.py
    .release/python/README.md   (copied from repo root)
"""

import sys
import textwrap
from dataclasses import dataclass, field
from pathlib import Path
from typing import Self

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # type: ignore[no-redef]


@dataclass(frozen=True)
class Author:
    name: str
    email: str | None = None

    def to_pep621(self) -> str:
        email_part = f', email = "{self.email}"' if self.email else ""
        return f'{{ name = "{self.name}"{email_part} }}'


@dataclass(frozen=True)
class PackageMetadata:
    name: str
    version: str
    description: str
    repository: str
    license_id: str
    authors: list[Author] = field(default_factory=list)

    @property
    def module_name(self) -> str:
        return self.name.lower().replace("-", "_")

    @property
    def classifiers(self) -> list[str]:
        return [
            "Programming Language :: Python :: 3",
            "Programming Language :: Python :: 3.12",
            "Programming Language :: Python :: 3 :: Only",
            "Environment :: Console",
            "Intended Audience :: Developers",
            "Intended Audience :: System Administrators",
            "Topic :: System :: Systems Administration",
            "Topic :: Utilities",
        ]

    @classmethod
    def from_cargo(cls, cargo_path: Path) -> Self:
        if not cargo_path.exists():
            raise FileNotFoundError(f"Missing Cargo.toml at {cargo_path}")

        data = tomllib.loads(cargo_path.read_text())["package"]

        # Resolve workspace-inherited fields from the root Cargo.toml
        workspace_path = cargo_path.parent.parent.parent / "Cargo.toml"
        workspace: dict = {}
        if workspace_path.exists():
            workspace = tomllib.loads(workspace_path.read_text()).get("workspace", {}).get("package", {})

        def resolve(field: str, fallback: str = "") -> str:
            val = data.get(field, fallback)
            if isinstance(val, dict) and val.get("workspace"):
                return workspace.get(field, fallback)
            return val or fallback

        parsed_authors: list[Author] = []
        raw_authors = data.get("authors", workspace.get("authors", []))
        if isinstance(raw_authors, dict) and raw_authors.get("workspace"):
            raw_authors = workspace.get("authors", [])
        for raw in raw_authors:
            match raw.split("<"):
                case [name, email_raw]:
                    parsed_authors.append(Author(name.strip(), email_raw.removesuffix(">").strip()))
                case [name]:
                    parsed_authors.append(Author(name.strip()))

        return cls(
            name=data["name"],
            version=resolve("version"),
            description=resolve("description", "Rust CLI wrapper"),
            repository=resolve("repository"),
            license_id=resolve("license", "MIT"),
            authors=parsed_authors,
        )


class Templates:
    PYPROJECT = textwrap.dedent("""
        [project]
        name = "{meta.name}"
        version = "{meta.version}"
        description = "{meta.description}"
        readme = "README.md"
        requires-python = ">=3.12"
        license = "{meta.license_id}"
        authors = [
            {authors}
        ]
        dependencies = []
        classifiers = [
            {classifiers}
        ]

        [project.scripts]
        {meta.name} = "{meta.module_name}:main"

        [project.urls]
        Repository = "{meta.repository}"

        [build-system]
        requires = ["uv_build>=0.11.2,<0.12.0"]
        build-backend = "uv_build"
    """).strip()

    # Thin shim: downloads the pre-built binary on first run, then delegates.
    CLI_WRAPPER = textwrap.dedent("""
        import platform
        import subprocess
        import sys
        from pathlib import Path


        def _bootstrap_binary() -> None:
            tag = "v{meta.version}"
            base = "{meta.repository}".rstrip("/")

            match platform.system().lower():
                case "windows":
                    url = f"{{base}}/releases/download/{{tag}}/{meta.name}-installer.ps1"
                    cmd = ["powershell", "-NoProfile", "-ExecutionPolicy", "Bypass",
                           "-Command", f"iwr -useb '{{url}}' | iex"]
                case _:
                    url = f"{{base}}/releases/download/{{tag}}/{meta.name}-installer.sh"
                    cmd = ["sh", "-c", f"curl -LsSf '{{url}}' | sh"]

            subprocess.run(cmd, check=False)


        def main() -> int:
            bin_name = "{meta.name}"
            if platform.system().lower() == "windows":
                bin_name += ".exe"

            # Prefer the absolute CARGO_HOME path to avoid shim self-recursion.
            exe = Path.home() / ".cargo" / "bin" / bin_name

            if not exe.exists():
                print(f"Binary not found at {{exe}}. Attempting to install...", file=sys.stderr)
                _bootstrap_binary()

            if exe.exists():
                return subprocess.run([str(exe), *sys.argv[1:]]).returncode

            print(f"Failed to find or install {{bin_name}} at {{exe}}", file=sys.stderr)
            return 1


        if __name__ == "__main__":
            sys.exit(main())
    """).strip()


def main() -> None:
    repo_root = Path(__file__).parent.parent
    cargo_path = repo_root / "crates" / "profilectl" / "Cargo.toml"

    try:
        meta = PackageMetadata.from_cargo(cargo_path)
    except Exception as e:
        print(f"FATAL: {e}", file=sys.stderr)
        sys.exit(1)

    out_dir = repo_root / ".release" / "python"
    pkg_dir = out_dir / "src" / meta.module_name
    pkg_dir.mkdir(parents=True, exist_ok=True)

    authors_toml = ",\n    ".join(a.to_pep621() for a in meta.authors)
    classifiers_toml = ",\n    ".join(f'"{c}"' for c in meta.classifiers)

    (out_dir / "pyproject.toml").write_text(
        Templates.PYPROJECT.format(meta=meta, authors=authors_toml, classifiers=classifiers_toml)
    )
    (pkg_dir / "__init__.py").write_text(Templates.CLI_WRAPPER.format(meta=meta))

    if (readme := repo_root / "README.md").exists():
        (out_dir / "README.md").write_text(readme.read_text())

    print(f"Generated {meta.name} v{meta.version} Python wrapper → {out_dir}")


if __name__ == "__main__":
    main()
