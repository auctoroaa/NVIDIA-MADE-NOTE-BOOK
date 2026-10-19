//! Vector-search interfaces.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub implemented: bool,
}

pub fn component_info() -> ComponentInfo {
    ComponentInfo { name: "codex-vector", version: env!("CARGO_PKG_VERSION"), implemented: false }
}
