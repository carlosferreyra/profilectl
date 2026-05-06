use clap::ValueEnum;

/// Subset of profile state to operate on.
///
/// Used by `apply`, `status`, and `check` to limit work to either the
/// installed-tools side or the dotfile-symlinks side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum Scope {
    /// Tool installations only.
    Tools,
    /// Dotfile symlinks only.
    Links,
    /// Both tools and links (default).
    #[default]
    All,
}
