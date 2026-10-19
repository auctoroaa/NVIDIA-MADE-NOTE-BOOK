//! Storage abstractions and persistence boundaries.

/// Stable crate metadata for the CODEX VERDANT scaffold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub implemented: bool,
}

/// Reports this crate's current implementation status.
pub fn component_info() -> ComponentInfo {
    ComponentInfo {
        name: "codex-storage",
        version: env!("CARGO_PKG_VERSION"),
        implemented: false,
    }
}
