fn print_component(name: &str, version: &str, implemented: bool) {
    println!(
        "- {} v{} [{}]",
        name,
        version,
        if implemented {
            "implemented"
        } else {
            "scaffold only"
        }
    );
}

fn main() {
    println!("CODEX VERDANT v{}", env!("CARGO_PKG_VERSION"));
    println!("AXIOM-000: Do not prematurely destroy potentially useful information.");
    println!();
    println!("Workspace components:");

    let component = codex_core::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_ingestion::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_embedding::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_storage::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_evolution::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_governance::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_gpu::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_vector::component_info();
    print_component(component.name, component.version, component.implemented);
    let component = codex_api::component_info();
    print_component(component.name, component.version, component.implemented);
}
