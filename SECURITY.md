# Security Policy

## Reporting a Vulnerability

We take the security of sarif-to-md-rs seriously. If you discover a security vulnerability, please report it responsibly.

### How to Report

**Please DO NOT open a public GitHub issue for security vulnerabilities.**

Instead, please use GitHub Security Advisories:

1. Go to the [Security tab](../../security/advisories)
2. Click "Report a vulnerability"
3. Fill out the form with details

This ensures the vulnerability is reported privately and securely.

### What to Include

Please include as much information as possible:

- Type of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)
- Your contact information for follow-up

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 5 business days
- **Fix Timeline**: Depends on severity and complexity

We will:
1. Confirm receipt of your report
2. Assess the vulnerability
3. Develop and test a fix
4. Release a security patch
5. Publicly disclose the vulnerability (with credit to you, if desired)

## Supported Versions

We provide security updates for:

| Version | Supported          |
| ------- | ------------------ |
| latest (main branch) | :white_check_mark: |
| Tagged releases | :white_check_mark: |

## Security Best Practices

When using sarif-to-md-rs:

1. **Input Validation**
    - Validate SARIF files from untrusted sources
    - Be cautious with user-provided file paths
    - Use absolute paths when possible

2. **File System Security**
    - Ensure output directory permissions are appropriate
    - Be careful with file overwrites
    - Validate file extensions and paths

3. **Dependency Management**
    - We automatically scan dependencies for vulnerabilities
    - Dependencies are validated for license compliance
    - Security advisories are monitored weekly

4. **Binary Security**
    - Download binaries only from official GitHub releases
    - Verify checksums when available
    - Use package managers (cargo, homebrew) when possible

## Known Security Considerations

### File Processing

sarif-to-md-rs processes SARIF JSON files which may contain:
- Large amounts of data (potential DoS)
- Malformed JSON (parser vulnerabilities)
- User-controlled file paths

Always validate input from untrusted sources.

### Dependencies

This tool depends on Rust crates from crates.io:
- All dependencies are scanned for known vulnerabilities
- License compliance is automatically checked
- Security advisories are monitored via RustSec

Our automated security scans check for:
- Known CVEs in dependencies
- License compliance issues
- Unmaintained or yanked crates

## Security Updates

Security fixes are released as soon as possible. Subscribe to:
- GitHub Security Advisories for this repo
- GitHub release notifications
- Watch this repository for updates

## Acknowledgments

We appreciate responsible disclosure and will credit researchers who report valid vulnerabilities (unless they prefer to remain anonymous).

## Automated Security Measures

This project includes several automated security measures:

- **Weekly dependency scans** with cargo-audit
- **License compliance checking** with cargo-deny
- **Vulnerability database updates** before each scan
- **Multi-platform security testing** in CI/CD
- **Semantic release process** for consistent updates

---

Thank you for helping keep sarif-to-md-rs secure! 🔒