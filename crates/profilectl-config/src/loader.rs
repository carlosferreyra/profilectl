use crate::profile::{Profile, ToolSet};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Resolve the dotfiles repo root from:
///   1. `PCTL_HOME` env var
///   2. Current working directory (if it contains `profiles/`)
///   3. `~/.dotfiles`
pub fn repo_root() -> PathBuf {
    if let Ok(home) = std::env::var("PCTL_HOME") {
        return PathBuf::from(home);
    }

    let cwd = std::env::current_dir().unwrap_or_default();
    if cwd.join("profiles").exists() {
        return cwd;
    }

    dirs::home_dir()
        .map(|h| h.join(".dotfiles"))
        .unwrap_or_default()
}

/// Load and return a named profile, resolving `bundles` and `extends` chains
/// (max depth 8 for `extends`).
///
/// Merge order: **bundles → extends parent → own definition**. Own values
/// take precedence; bundles and parents fill in gaps and contribute to unions.
pub fn load_profile(name: &str) -> Result<Profile> {
    load_profile_from(&repo_root(), name, 0)
}

fn load_profile_from(root: &Path, name: &str, depth: u8) -> Result<Profile> {
    if depth > 8 {
        anyhow::bail!("Profile extends chain too deep (possible cycle near '{name}')");
    }

    let path = root.join("profiles").join(format!("{name}.toml"));
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("Profile not found: {}", path.display()))?;

    let mut profile: Profile =
        toml::from_str(&raw).with_context(|| format!("Failed to parse profile '{name}'"))?;

    // Resolve `extends`: parent fields fill gaps in the child.
    if let Some(parent_name) = profile.extends.clone() {
        let parent = load_profile_from(root, &parent_name, depth + 1)?;
        merge_profiles(&mut profile, parent);
    }

    // Resolve `bundles`: each bundle fills gaps in the (already extends-merged) profile.
    // Bundles are baked into the binary; missing bundles surface as a hard error so
    // typos in profile TOML fail loud rather than silently dropping tools.
    for bundle_name in profile.bundles.clone() {
        let bundle = resolve_bundle(&bundle_name)
            .with_context(|| format!("Unknown bundle '{bundle_name}' in profile '{name}'"))?;
        merge_profiles(&mut profile, bundle);
    }

    Ok(profile)
}

/// Resolve a bundle name to a `Profile` parsed from its baked-in TOML fragment.
///
/// Stub: real bundle TOML fragments live under `bundles/` and will be embedded
/// via `include_str!` in a follow-up. For now this returns `None` for every
/// name, so profiles that declare bundles fail with a clear "Unknown bundle"
/// error and tests for the merge mechanics drive the rest of the pipeline.
pub fn resolve_bundle(_name: &str) -> Option<Profile> {
    None
}

/// Merge parent (or bundle) into child: child values take precedence,
/// parent fills gaps and contributes to unions.
fn merge_profiles(child: &mut Profile, parent: Profile) {
    // Links: append parent links that aren't already overridden by child.
    let child_sources: std::collections::HashSet<_> =
        child.links.iter().map(|l| l.source.clone()).collect();
    for link in parent.links {
        if !child_sources.contains(&link.source) {
            child.links.push(link);
        }
    }

    merge_toolset(&mut child.tools, parent.tools);

    // Env: parent fills keys not set by child.
    for (key, value) in parent.env {
        child.env.entry(key).or_insert(value);
    }
}

