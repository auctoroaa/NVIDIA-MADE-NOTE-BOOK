fn main() {
    println!("CODEX VERDANT v{}", env!("CARGO_PKG_VERSION"));
    println!("AXIOM-000: Do not prematurely destroy potentially useful information.");
    println!();
    println!("Workspace components:");

    let components = [
        codex_core::component_info(),
        codex_ingestion::component_info(),
        codex_embedding::component_info(),
        codex_storage::component_info(),
        codex_evolution::component_info(),
        codex_governance::component_info(),
        codex_gpu::component_info(),
        codex_vector::component_info(),
        codex_api::component_info(),
    ];

    for component in components.map(|component| {
        (component.name, component.version, component.implemented)
    }) {
        println!(
            "- {} v{} [{}]",
            component.0,
            component.1,
            if component.2 {
                "implemented"
            } else {
                "scaffold only"
            }
        );
    }
}
