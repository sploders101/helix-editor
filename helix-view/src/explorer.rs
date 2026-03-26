use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorerConfig {
    /// Explorer position
    pub position: ExplorerPosition,
    /// Explorer column width
    pub column_width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExplorerPosition {
    Left,
    Right,
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        return Self {
            position: ExplorerPosition::Left,
            column_width: 25,
        };
    }
}
