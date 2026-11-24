# SARIF Security Report
## gosec vdev

###  Summary

| Severity | Count |
|----------|-------|
| Error | 8 |
| Warning | 1 |
| **Total** | **9** |

---

###  Detailed Results

#### Error - G701

**Message:** SQL injection via taint analysis

**Description:** SQL injection via taint analysis

**Tags:**
- `security`
- `HIGH`

**Locations:**
- `main.go`:14:22

#### Error - G702

**Message:** Command injection via taint analysis

**Description:** Command injection via taint analysis

**Tags:**
- `security`
- `HIGH`

**Locations:**
- `main.go`:16:19

#### Error - G705

**Message:** XSS via taint analysis

**Description:** XSS via taint analysis

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:17:14

#### Error - G401

**Message:** Use of weak cryptographic primitive

**Description:** Use of weak cryptographic primitive

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:17:24

#### Error - G114

**Message:** Use of net/http serve function that has no support for setting timeouts

**Description:** Use of net/http serve function that has no support for setting timeouts

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:22:2

#### Error - G204

**Message:** Subprocess launched with variable

**Description:** Use of net/http serve function that has no support for setting timeouts

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:16:7

#### Error - G202

**Message:** SQL string concatenation

**Description:** Use of net/http serve function that has no support for setting timeouts

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:14:23

#### Error - G501

**Message:** Blocklisted import crypto/md5: weak cryptographic primitive

**Description:** Use of weak cryptographic primitive

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:4:2

#### Warning - G104

**Message:** Errors unhandled

**Description:** Errors unhandled

**Tags:**
- `security`
- `LOW`

**Locations:**
- `main.go`:22:2

---

*Report generated on 2026-11-24 15:30:19 UTC*