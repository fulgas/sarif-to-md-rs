# sarif-to-md

[![CI](https://github.com/fulgas/sarif-to-md-rs/workflows/CI/badge.svg)](https://github.com/fulgas/sarif-to-md-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/sarif-to-md.svg)](https://crates.io/crates/sarif-to-md)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.90%2B-blue.svg)](https://www.rust-lang.org)

A high-performance Rust library and CLI tool for converting SARIF (Static Analysis Results Interchange Format) security reports into human-readable Markdown documentation.

## Features

- **Fast & Efficient** - Written in Rust for optimal performance
- **Multiple Output Formats** - Support for GitHub Flavored Markdown and CommonMark
- **Customizable** - Optional emoji support for visual clarity
- **Comprehensive** - Extracts vulnerability details, CWE/CVE references, locations, and metadata
- **Flexible** - Use as a CLI tool or integrate as a library
- **Zero Runtime Dependencies** - Fully statically linked binaries

## What is SARIF?

SARIF (Static Analysis Results Interchange Format) is an industry-standard format for the output of static analysis tools. It's used by major security scanners including:
- Snyk
- GitHub CodeQL
- Semgrep
- ESLint
- And many more...

This tool helps you convert these machine-readable reports into beautiful, human-friendly Markdown documentation.

## Installation

### From Binary Releases

Download pre-built binaries for your platform from the [releases page](https://github.com/fulgas/sarif-to-md-rs/releases).

#### Linux / macOS
```bash
# Download and extract (replace VERSION and TARGET with your desired version and platform)
curl -L https://github.com/fulgas/sarif-to-md-rs/releases/download/vVERSION/sarif-to-md-VERSION-TARGET.tar.gz | tar xz

# Move to PATH
sudo mv sarif-to-md /usr/local/bin/

# Verify installation
sarif-to-md --version
```

#### Windows
Download the `.zip` file from releases, extract, and add to your PATH.

### From Source

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/fulgas/sarif-to-md-rs.git
cd sarif-to-md-rs
cargo install --path crates/sarif-to-md
```

### From Crates.io

```bash
cargo install sarif-to-md
```

## Quick Start

### CLI Usage

Basic conversion:
```bash
sarif-to-md -i report.sarif -o report.md
```

With options:
```bash
# GitHub Flavored Markdown with emoji
sarif-to-md -i report.sarif -o report.md -f github-flavored -e

# CommonMark format to stdout
sarif-to-md -i report.sarif -f common-mark
```

### Library Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
sarif-to-md-core = "0.1"
```

Example code:
```rust
use sarif_to_md_core::{
    ReportProcessorBuilder,
    generators::SarifMarkdownGenerator,
    markdown::MarkdownFormat,
};
use std::fs;

fn main() -> anyhow::Result<()> {
    // Read SARIF file
    let content = fs::read_to_string("report.sarif")?;
    
    // Create processor with GitHub Flavored Markdown
    let processor = ReportProcessorBuilder::new()
        .generator(SarifMarkdownGenerator::new(
            MarkdownFormat::GitHubFlavored,
            true  // with_emoji
        ))
        .content(content)
        .build()?;
    
    // Generate Markdown
    let markdown = processor.generate()?;
    
    // Save to file
    fs::write("report.md", markdown)?;
    
    Ok(())
}
```

## CLI Reference

```
sarif-to-md [OPTIONS] <COMMAND>

Options:
  -i, --input <FILE>           Input SARIF JSON file path
  -o, --output <FILE>          Output markdown file path (stdout if not specified)
  -f, --output-format <FORMAT> Markdown format [default: common-mark]
                               [possible values: github-flavored, common-mark]
  -e, --with-emoji             Include emoji in the output
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

The [`examples/`](examples/) directory contains 10 curated SARIF files demonstrating all major features of the SARIF format, along with pre-generated markdown outputs in both GitHub Flavored and CommonMark formats.

Try the converter with the provided examples:
```bash
# Basic usage with minimal example
sarif-to-md -i examples/sarif-files/01-minimal.sarif -o output.md

# GitHub Flavored Markdown with emoji
sarif-to-md -i examples/sarif-files/02-rule-metadata.sarif -o report.md -f github-flavored -e
```

See the [examples README](examples/README.md) for complete usage instructions and integration with security tools like Semgrep, ESLint, Snyk, and CodeQL.

## Output Examples

The generated Markdown includes:

- **Summary Table** - Overview of vulnerabilities by severity
- **Detailed Results** - Full information for each finding including:
    - Rule ID and severity
    - Vulnerability description
    - CWE and CVE identifiers
    - File locations with line/column numbers
    - Remediation guidance
    - Tags and metadata

### GitHub Flavored Markdown
Uses collapsible `<details>` sections for better organization in GitHub PRs and issues.

### CommonMark
Standard markdown format compatible with all markdown renderers.

## Project Structure

```
sarif-to-md-rs/
├── crates/
│   ├── sarif-to-md/          # CLI application
│   └── sarif-to-md-core/     # Core library
├── examples/                 # SARIF examples and outputs
├── LICENSE-MIT
├── LICENSE-APACHE
└── README.md
```

## Development

### Prerequisites
- Rust 1.90 or later
- Cargo

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Linting
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

## CI/CD

The project uses GitHub Actions for:
- Continuous Integration (Linux, macOS, Windows)
- Automated releases with semantic versioning
- Security audits
- Dependency management

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.md))
- MIT License ([LICENSE-MIT](LICENSE-MIT.md))

at your option.

## Acknowledgments

- Built with [serde-sarif](https://crates.io/crates/serde-sarif) for SARIF parsing
- Uses [askama](https://crates.io/crates/askama) for template rendering
- Inspired by the need for better security report documentation

## Support

- Issues: [GitHub Issues](https://github.com/fulgas/sarif-to-md-rs/issues)
- Discussions: [GitHub Discussions](https://github.com/fulgas/sarif-to-md-rs/discussions)
- Support: [Buy Me a Coffee](https://buymeacoffee.com/fulgas)