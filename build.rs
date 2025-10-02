fn main() {
    let contract_dirs: Vec<&str> = vec!["./contracts"];
    blueprint_sdk::build::soldeer_update();
    blueprint_sdk::build::build_contracts(contract_dirs);
}
