//! Local state inventory for `profilectl`.

use std::path::PathBuf;

/// Persisted state about artifacts profilectl manages on a machine.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    /// Managed filesystem artifacts.
    pub managed_items: Vec<ManagedItem>,
    /// Last successful profile name.
    pub last_profile: Option<String>,
}

/// A single managed artifact in the local state inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedItem {
    /// Managed path.
    pub path: PathBuf,
    /// Kind of artifact stored at the path.
    pub kind: ManagedItemKind,
}

/// Managed artifact kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedItemKind {
    /// A symlink created by profilectl.
    Symlink,
    /// A rendered template output.
    RenderedFile,
    /// A backup created before replacing a conflict.
    Backup,
}
