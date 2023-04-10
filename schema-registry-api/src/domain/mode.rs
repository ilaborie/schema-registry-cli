/// The resource mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceMode {
    /// Import
    Import,
    /// Read only
    ReadOnly,
    /// Read write
    #[default]
    ReadWrite,
}

/// The mode wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Mode {
    /// The mode
    pub mode: ResourceMode,
}
