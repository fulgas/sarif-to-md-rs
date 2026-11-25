# 🛡️ SARIF Security Report
gosec vdev

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 8 |
| 🟠 - Warning | 1 |
| **Total** | **9** |

---

### 🐛 Detailed Results
<details>
<summary>
🔴 - Error [G701] - G701
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G702] - G702
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G705] - G705
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G401] - Use of Weak Hash
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G114] - Use of Potentially Dangerous Function
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G204] - Use of Potentially Dangerous Function
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G202] - Use of Potentially Dangerous Function
</summary>

#### Details

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
</details>
<details>
<summary>
🔴 - Error [G501] - Use of Weak Hash
</summary>

#### Details

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
</details>
<details>
<summary>
🟠 - Warning [G104] - Improper Check or Handling of Exceptional Conditions
</summary>

#### Details

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
</details>

---

<sub>*Report generated on 1970-01-01 00:00:00 UTC*</sub>