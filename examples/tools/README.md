# Tool SARIF examples

Real SARIF written by security tools, used to check that `sarif-to-md` renders any
producer's output, not only the SARIF specification samples in `../sarif-files`.

- `fixture/` is a small project with deliberate problems (SQL injection, shell
  injection, weak hashing, a vulnerable `requirements.txt`, an unpinned Dockerfile,
  public S3 and SSH in Terraform, a privileged Kubernetes pod). It is never built or run.
- `sarif/` holds each tool's output for `fixture/`.
- `expected-outputs/` holds the Markdown `sarif-to-md` renders for each file, in
  CommonMark and in GitHub Flavored Markdown with emoji.

The `tool_outputs` integration test in `crates/sarif-to-md` renders every file in
`sarif/` and compares it with `expected-outputs/`.

| File | Tool | Version | Results |
|------|------|---------|--------:|
| `bandit.sarif` | Bandit |  | 5 |
| `checkov.sarif` | Checkov | 3.2.490 | 25 |
| `gitleaks.sarif` | gitleaks | v8.0.0 | 0 |
| `gosec.sarif` | gosec | dev | 9 |
| `grype.sarif` | grype | 0.118.0 | 7 |
| `hadolint.sarif` | Hadolint | 2.14.0 | 7 |
| `kics.sarif` | KICS | v2.2.1 | 42 |
| `osv-scanner.sarif` | osv-scanner | 2.7.1 | 48 |
| `semgrep.sarif` | Semgrep OSS | 1.185.0 | 4 |
| `terrascan.sarif` | terrascan | 1.19.9 | 20 |
| `tfsec.sarif` | defsec |  | 13 |
| `trivy.sarif` | Trivy | 0.67.2 | 44 |

## Regenerating

```bash
examples/tools/generate.sh            # every tool
examples/tools/generate.sh trivy kics # only some
```

The script needs Docker, plus `trivy`, `grype`, `hadolint` and `checkov` on `PATH`.
Snyk runs only after `snyk auth`. After regenerating, refresh the expected outputs:

```bash
for f in examples/tools/sarif/*.sarif; do
  t=$(basename "$f" .sarif)
  SOURCE_DATE_EPOCH=0 cargo run -q -p sarif-to-md -- -i "$f" -o "examples/tools/expected-outputs/common-mark/$t.md"
  SOURCE_DATE_EPOCH=0 cargo run -q -p sarif-to-md -- -i "$f" -f github-flavored -e \
    -o "examples/tools/expected-outputs/github-flavored/$t.md"
done
```

## Adding a tool

Add a function for it to `generate.sh` and to the `all` list, run it, and render its
expected outputs as above. The test picks the new file up by itself.

