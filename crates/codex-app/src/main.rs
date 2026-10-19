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

    for component in components {
        println!(
            "- {} v{} [{}]",
            component.name,
            component.version,
            if component.implemented {
                "implemented"
            } else {
                "scaffold only"
            }
        );
    }
}
