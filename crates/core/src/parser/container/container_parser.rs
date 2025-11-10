use crate::model::security_report::{
    ProjectType, SecurityProject, SecurityReport, Severity, Vulnerability, VulnerabilitySummary,
};
use crate::parser::container::SnykContainer;
use crate::parser::{Parser, ParserError};
use anyhow::Result;
use std::collections::HashMap;

pub(crate) struct SnykContainerParser;

impl Parser for SnykContainerParser {
    fn parse<'a>(&self, content: &str) -> Result<SecurityReport, ParserError> {
        let snyk_container: SnykContainer = serde_json::from_str(&content)?;
        println!("✅ Successfully parsed JSON");

        let mut projects = Vec::new();

        let image_vulns = parse_and_deduplicate_vulnerabilities(
            &snyk_container.vulnerabilities,
            snyk_container.unique_count as usize,
        );

        let image_summary = calculate_summary(&image_vulns);

        println!(
            "   🐳 Docker Image '{}': {} unique vulnerabilities (from {} total entries)",
            snyk_container.project_name,
            image_vulns.len(),
            snyk_container.vulnerabilities.len()
        );
        println!(
            "      → {} Critical, {} High, {} Medium, {} Low",
            image_summary.critical, image_summary.high, image_summary.medium, image_summary.low
        );

        projects.push(SecurityProject {
            name: snyk_container.project_name,
            organization: snyk_container.org,
            project_type: ProjectType::DockerImage,
            target_file: snyk_container.target_file,
            vulnerabilities: image_vulns,
            summary: image_summary,
        });

        for app in &snyk_container.applications {
            let app_vulns = parse_and_deduplicate_vulnerabilities(
                &app.vulnerabilities,
                app.unique_count as usize,
            );
            let app_summary = calculate_summary(&app_vulns);
            println!(
                "   📦 Application '{}': {} unique vulnerabilities (from {} total entries)",
                app.project_name,
                app_vulns.len(),
                app.vulnerabilities.len()
            );
            println!(
                "      → {} Critical, {} High, {} Medium, {} Low",
                app_summary.critical, app_summary.high, app_summary.medium, app_summary.low
            );

            projects.push(SecurityProject {
                name: app.project_name.clone(),
                organization: app.org.clone(),
                project_type: ProjectType::Application,
                target_file: app.target_file.clone(),
                vulnerabilities: app_vulns,
                summary: app_summary,
            });
        }

        println!("\n📊 Total projects parsed: {}", projects.len());

        Ok(SecurityReport { projects })
    }
}

/// Parses vulnerabilities and deduplicates by ID, merging duplicate information
fn parse_and_deduplicate_vulnerabilities(
    vulns: &[crate::parser::container::VulnerabilitiesItem],
    expected_unique_count: usize,
) -> Vec<Vulnerability> {
    let mut vuln_map: HashMap<String, Vulnerability> = HashMap::new();

    for v in vulns {
        // Skip if no ID
        let Some(id) = &v.id else { continue };

        if let Some(existing_vuln) = vuln_map.get_mut(id) {
            // Merge duplicate: add dependency path if present
            if !v.from.is_empty() {
                existing_vuln.from_paths.push(v.from.clone());
            }
        } else {
            // New vulnerability
            let severity = v
                .severity_with_critical
                .as_ref()
                .or(v.severity.as_ref())
                .map(|s| Severity::from_str(s))
                .unwrap_or(Severity::Low);

            let cvss_score = v.cvss_details.first().and_then(|d| d.cvss_v3_base_score);

            let cve_ids = v
                .identifiers
                .as_ref()
                .map(|i| i.cve.clone())
                .unwrap_or_default();

            let from_paths = if !v.from.is_empty() {
                vec![v.from.clone()]
            } else {
                vec![]
            };

            vuln_map.insert(
                id.clone(),
                Vulnerability {
                    id: id.clone(),
                    title: v.title.clone().unwrap_or_else(|| id.clone()),
                    severity,
                    package_name: v.package_name.clone().unwrap_or_default(),
                    version: v.version.clone().unwrap_or_default(),
                    cvss_score,
                    is_upgradable: v.is_upgradable.unwrap_or(false),
                    is_patchable: v.is_patchable.unwrap_or(false),
                    cve_ids,
                    from_paths,
                },
            );
        }
    }

    let mut deduplicated: Vec<_> = vuln_map.into_values().collect();
    deduplicated.sort_by(|a, b| a.severity.cmp(&b.severity));

    // Validation check
    if expected_unique_count > 0 && deduplicated.len() != expected_unique_count {
        eprintln!(
            "      ⚠️  Warning: Expected {} unique vulnerabilities but found {}",
            expected_unique_count,
            deduplicated.len()
        );
    }

    deduplicated
}

fn calculate_summary(vulnerabilities: &[Vulnerability]) -> VulnerabilitySummary {
    let mut summary = VulnerabilitySummary::default();

    for vuln in vulnerabilities {
        match vuln.severity {
            Severity::Critical => summary.critical += 1,
            Severity::High => summary.high += 1,
            Severity::Medium => summary.medium += 1,
            Severity::Low => summary.low += 1,
        }
    }

    summary.unique_count = vulnerabilities.len();
    summary
}
