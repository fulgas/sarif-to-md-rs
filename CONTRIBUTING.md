# Contributing to sarif-to-md-rs

Thank you for your interest in contributing to sarif-to-md-rs! We welcome contributions from the community.

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Issues

- **Search existing issues** first to avoid duplicates
- Use the issue templates when available
- Include clear, detailed information:
  - Steps to reproduce
  - Expected vs actual behavior
  - Environment details (OS, Docker version, etc.)
  - Relevant logs or error messages

### Suggesting Enhancements

- Open an issue describing the enhancement
- Explain the use case and benefits
- Be open to discussion and feedback

### Pull Requests

1. **Fork the repository** and create a branch from `main`
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Follow existing code style and conventions
   - Keep changes focused and atomic
   - Write clear, descriptive commit messages

3. **Test your changes**
   - Ensure existing tests pass
   - Add new tests for new functionality
   - Test locally using the test workflows

4. **Update documentation**
   - Update README.md if needed
   - Add inline comments for complex logic
   - Update examples if behavior changes

5. **Submit the pull request**
   - Provide a clear description of changes
   - Reference any related issues
   - Be responsive to feedback and reviews

## Development Setup

### Prerequisites

- Rust (stable toolchain)
- Cargo
- Git
- A GitHub account

### Local Development

```bash
# Clone and build
git clone https://github.com/your-username/sarif-to-md-rs
cd sarif-to-md-rs
cargo build

# Run tests
cargo test

# Run linting
cargo clippy -- -D warnings
cargo fmt --all -- --check

# Check licenses and dependencies
cargo deny check
```

### Testing Your Changes

```bash
# Build and test locally
cargo build --release
./target/release/sarif-to-md --help

# Test with sample SARIF file
./target/release/sarif-to-md input.sarif -o output.md

# Run full test suite across platforms
cargo test --all-features --workspace
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Write idiomatic Rust code
- Add documentation for public APIs
- Include unit tests for new functionality

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/) specification for semantic commit messages. This helps with automated versioning and changelog generation.

### Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries
- **ci**: Changes to CI configuration files and scripts
- **build**: Changes that affect the build system or external dependencies
- **revert**: Reverts a previous commit

### Examples

```
feat: add support for custom markdown formats
feat(core): add HTML table output format
fix: resolve SARIF parsing error for empty results
fix(cli): handle missing output file gracefully
docs: update README with new CLI options
docs(contributing): add development setup guide
test: add integration tests for markdown generation
ci: add cross-platform build workflow
chore: update dependencies to latest versions
refactor: simplify markdown generator interface
```

### Breaking Changes

Breaking changes should be indicated by adding `!` after the type/scope and including `BREAKING CHANGE:` in the footer:

```
feat!: change default output format to GitHub-style

BREAKING CHANGE: The default markdown format has changed from basic to GitHub-flavored.
Users who rely on the previous format should explicitly set --format=basic.
```

### Scope (Optional)

The scope provides additional context:
- `core`: Changes to sarif-to-md-core crate
- `cli`: Changes to command-line interface
- `generator`: Changes to markdown generators
- `ci`: Changes to CI workflows
- `docs`: Documentation changes

### Tips

- Keep the subject line under 72 characters
- Use imperative mood ("add" not "added" or "adds")
- Don't capitalize the first letter of the subject
- No period at the end of the subject line
- Separate subject from body with a blank line
- Use the body to explain what and why, not how
- Reference issues and pull requests in the footer

## Review Process

1. All submissions require review
2. Commit messages will be validated for semantic format
3. Maintainers will provide feedback
4. Address feedback and update your PR
5. Once approved, maintainers will merge

## Testing Requirements

- All new features must include unit tests
- Bug fixes should include regression tests
- Integration tests for new CLI functionality
- Documentation tests for public APIs (`cargo doc --no-deps`)
- Ensure all tests pass: `cargo test --all-features --workspace`
- Check code coverage (aim for >80%)

## Quality Requirements

Before submitting a PR, ensure:

```bash
# Format and lint
cargo fmt --all
cargo clippy -- -D warnings

# Security and license checks
cargo audit
cargo deny check

# Documentation
cargo doc --no-deps --document-private-items

# All tests pass
cargo test --all-features --workspace
```

## Release Process

Releases are automated using semantic-release:
- Commits determine version bumps automatically
- Release notes are generated from commit messages
- Cross-platform binaries are built and attached to releases
- Crates are published to crates.io automatically
- Use conventional commits to trigger proper releases

## Questions?

- Open a discussion on GitHub
- Check existing issues and documentation
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing! 🎉