fn merge_toolset(child: &mut ToolSet, parent: ToolSet) {
    fn union(child_list: &mut Vec<String>, parent_list: Vec<String>) {
        for item in parent_list {
            if !child_list.contains(&item) {
                child_list.push(item);
            }
        }
    }

    union(&mut child.mise, parent.mise);
    union(&mut child.brew, parent.brew);
    union(&mut child.apt, parent.apt);
    union(&mut child.dnf, parent.dnf);
    union(&mut child.pacman, parent.pacman);
    union(&mut child.winget, parent.winget);
    union(&mut child.choco, parent.choco);
    union(&mut child.scoop, parent.scoop);

    for (manager, parent_list) in parent.other {
        let child_list = child.other.entry(manager).or_default();
        union(child_list, parent_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Link;

    fn profile_with_tools(name: &str, brew: &[&str], mise: &[&str]) -> Profile {
        let mut profile = Profile {
            name: name.to_string(),
            ..Default::default()
        };
        profile.tools.brew = brew.iter().map(|s| s.to_string()).collect();
        profile.tools.mise = mise.iter().map(|s| s.to_string()).collect();
        profile
    }

    #[test]
    fn merge_unions_tool_lists_without_duplicates() {
        let mut child = profile_with_tools("child", &["git"], &["zsh"]);
        let parent = profile_with_tools("parent", &["git", "lazygit"], &["starship"]);

        merge_profiles(&mut child, parent);

        assert_eq!(child.tools.brew, vec!["git", "lazygit"]);
        assert_eq!(child.tools.mise, vec!["zsh", "starship"]);
    }

    #[test]
    fn merge_preserves_child_link_when_source_clashes() {
        let mut child = Profile {
            name: "child".to_string(),
            links: vec![Link {
                source: "config/zshrc".to_string(),
                target: "~/.zshrc".to_string(),
                optional: false,
            }],
            ..Default::default()
        };
        let parent = Profile {
            name: "parent".to_string(),
            links: vec![
                Link {
                    source: "config/zshrc".to_string(),
                    target: "~/wrong-target".to_string(),
                    optional: true,
                },
                Link {
                    source: "config/gitconfig".to_string(),
                    target: "~/.gitconfig".to_string(),
                    optional: false,
                },
            ],
            ..Default::default()
        };

        merge_profiles(&mut child, parent);

        assert_eq!(child.links.len(), 2);
        assert_eq!(child.links[0].target, "~/.zshrc");
        assert_eq!(child.links[1].source, "config/gitconfig");
    }

    #[test]
    fn merge_other_tools_unions_per_manager() {
        let mut child = Profile {
            name: "child".to_string(),
            ..Default::default()
        };
        child
            .tools
            .other
            .insert("nix".to_string(), vec!["ripgrep".to_string()]);

        let mut parent = Profile {
            name: "parent".to_string(),
            ..Default::default()
        };
        parent.tools.other.insert(
            "nix".to_string(),
            vec!["ripgrep".to_string(), "fd".to_string()],
        );
        parent
            .tools
            .other
            .insert("guix".to_string(), vec!["emacs".to_string()]);

        merge_profiles(&mut child, parent);

        assert_eq!(child.tools.other["nix"], vec!["ripgrep", "fd"]);
        assert_eq!(child.tools.other["guix"], vec!["emacs"]);
    }

    /// The bundled `profiles/default.toml` must continue to parse against the
    /// new `Profile` schema (collapsed `ToolSet`, `bundles` field).
    #[test]
    fn default_profile_parses_against_current_schema() {
        let raw = include_str!("../../../profiles/default.toml");
        let profile: Profile =
            toml::from_str(raw).expect("default.toml parses against current schema");
        assert_eq!(profile.name, "default");
        assert!(profile.bundles.contains(&"zsh".to_string()));
        assert!(profile.bundles.contains(&"git".to_string()));
        assert!(!profile.tools.mise.is_empty(), "mise list should be set");
    }

    /// Every bundled fragment under `bundles/` must parse against the same
    /// `Profile` schema so they can be merged into a profile via `bundles`.
    #[test]
    fn bundled_fragments_parse_against_current_schema() {
        let bundles: &[(&str, &str)] = &[
            ("mise", include_str!("../../../bundles/mise.toml")),
            ("uv", include_str!("../../../bundles/uv.toml")),
            ("rustup", include_str!("../../../bundles/rustup.toml")),
            ("bun", include_str!("../../../bundles/bun.toml")),
            ("go", include_str!("../../../bundles/go.toml")),
            ("docker", include_str!("../../../bundles/docker.toml")),
            ("zsh", include_str!("../../../bundles/zsh.toml")),
            ("git", include_str!("../../../bundles/git.toml")),
            ("vscode", include_str!("../../../bundles/vscode.toml")),
        ];

        for (name, raw) in bundles {
            let profile: Profile = toml::from_str(raw)
                .unwrap_or_else(|err| panic!("bundle '{name}' failed to parse: {err}"));
            assert_eq!(&profile.name, name, "bundle '{name}' has mismatched name");
        }
    }
}
