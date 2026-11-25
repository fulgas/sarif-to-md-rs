# 🛡️ SARIF Security Report
Hadolint v2.14.0

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 1 |
| 🟠 - Warning | 4 |
| 🟡 - Note | 2 |
| **Total** | **7** |

---

### 🐛 Detailed Results
<details>
<summary>
🟠 - Warning [DL3007]
</summary>

#### Details

**Message:**

```text
Using latest is prone to errors if the image will ever update. Pin the version explicitly to a release tag
```

**Locations:**
- `Dockerfile`:1:1
</details>
<details>
<summary>
🟠 - Warning [DL3008]
</summary>

#### Details

**Message:**

```text
Pin versions in apt get install. Instead of `apt-get install <package>` use `apt-get install <package>=<version>`
```

**Locations:**
- `Dockerfile`:2:1
</details>
<details>
<summary>
🟡 - Note [DL3015]
</summary>

#### Details

**Message:**

```text
Avoid additional packages by specifying `--no-install-recommends`
```

**Locations:**
- `Dockerfile`:2:1
</details>
<details>
<summary>
🟡 - Note [DL3009]
</summary>

#### Details

**Message:**

```text
Delete the apt lists (/var/lib/apt/lists) after installing something
```

**Locations:**
- `Dockerfile`:2:1
</details>
<details>
<summary>
🔴 - Error [DL3020]
</summary>

#### Details

**Message:**

```text
Use COPY instead of ADD for files and folders
```

**Locations:**
- `Dockerfile`:3:1
</details>
<details>
<summary>
🟠 - Warning [DL3042]
</summary>

#### Details

**Message:**

```text
Avoid use of cache directory with pip. Use `pip install --no-cache-dir <package>`
```

**Locations:**
- `Dockerfile`:5:1
</details>
<details>
<summary>
🟠 - Warning [DL3025]
</summary>

#### Details

**Message:**

```text
Use arguments JSON notation for CMD and ENTRYPOINT arguments
```

**Locations:**
- `Dockerfile`:6:1
</details>

---

<sub>*Report generated on 1970-01-01 00:00:00 UTC*</sub>