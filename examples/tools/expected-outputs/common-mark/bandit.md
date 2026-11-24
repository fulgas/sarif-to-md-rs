# SARIF Security Report
## Bandit

###  Summary

| Severity | Count |
|----------|-------|
| Error | 1 |
| Warning | 2 |
| Note | 2 |
| **Total** | **5** |

---

###  Detailed Results

#### Note - B404

**Message:** Consider possible security implications associated with the subprocess module.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/blacklists/blacklist_imports.html#b404-import-subprocess)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** LOW

**Locations:**
- `app.py`:2

#### Note - B105

**Message:** Possible hardcoded password: 'hunter2-not-a-real-password'

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b105_hardcoded_password_string.html)

**Issue Confidence:** MEDIUM

**Additional Properties:**
- **issue_severity:** LOW

**Locations:**
- `app.py`:6

#### Warning - B608

**Message:** Possible SQL injection vector through string-based query construction.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b608_hardcoded_sql_expressions.html)

**Issue Confidence:** LOW

**Additional Properties:**
- **issue_severity:** MEDIUM

**Locations:**
- `app.py`:11

#### Error - B602

**Message:** subprocess call with shell=True identified, security issue.

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b602_subprocess_popen_with_shell_equals_true.html)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** HIGH

**Locations:**
- `app.py`:16

#### Warning - B506

**Message:** Use of unsafe yaml load. Allows instantiation of arbitrary objects. Consider yaml.safe_load().

**Documentation:** [View details](https://bandit.readthedocs.io/en/1.9.4/plugins/b506_yaml_load.html)

**Issue Confidence:** HIGH

**Additional Properties:**
- **issue_severity:** MEDIUM

**Locations:**
- `app.py`:20

---

*Report generated on 1970-01-01 00:00:00 UTC*