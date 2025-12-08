use anyhow::Result;
use rstest::*;
use std::fs;

pub fn load_example_sarif(name: &str) -> Result<String> {
    let path = format!("../../examples/sarif-files/{}.sarif", name);
    fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to load SARIF example {}: {}", name, e))
}

#[fixture]
pub fn minimal_sarif() -> String {
    load_example_sarif("01-minimal").expect("Failed to load minimal SARIF")
}

#[fixture]
pub fn rule_metadata_sarif() -> String {
    load_example_sarif("02-rule-metadata").expect("Failed to load rule metadata SARIF")
}

#[fixture]
pub fn suppressions_sarif() -> String {
    load_example_sarif("03-suppressions").expect("Failed to load suppressions SARIF")
}

#[fixture]
pub fn code_flows_sarif() -> String {
    load_example_sarif("04-code-flows").expect("Failed to load code flows SARIF")
}

#[fixture]
pub fn context_region_sarif() -> String {
    load_example_sarif("05-context-region").expect("Failed to load context region SARIF")
}

#[fixture]
pub fn result_stacks_sarif() -> String {
    load_example_sarif("07-result-stacks").expect("Failed to load result stacks SARIF")
}

#[fixture]
pub fn embedded_content_sarif() -> String {
    load_example_sarif("08-embedded-content").expect("Failed to load embedded content SARIF")
}
