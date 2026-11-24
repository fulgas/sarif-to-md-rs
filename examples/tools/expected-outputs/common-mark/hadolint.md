# SARIF Security Report
## Hadolint v2.14.0

###  Summary

| Severity | Count |
|----------|-------|
| Error | 1 |
| Warning | 4 |
| Note | 2 |
| **Total** | **7** |

---

###  Detailed Results

#### Warning - DL3007

**Message:** Using latest is prone to errors if the image will ever update. Pin the version explicitly to a release tag

**Locations:**
- `Dockerfile`:1:1

#### Warning - DL3008

**Message:** Pin versions in apt get install. Instead of `apt-get install <package>` use `apt-get install <package>=<version>`

**Locations:**
- `Dockerfile`:2:1

#### Note - DL3015

**Message:** Avoid additional packages by specifying `--no-install-recommends`

**Locations:**
- `Dockerfile`:2:1

#### Note - DL3009

**Message:** Delete the apt lists (/var/lib/apt/lists) after installing something

**Locations:**
- `Dockerfile`:2:1

#### Error - DL3020

**Message:** Use COPY instead of ADD for files and folders

**Locations:**
- `Dockerfile`:3:1

#### Warning - DL3042

**Message:** Avoid use of cache directory with pip. Use `pip install --no-cache-dir <package>`

**Locations:**
- `Dockerfile`:5:1

#### Warning - DL3025

**Message:** Use arguments JSON notation for CMD and ENTRYPOINT arguments

**Locations:**
- `Dockerfile`:6:1

---

*Report generated on 1970-01-01 00:00:00 UTC*