//! Profile schema and resolution for `profilectl`.

/// A profile declaration before it is lowered into desired state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Profile {
    /// Profile name.
    pub name: String,
    /// Parent profile names resolved before this profile's own fields.
    pub extends: Vec<String>,
    /// Built-in bundles resolved before inherited profiles.
    pub bundles: Vec<String>,
    /// File links declared by this profile.
    pub links: Vec<Link>,
    /// Tool declarations grouped by package manager.
    pub tools: Vec<Tool>,
    /// Explicit tasks declared by this profile.
    pub tasks: Vec<Task>,
}

/// A source-to-target file link.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    /// Repository-relative source path.
    pub source: String,
    /// Machine target path.
    pub target: String,
    /// Whether missing sources should be skipped.
    pub optional: bool,
}

/// A tool package declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
    /// Package manager name.
    pub manager: String,
    /// Package name.
    pub package: String,
}

/// A controlled task declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Stable task name.
    pub name: String,
    /// Repository-relative command to execute.
    pub command: String,
    /// Trigger policy.
    pub when: TaskTrigger,
}

/// Task trigger policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskTrigger {
    /// Run every time the profile is applied.
    Always,
    /// Run only once per machine.
    Once,
    /// Run when the task input hash changes.
    Changed,
}
