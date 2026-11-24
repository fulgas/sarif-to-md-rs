# SARIF Security Report
## Semgrep OSS

###  Summary

| Severity | Count |
|----------|-------|
| Error | 2 |
| Warning | 2 |
| **Total** | **4** |

---

###  Detailed Results

#### Error - python.lang.security.audit.subprocess-shell-true.subprocess-shell-true

**Message:** Found 'subprocess' function 'call' with 'shell=True'. This is dangerous because this call will spawn the command using a shell process. Doing so propagates current shell settings and variables, which makes it much easier for a malicious actor to execute commands. Use 'shell=False' instead.

**Description:** Semgrep Finding: python.lang.security.audit.subprocess-shell-true.subprocess-shell-true

**Tags:**
- `CWE-78: Improper Neutralization of Special Elements used in an OS Command ('OS Command Injection')`
- `MEDIUM CONFIDENCE`
- `OWASP-A01:2017 - Injection`
- `OWASP-A03:2021 - Injection`
- `OWASP-A05:2025 - Injection`
- `security`

**Documentation:** [View details](https://semgrep.dev/r/python.lang.security.audit.subprocess-shell-true.subprocess-shell-true)

**Locations:**
- `app.py`:16:39

#### Error - go.lang.security.injection.tainted-sql-string.tainted-sql-string

**Message:** User data flows into this manually-constructed SQL string. User data can be safely inserted into SQL strings using prepared statements or an object-relational mapper (ORM). Manually-constructed SQL strings is a possible indicator of SQL injection, which could let an attacker steal or manipulate data from the database. Instead, use prepared statements (`db.Query("SELECT * FROM t WHERE id = ?", id)`) or a safe library.

**Description:** Semgrep Finding: go.lang.security.injection.tainted-sql-string.tainted-sql-string

**Tags:**
- `CWE-89: Improper Neutralization of Special Elements used in an SQL Command ('SQL Injection')`
- `HIGH CONFIDENCE`
- `OWASP-A01:2017 - Injection`
- `OWASP-A03:2021 - Injection`
- `OWASP-A05:2025 - Injection`
- `security`

**Documentation:** [View details](https://semgrep.dev/r/go.lang.security.injection.tainted-sql-string.tainted-sql-string)

**Locations:**
- `go/main.go`:14:23

#### Warning - go.lang.security.audit.crypto.use_of_weak_crypto.use-of-md5

**Message:** Detected MD5 hash algorithm which is considered insecure. MD5 is not collision resistant and is therefore not suitable as a cryptographic signature. Use SHA256 or SHA3 instead.

**Description:** Semgrep Finding: go.lang.security.audit.crypto.use_of_weak_crypto.use-of-md5

**Tags:**
- `CWE-328: Use of Weak Hash`
- `MEDIUM CONFIDENCE`
- `OWASP-A02:2021 - Cryptographic Failures`
- `OWASP-A03:2017 - Sensitive Data Exposure`
- `OWASP-A04:2025 - Cryptographic Failures`
- `security`

**Documentation:** [View details](https://semgrep.dev/r/go.lang.security.audit.crypto.use_of_weak_crypto.use-of-md5)

**Locations:**
- `go/main.go`:17:24

#### Warning - go.lang.security.audit.net.use-tls.use-tls

**Message:** Found an HTTP server without TLS. Use 'http.ListenAndServeTLS' instead. See https://golang.org/pkg/net/http/#ListenAndServeTLS for more information.

**Description:** Semgrep Finding: go.lang.security.audit.net.use-tls.use-tls

**Tags:**
- `CWE-319: Cleartext Transmission of Sensitive Information`
- `MEDIUM CONFIDENCE`
- `OWASP-A02:2021 - Cryptographic Failures`
- `OWASP-A03:2017 - Sensitive Data Exposure`
- `OWASP-A04:2025 - Cryptographic Failures`
- `security`

**Documentation:** [View details](https://semgrep.dev/r/go.lang.security.audit.net.use-tls.use-tls)

**Locations:**
- `go/main.go`:22:2

---

*Report generated on 2026-11-24 15:30:30 UTC*