# SARIF Security Report
## terrascan v1.19.9

###  Summary

| Severity | Count |
|----------|-------|
| Error | 5 |
| Warning | 12 |
| Note | 3 |
| **Total** | **20** |

---

###  Detailed Results

#### Warning - AC_DOCKER_0041

**Message:**

```text
Ensure :latest version is not used for Docker file
```

**Description:**

```text
Ensure :latest version is not used for Docker file
```

**Locations:**
- `file:///iac/Dockerfile`:1

#### Warning - AC_DOCKER_0010

**Message:**

```text
Ensure Pinned Pip Package Version
```

**Description:**

```text
Ensure Pinned Pip Package Version
```

**Locations:**
- `file:///iac/Dockerfile`:5

#### Warning - AC_DOCKER_0031

**Message:**

```text
Ensure when installing packages with pip, the '--no-cache-dir' flag should be set to make Docker images smaller
```

**Description:**

```text
Ensure when installing packages with pip, the '--no-cache-dir' flag should be set to make Docker images smaller
```

**Locations:**
- `file:///iac/Dockerfile`:5

#### Warning - AC_K8S_0080

**Message:**

```text
Default seccomp profile not enabled will make the container to make non-essential system calls
```

**Description:**

```text
Default seccomp profile not enabled will make the container to make non-essential system calls
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0097

**Message:**

```text
CPU Request Not Set in config file.
```

**Description:**

```text
CPU Request Not Set in config file.
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Note - AC_K8S_0072

**Message:**

```text
No readiness probe will affect automatic recovery in case of unexpected errors
```

**Description:**

```text
No readiness probe will affect automatic recovery in case of unexpected errors
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Note - AC_K8S_0070

**Message:**

```text
No liveness probe will ensure there is no recovery in case of unexpected errors
```

**Description:**

```text
No liveness probe will ensure there is no recovery in case of unexpected errors
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Note - AC_K8S_0068

**Message:**

```text
No tag or container image with :Latest tag makes difficult to rollback and track
```

**Description:**

```text
No tag or container image with :Latest tag makes difficult to rollback and track
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0098

**Message:**

```text
CPU Limits Not Set in config file.
```

**Description:**

```text
CPU Limits Not Set in config file.
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0100

**Message:**

```text
Memory Limits Not Set in config file.
```

**Description:**

```text
Memory Limits Not Set in config file.
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0099

**Message:**

```text
Memory Request Not Set in config file.
```

**Description:**

```text
Memory Request Not Set in config file.
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0073

**Message:**

```text
AppArmor profile not set to default or custom profile will make the container vulnerable to kernel level threats
```

**Description:**

```text
AppArmor profile not set to default or custom profile will make the container vulnerable to kernel level threats
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0064

**Message:**

```text
Apply Security Context to Your Pods and Containers
```

**Description:**

```text
Apply Security Context to Your Pods and Containers
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Error - AC_K8S_0085

**Message:**

```text
Containers Should Not Run with AllowPrivilegeEscalation
```

**Description:**

```text
Containers Should Not Run with AllowPrivilegeEscalation
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0069

**Message:**

```text
Image without digest affects the integrity principle of image security
```

**Description:**

```text
Image without digest affects the integrity principle of image security
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Error - AC_K8S_0087

**Message:**

```text
Minimize Admission of Root Containers
```

**Description:**

```text
Minimize Admission of Root Containers
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Warning - AC_K8S_0078

**Message:**

```text
Container images with readOnlyRootFileSystem set as false mounts the container root file system with write permissions
```

**Description:**

```text
Container images with readOnlyRootFileSystem set as false mounts the container root file system with write permissions
```

**Locations:**
- `file:///iac/k8s/deployment.yaml`:1

#### Error - AC_AWS_0227

**Message:**

```text
Security Groups - Unrestricted Specific Ports - (SSH,22)
```

**Description:**

```text
Security Groups - Unrestricted Specific Ports - (SSH,22)
```

**Locations:**
- `file:///iac/main.tf`:6

#### Error - AC_AWS_0214

**Message:**

```text
Enabling S3 versioning will enable easy recovery from both unintended user actions, like deletes and overwrites
```

**Description:**

```text
Enabling S3 versioning will enable easy recovery from both unintended user actions, like deletes and overwrites
```

**Locations:**
- `file:///iac/main.tf`:1

#### Error - AC_AWS_0210

**Message:**

```text
Misconfigured S3 buckets can leak private information to the entire internet or allow unauthorized data tampering / deletion
```

**Description:**

```text
Misconfigured S3 buckets can leak private information to the entire internet or allow unauthorized data tampering / deletion
```

**Locations:**
- `file:///iac/main.tf`:1

---

*Report generated on 1970-01-01 00:00:00 UTC*