# SARIF Examples

This directory contains 10 curated SARIF examples demonstrating all major features of the SARIF format.

## What's Included

| File                        | Description                   |
|-----------------------------|-------------------------------|
| `01-minimal.sarif`          | Basic SARIF structure         |
| `02-rule-metadata.sarif`    | Rules with CWE/CVE metadata   |
| `03-suppressions.sarif`     | Suppressed findings           |
| `04-code-flows.sarif`       | Code flows for taint analysis |
| `05-context-region.sarif`   | Code snippets with context    |
| `06-baseline.sarif`         | Baseline comparison           |
| `07-result-stacks.sarif`    | Stack traces                  |
| `08-embedded-content.sarif` | Embedded file content         |
| `09-uri-base-ids.sarif`     | URI base ID resolution        |
| `10-default-config.sarif`   | Default rule configuration    |

## Usage

Try the converter with these examples:

```bash
# Basic usage
sarif-to-md -i examples/sarif-files/01-minimal.sarif -o output.md

# GitHub Flavored Markdown with emoji
sarif-to-md -i examples/sarif-files/02-rule-metadata.sarif \
  -o report.md \
  -f github-flavored \
  -e

# Convert all examples
for f in examples/sarif-files/*.sarif; do
  sarif-to-md -i "$f" -o "$(basename "$f" .sarif).md"
done
```

## Expected Outputs

Pre-generated markdown outputs are provided in both formats:

- `expected-outputs/github-flavored/` - Collapsible sections for GitHub
- `expected-outputs/common-mark/` - Universal compatibility

These demonstrate the converter's output and can be used for comparison.

## Using with Your Security Tools

After trying these examples, use sarif-to-md with your own security tools:

**Semgrep:**
```bash
semgrep --config=auto --sarif . > report.sarif
sarif-to-md -i report.sarif -o report.md -f github-flavored -e
```

**ESLint:**
```bash
npm install -g @microsoft/eslint-formatter-sarif
eslint . --format @microsoft/eslint-formatter-sarif -o report.sarif
sarif-to-md -i report.sarif -o report.md
```

**Snyk:**
```bash
snyk test --sarif > report.sarif
sarif-to-md -i report.sarif -o report.md
```

**GitHub CodeQL:**
```bash
# Download from Actions artifacts
sarif-to-md -i codeql-results.sarif -o security-report.md
```

## Source & License

- **Source:** https://github.com/microsoft/sarif-tutorials  
- **License:** CC BY 4.0 International  
- **Attribution:** Examples from Microsoft SARIF Tutorials