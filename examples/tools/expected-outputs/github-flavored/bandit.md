# 🛡️ SARIF Security Report
Bandit

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 1 |
| 🟠 - Warning | 2 |
| 🟡 - Note | 2 |
| **Total** | **5** |

---

### 🐛 Detailed Results
<details>
<summary>
🟡 - Note [B404] - blacklist
</summary>

#### Details

**Message:**
> Consider possible security implications associated with the subprocess module.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/blacklists/blacklist_imports.html#b404-import-subprocess)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** LOW

**Locations:**
- `app.py`:2
</details>
<details>
<summary>
🟡 - Note [B105] - hardcoded_password_string
</summary>

#### Details

**Message:**
> Possible hardcoded password: 'hunter2-not-a-real-password'

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b105_hardcoded_password_string.html)

**Issue Confidence:** MEDIUM

**Additional Properties:**
- **issue_severity:** LOW

**Locations:**
- `app.py`:6
</details>
<details>
<summary>
🟠 - Warning [B608] - hardcoded_sql_expressions
</summary>

#### Details

**Message:**
> Possible SQL injection vector through string-based query construction.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b608_hardcoded_sql_expressions.html)

**Issue Confidence:** LOW

**Additional Properties:**
- **issue_severity:** MEDIUM

**Locations:**
- `app.py`:11
</details>
<details>
<summary>
🔴 - Error [B602] - subprocess_popen_with_shell_equals_true
</summary>

#### Details

**Message:**
> subprocess call with shell=True identified, security issue.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b602_subprocess_popen_with_shell_equals_true.html)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** HIGH

**Locations:**
- `app.py`:16
</details>
<details>
<summary>
🟠 - Warning [B506] - yaml_load
</summary>

#### Details

**Message:**
> Use of unsafe yaml load. Allows instantiation of arbitrary objects. Consider yaml.safe_load().

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b506_yaml_load.html)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** MEDIUM

**Locations:**
- `app.py`:20
</details>

---

<sub>*Report generated on 2026-11-24 15:53:41 UTC*</sub>