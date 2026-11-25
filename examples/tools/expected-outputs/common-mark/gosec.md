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

**Message:**

```text
SQL injection via taint analysis
```

**Description:**

```text
SQL injection via taint analysis
```

**Tags:**
- `security`
- `HIGH`

**Locations:**
- `main.go`:14:22

#### Error - G702

**Message:**

```text
Command injection via taint analysis
```

**Description:**

```text
Command injection via taint analysis
```

**Tags:**
- `security`
- `HIGH`

**Locations:**
- `main.go`:16:19

#### Error - G705

**Message:**

```text
XSS via taint analysis
```

**Description:**

```text
XSS via taint analysis
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:17:14

#### Error - G401

**Message:**

```text
Use of weak cryptographic primitive
```

**Description:**

```text
Use of weak cryptographic primitive
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:17:24

#### Error - G114

**Message:**

```text
Use of net/http serve function that has no support for setting timeouts
```

**Description:**

```text
Use of net/http serve function that has no support for setting timeouts
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:22:2

#### Error - G204

**Message:**

```text
Subprocess launched with variable
```

**Description:**

```text
Use of net/http serve function that has no support for setting timeouts
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:16:7

#### Error - G202

**Message:**

```text
SQL string concatenation
```

**Description:**

```text
Use of net/http serve function that has no support for setting timeouts
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:14:23

#### Error - G501

**Message:**

```text
Blocklisted import crypto/md5: weak cryptographic primitive
```

**Description:**

```text
Use of weak cryptographic primitive
```

**Tags:**
- `security`
- `MEDIUM`

**Locations:**
- `main.go`:4:2

#### Warning - G104

**Message:**

```text
Errors unhandled
```

**Description:**

```text
Errors unhandled
```

**Tags:**
- `security`
- `LOW`

**Locations:**
- `main.go`:22:2

---

*Report generated on 1970-01-01 00:00:00 UTC*