#!/usr/bin/env bash
# Regenerates examples/tools/sarif/*.sarif by scanning examples/tools/fixture with each tool.
# Needs Docker, plus trivy, grype, hadolint and checkov on PATH (e.g. via Homebrew).
# Snyk needs `snyk auth` first and is skipped when it is not authenticated.
#
# Usage: examples/tools/generate.sh [tool ...]   (default: all tools)
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
fixture="$here/fixture"
out="$here/sarif"
mkdir -p "$out"
cd "$fixture"

run() { echo "==> $1"; }

bandit() {
  docker run --rm -v "$fixture":/src -w /src python:3.12-slim sh -c \
    "pip -q install bandit bandit-sarif-formatter >/dev/null && bandit -r app.py -f sarif -o /tmp/bandit.sarif; cat /tmp/bandit.sarif" \
    >"$out/bandit.sarif" || true
}
checkov() {
  local tmp; tmp="$(mktemp -d)"
  command checkov -d . -o sarif --output-file-path "$tmp" --quiet --compact >/dev/null || true
  mv "$tmp/results_sarif.sarif" "$out/checkov.sarif"
}
gitleaks() {
  local tmp; tmp="$(mktemp -d "$here/.gitleaks.XXXX")"
  docker run --rm -v "$fixture":/path -v "$tmp":/out ghcr.io/gitleaks/gitleaks:latest \
    dir /path -f sarif -r /out/gitleaks.sarif --no-banner >/dev/null 2>&1 || true
  mv "$tmp/gitleaks.sarif" "$out/gitleaks.sarif" && rmdir "$tmp"
}
gosec() {
  docker run --rm -v "$fixture/go":/src -w /src securego/gosec:latest \
    -fmt sarif -no-fail -quiet ./... >"$out/gosec.sarif"
}
grype() { command grype dir:. -o sarif >"$out/grype.sarif"; }
hadolint() { command hadolint -f sarif Dockerfile >"$out/hadolint.sarif" || true; }
kics() {
  local tmp; tmp="$(mktemp -d "$here/.kics.XXXX")"
  docker run --rm -v "$fixture":/path -v "$tmp":/out checkmarx/kics:latest \
    scan -p /path -o /out --output-name kics --report-formats sarif --no-progress --silent || true
  mv "$tmp/kics.sarif" "$out/kics.sarif" && rmdir "$tmp"
}
osv-scanner() {
  docker run --rm -v "$fixture":/src ghcr.io/google/osv-scanner:latest \
    scan source --format sarif -r /src >"$out/osv-scanner.sarif" || true
}
semgrep() {
  docker run --rm -v "$fixture":/src -w /src semgrep/semgrep semgrep scan \
    --config p/python --config p/golang --config p/secrets --metrics off --sarif . \
    >"$out/semgrep.sarif" 2>/dev/null || true
}
snyk() {
  if ! command snyk whoami >/dev/null 2>&1; then echo "    skipped: run 'snyk auth' first"; return; fi
  command snyk code test --sarif-file-output="$out/snyk-code.sarif" . >/dev/null || true
  command snyk test --file=requirements.txt --package-manager=pip \
    --sarif-file-output="$out/snyk-open-source.sarif" >/dev/null || true
  command snyk iac test --sarif-file-output="$out/snyk-iac.sarif" . >/dev/null || true
}
terrascan() {
  docker run --rm -v "$fixture":/iac tenable/terrascan:latest scan -d /iac -o sarif \
    >"$out/terrascan.sarif" 2>/dev/null || true
}
tfsec() {
  docker run --rm -v "$fixture":/src aquasec/tfsec:latest /src --format sarif --no-color \
    >"$out/tfsec.sarif" 2>/dev/null || true
}
trivy() {
  command trivy fs --quiet --skip-version-check --scanners vuln,misconfig,secret --format sarif -o "$out/trivy.sarif" . >/dev/null
  # trivy records the absolute scan path; keep the checked-in file machine-independent.
  sed -i.bak "s|file://$fixture/|file:///src/|g" "$out/trivy.sarif" && rm "$out/trivy.sarif.bak"
}

all=(bandit checkov gitleaks gosec grype hadolint kics osv-scanner semgrep snyk terrascan tfsec trivy)
for tool in "${@:-${all[@]}}"; do
  run "$tool"
  "$tool"
done
