//! Policy checks and invariant enforcement.

use serde::{Deserialize, Serialize};

/// Stable crate metadata for the CODEX VERDANT scaffold.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub implemented: bool,
}

/// Reports this crate's current implementation status.
pub fn component_info() -> ComponentInfo {
    ComponentInfo {
        name: "codex-governance",
        version: env!("CARGO_PKG_VERSION"),
        implemented: false,
    }
}
