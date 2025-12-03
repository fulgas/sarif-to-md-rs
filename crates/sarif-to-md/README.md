# sarif-to-md CLI

Command-line interface for converting SARIF security reports to Markdown format.

## Installation

### From Crates.io
```bash
cargo install sarif-to-md
```

### From Source
```bash
git clone https://github.com/fulgas/sarif-to-md-rs.git
cd sarif-to-md-rs
cargo install --path crates/sarif-to-md
```

### From Binary Releases
Download pre-built binaries from the [releases page](https://github.com/fulgas/sarif-to-md-rs/releases).

## Usage

### Basic Examples

Convert SARIF to Markdown:
```bash
sarif-to-md -i security-report.sarif -o report.md
```

Output to stdout:
```bash
sarif-to-md -i security-report.sarif
```

### Advanced Examples

GitHub Flavored Markdown with emoji:
```bash
sarif-to-md \
  -i security-report.sarif \
  -o report.md \
  -f github-flavored \
  -e 
```

CommonMark format (default):
```bash
sarif-to-md \
  -i security-report.sarif \
  -o report.md \
  -f common-mark
```

### Pipeline Usage

Use in CI/CD pipelines:
```bash
# Run security scanner and convert results
snyk test --sarif > results.sarif
sarif-to-md -i results.sarif -o security-report.md -f github-flavored -e sarif
```

Combine with other tools:
```bash
# Generate report and create GitHub issue
sarif-to-md -i scan.sarif sarif | gh issue create \
  --title "Security Scan Results" \
  --body-file -
```

## Command Reference

```
sarif-to-md [OPTIONS] <COMMAND>

Options:
  -i, --input <FILE>           Input SARIF JSON file path [required]
  -o, --output <FILE>          Output markdown file (stdout if omitted)
  -f, --output-format <FORMAT> Markdown output format [default: common-mark]
                               Values: github-flavored, common-mark
  -e, --with-emoji             Include emoji in severity indicators
  -h, --help                   Print help information
  -V, --version                Print version information
```

## Output Formats

### GitHub Flavored Markdown
Best for GitHub PRs, issues, and repositories. Features:
- Collapsible `<details>` sections
- HTML formatting
- Optimized for GitHub rendering

### CommonMark
Standard Markdown format. Features:
- Universal compatibility
- Plain text formatting
- Works with any Markdown renderer

## Exit Codes

- `0` - Success
- `1` - Error (invalid input, file not found, parsing error)

## Environment Variables

None currently used. All configuration is via command-line flags.

## Examples Directory

See the parent repository for example SARIF files and their generated Markdown outputs.

## Integration Examples

### GitHub Actions
```yaml
- name: Convert SARIF to Markdown
  run: |
    sarif-to-md -i results.sarif -o security-report.md -f github-flavored -e
    
- name: Comment PR with results
  uses: actions/github-script@v6
  with:
    script: |
      const fs = require('fs');
      const report = fs.readFileSync('security-report.md', 'utf8');
      github.rest.issues.createComment({
        issue_number: context.issue.number,
        owner: context.repo.owner,
        repo: context.repo.repo,
        body: report
      });
```

### GitLab CI
```yaml
security-report:
  script:
    - security-scanner --output results.sarif
    - sarif-to-md -i results.sarif -o report.md
  artifacts:
    paths:
      - report.md
    reports:
      markdown: report.md
```

## Troubleshooting

### "File not found" Error
Ensure the input file path is correct and the file exists:
```bash
ls -la security-report.sarif
```

### Invalid SARIF Format
Validate your SARIF file against the schema:
```bash
# Using a SARIF validator
npx @microsoft/sarif-validator security-report.sarif
```

### Permission Denied
Check file permissions:
```bash
chmod +r security-report.sarif
```

## Performance

The CLI is optimized for large SARIF files:
- Handles 10,000+ findings efficiently
- Low memory footprint
- Fast template rendering

## License

This project is dual-licensed under MIT or Apache-2.0. See the parent repository for full license text.

## Support

For issues and feature requests, please use the [GitHub issue tracker](https://github.com/fulgas/sarif-to-md-rs/issues).