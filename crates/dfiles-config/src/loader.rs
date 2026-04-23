use crate::profile::Profile;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Resolve the dotfiles repo root from:
///   1. `DFILES_HOME` env var
///   2. Current working directory (if it contains `profiles/`)
///   3. `~/.dotfiles`
pub fn repo_root() -> PathBuf {
    if let Ok(home) = std::env::var("DFILES_HOME") {
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

/// Load and return a named profile, resolving `extends` chains (max depth 8).
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

    // Resolve inheritance: parent fields fill in missing child fields.
    if let Some(ref parent_name) = profile.extends.clone() {
        let parent = load_profile_from(root, parent_name, depth + 1)?;
        merge_profiles(&mut profile, parent);
    }

    Ok(profile)
}

/// Merge parent into child: child values take precedence, parent fills gaps.
fn merge_profiles(child: &mut Profile, parent: Profile) {
    // Links: append parent links that aren't already overridden by child.
    let child_sources: std::collections::HashSet<_> =
        child.links.iter().map(|l| l.source.clone()).collect();
    for link in parent.links {
        if !child_sources.contains(&link.source) {
            child.links.push(link);
        }
    }

    // Tools: union — child + any parent tools not already listed.
    let merge_vec = |child_list: &mut Vec<String>, parent_list: Vec<String>| {
        for item in parent_list {
            if !child_list.contains(&item) {
                child_list.push(item);
            }
        }
    };
    merge_vec(&mut child.tools.brew, parent.tools.brew);
    merge_vec(&mut child.tools.cargo, parent.tools.cargo);
    merge_vec(&mut child.tools.uv, parent.tools.uv);
    merge_vec(&mut child.tools.npm, parent.tools.npm);
    merge_vec(&mut child.tools.bun, parent.tools.bun);

    // Env: parent fills keys not set by child.
    for (k, v) in parent.env {
        child.env.entry(k).or_insert(v);
    }
}
