# 🛡️ SARIF Security Report
terrascan v1.19.9

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 5 |
| 🟠 - Warning | 12 |
| 🟡 - Note | 3 |
| **Total** | **20** |

---

### 🐛 Detailed Results
<details>
<summary>
🟠 - Warning [AC_DOCKER_0041] - imageVersionnotusinglatest
</summary>

#### Details

**Message:**
> Ensure :latest version is not used for Docker file

**Description:**
> Ensure :latest version is not used for Docker file

**Locations:**
- `file:///iac/Dockerfile`:1
</details>
<details>
<summary>
🟠 - Warning [AC_DOCKER_0010] - pipUnpinnedVersion
</summary>

#### Details

**Message:**
> Ensure Pinned Pip Package Version

**Description:**
> Ensure Pinned Pip Package Version

**Locations:**
- `file:///iac/Dockerfile`:5
</details>
<details>
<summary>
🟠 - Warning [AC_DOCKER_0031] - pipInstallKeepingCachedPackages
</summary>

#### Details

**Message:**
> Ensure when installing packages with pip, the '--no-cache-dir' flag should be set to make Docker images smaller

**Description:**
> Ensure when installing packages with pip, the '--no-cache-dir' flag should be set to make Docker images smaller

**Locations:**
- `file:///iac/Dockerfile`:5
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0080] - secCompProfile
</summary>

#### Details

**Message:**
> Default seccomp profile not enabled will make the container to make non-essential system calls

**Description:**
> Default seccomp profile not enabled will make the container to make non-essential system calls

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0097] - CpuRequestsCheck
</summary>

#### Details

**Message:**
> CPU Request Not Set in config file.

**Description:**
> CPU Request Not Set in config file.

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟡 - Note [AC_K8S_0072] - noReadinessProbe
</summary>

#### Details

**Message:**
> No readiness probe will affect automatic recovery in case of unexpected errors

**Description:**
> No readiness probe will affect automatic recovery in case of unexpected errors

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟡 - Note [AC_K8S_0070] - nolivenessProbe
</summary>

#### Details

**Message:**
> No liveness probe will ensure there is no recovery in case of unexpected errors

**Description:**
> No liveness probe will ensure there is no recovery in case of unexpected errors

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟡 - Note [AC_K8S_0068] - imageWithLatestTag
</summary>

#### Details

**Message:**
> No tag or container image with :Latest tag makes difficult to rollback and track

**Description:**
> No tag or container image with :Latest tag makes difficult to rollback and track

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0098] - CpulimitsCheck
</summary>

#### Details

**Message:**
> CPU Limits Not Set in config file.

**Description:**
> CPU Limits Not Set in config file.

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0100] - MemorylimitsCheck
</summary>

#### Details

**Message:**
> Memory Limits Not Set in config file.

**Description:**
> Memory Limits Not Set in config file.

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0099] - MemoryRequestsCheck
</summary>

#### Details

**Message:**
> Memory Request Not Set in config file.

**Description:**
> Memory Request Not Set in config file.

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0073] - appArmorProfile
</summary>

#### Details

**Message:**
> AppArmor profile not set to default or custom profile will make the container vulnerable to kernel level threats

**Description:**
> AppArmor profile not set to default or custom profile will make the container vulnerable to kernel level threats

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0064] - securityContextUsed
</summary>

#### Details

**Message:**
> Apply Security Context to Your Pods and Containers

**Description:**
> Apply Security Context to Your Pods and Containers

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [AC_K8S_0085] - privilegeEscalationCheck
</summary>

#### Details

**Message:**
> Containers Should Not Run with AllowPrivilegeEscalation

**Description:**
> Containers Should Not Run with AllowPrivilegeEscalation

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0069] - imageWithoutDigest
</summary>

#### Details

**Message:**
> Image without digest affects the integrity principle of image security

**Description:**
> Image without digest affects the integrity principle of image security

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [AC_K8S_0087] - runAsNonRootCheck
</summary>

#### Details

**Message:**
> Minimize Admission of Root Containers

**Description:**
> Minimize Admission of Root Containers

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🟠 - Warning [AC_K8S_0078] - readOnlyFileSystem
</summary>

#### Details

**Message:**
> Container images with readOnlyRootFileSystem set as false mounts the container root file system with write permissions

**Description:**
> Container images with readOnlyRootFileSystem set as false mounts the container root file system with write permissions

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [AC_AWS_0227] - port22OpenToInternet
</summary>

#### Details

**Message:**
> Security Groups - Unrestricted Specific Ports - (SSH,22)

**Description:**
> Security Groups - Unrestricted Specific Ports - (SSH,22)

**Locations:**
- `file:///iac/main.tf`:6
</details>
<details>
<summary>
🔴 - Error [AC_AWS_0214] - s3Versioning
</summary>

#### Details

**Message:**
> Enabling S3 versioning will enable easy recovery from both unintended user actions, like deletes and overwrites

**Description:**
> Enabling S3 versioning will enable easy recovery from both unintended user actions, like deletes and overwrites

**Locations:**
- `file:///iac/main.tf`:1
</details>
<details>
<summary>
🔴 - Error [AC_AWS_0210] - allUsersReadAccess
</summary>

#### Details

**Message:**
> Misconfigured S3 buckets can leak private information to the entire internet or allow unauthorized data tampering / deletion

**Description:**
> Misconfigured S3 buckets can leak private information to the entire internet or allow unauthorized data tampering / deletion

**Locations:**
- `file:///iac/main.tf`:1
</details>
</details>

---

<sub>*Report generated on 2026-11-24 15:30:33 UTC*</sub>