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
> SQL injection via taint analysis

**Description:**
> SQL injection via taint analysis

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
> Command injection via taint analysis

**Description:**
> Command injection via taint analysis

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
> XSS via taint analysis

**Description:**
> XSS via taint analysis

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
> Use of weak cryptographic primitive

**Description:**
> Use of weak cryptographic primitive

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
> Use of net/http serve function that has no support for setting timeouts

**Description:**
> Use of net/http serve function that has no support for setting timeouts

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
> Subprocess launched with variable

**Description:**
> Use of net/http serve function that has no support for setting timeouts

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
> SQL string concatenation

**Description:**
> Use of net/http serve function that has no support for setting timeouts

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
> Blocklisted import crypto/md5: weak cryptographic primitive

**Description:**
> Use of weak cryptographic primitive

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
> Errors unhandled

**Description:**
> Errors unhandled

**Tags:**
- `security`
- `LOW`

**Locations:**
- `main.go`:22:2
</details>
</details>

---

<sub>*Report generated on 2026-11-24 15:30:20 UTC*</sub>