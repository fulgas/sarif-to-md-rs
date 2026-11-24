# 🛡️ SARIF Security Report
Trivy v0.67.2

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 16 |
| 🟠 - Warning | 11 |
| 🟡 - Note | 17 |
| **Total** | **44** |

---

### 🐛 Detailed Results
<details>
<summary>
🔴 - Error [CVE-2020-14343] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: PyYAML  
> Installed Version: 5.3  
> Vulnerability CVE-2020-14343  
> Severity: CRITICAL  
> Fixed Version: 5.4  
> Link: [CVE-2020-14343](https://avd.aquasec.com/nvd/cve-2020-14343)

**Description:**
> PyYAML: incomplete fix for CVE-2020-1747

**Tags:**
- `vulnerability`
- `security`
- `CRITICAL`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2020-14343)

**Locations:**
- `requirements.txt`:1:1
</details>
<details>
<summary>
🔴 - Error [CVE-2020-1747] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: PyYAML  
> Installed Version: 5.3  
> Vulnerability CVE-2020-1747  
> Severity: CRITICAL  
> Fixed Version: 5.3.1  
> Link: [CVE-2020-1747](https://avd.aquasec.com/nvd/cve-2020-1747)

**Description:**
> PyYAML: arbitrary command execution through python/object/new when FullLoader is used

**Tags:**
- `vulnerability`
- `security`
- `CRITICAL`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2020-1747)

**Locations:**
- `requirements.txt`:1:1
</details>
<details>
<summary>
🔴 - Error [CVE-2018-18074] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: requests  
> Installed Version: 2.19.0  
> Vulnerability CVE-2018-18074  
> Severity: HIGH  
> Fixed Version: 2.20.0  
> Link: [CVE-2018-18074](https://avd.aquasec.com/nvd/cve-2018-18074)

**Description:**
> python-requests: Redirect from HTTPS to HTTP does not remove Authorization header

**Tags:**
- `vulnerability`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2018-18074)

**Locations:**
- `requirements.txt`:2:1
</details>
<details>
<summary>
🟠 - Warning [CVE-2023-32681] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: requests  
> Installed Version: 2.19.0  
> Vulnerability CVE-2023-32681  
> Severity: MEDIUM  
> Fixed Version: 2.31.0  
> Link: [CVE-2023-32681](https://avd.aquasec.com/nvd/cve-2023-32681)

**Description:**
> python-requests: Unintended leak of Proxy-Authorization header

**Tags:**
- `vulnerability`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2023-32681)

**Locations:**
- `requirements.txt`:2:1
</details>
<details>
<summary>
🟠 - Warning [CVE-2024-35195] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: requests  
> Installed Version: 2.19.0  
> Vulnerability CVE-2024-35195  
> Severity: MEDIUM  
> Fixed Version: 2.32.0  
> Link: [CVE-2024-35195](https://avd.aquasec.com/nvd/cve-2024-35195)

**Description:**
> requests: subsequent requests to the same host ignore cert verification

**Tags:**
- `vulnerability`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2024-35195)

**Locations:**
- `requirements.txt`:2:1
</details>
<details>
<summary>
🟠 - Warning [CVE-2024-47081] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: requests  
> Installed Version: 2.19.0  
> Vulnerability CVE-2024-47081  
> Severity: MEDIUM  
> Fixed Version: 2.32.4  
> Link: [CVE-2024-47081](https://avd.aquasec.com/nvd/cve-2024-47081)

**Description:**
> requests: Requests vulnerable to .netrc credentials leak via malicious URLs

**Tags:**
- `vulnerability`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2024-47081)

**Locations:**
- `requirements.txt`:2:1
</details>
<details>
<summary>
🟠 - Warning [CVE-2026-25645] - LanguageSpecificPackageVulnerability
</summary>

#### Details

**Message:**
> Package: requests  
> Installed Version: 2.19.0  
> Vulnerability CVE-2026-25645  
> Severity: MEDIUM  
> Fixed Version: 2.33.0  
> Link: [CVE-2026-25645](https://avd.aquasec.com/nvd/cve-2026-25645)

**Description:**
> requests: Requests: Security bypass due to predictable temporary file creation

**Tags:**
- `vulnerability`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/nvd/cve-2026-25645)

**Locations:**
- `requirements.txt`:2:1
</details>
<details>
<summary>
🟠 - Warning [DS001] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: Dockerfile  
> Type: dockerfile  
> Vulnerability DS001  
> Severity: MEDIUM  
> Message: Specify a tag in the 'FROM' statement for image 'python'  
> Link: [DS001](https://avd.aquasec.com/misconfig/ds001)

**Description:**
> ':latest' tag used

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ds001)

**Locations:**
- `Dockerfile`:1:1
</details>
<details>
<summary>
🔴 - Error [DS002] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: Dockerfile  
> Type: dockerfile  
> Vulnerability DS002  
> Severity: HIGH  
> Message: Specify at least 1 USER command in Dockerfile with non-root user as argument  
> Link: [DS002](https://avd.aquasec.com/misconfig/ds002)

**Description:**
> Image user should not be 'root'

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ds002)

**Locations:**
- `Dockerfile`:1:1
</details>
<details>
<summary>
🟡 - Note [DS005] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: Dockerfile  
> Type: dockerfile  
> Vulnerability DS005  
> Severity: LOW  
> Message: Consider using 'COPY . /app' command instead of 'ADD . /app'  
> Link: [DS005](https://avd.aquasec.com/misconfig/ds005)

**Description:**
> ADD instead of COPY

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ds005)

**Locations:**
- `Dockerfile`:3:1
</details>
<details>
<summary>
🟡 - Note [DS026] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: Dockerfile  
> Type: dockerfile  
> Vulnerability DS026  
> Severity: LOW  
> Message: Add HEALTHCHECK instruction in your Dockerfile  
> Link: [DS026](https://avd.aquasec.com/misconfig/ds026)

**Description:**
> No HEALTHCHECK defined

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ds026)

**Locations:**
- `Dockerfile`:1:1
</details>
<details>
<summary>
🔴 - Error [DS029] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: Dockerfile  
> Type: dockerfile  
> Vulnerability DS029  
> Severity: HIGH  
> Message: '--no-install-recommends' flag is missed: 'apt-get update && apt-get install -y curl'  
> Link: [DS029](https://avd.aquasec.com/misconfig/ds029)

**Description:**
> 'apt-get' missing '--no-install-recommends'

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ds029)

**Locations:**
- `Dockerfile`:2:1
</details>
<details>
<summary>
🟠 - Warning [KSV001] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV001  
> Severity: MEDIUM  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.allowPrivilegeEscalation' to false  
> Link: [KSV001](https://avd.aquasec.com/misconfig/ksv001)

**Description:**
> Can elevate its own privileges

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv001)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV003] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV003  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should add 'ALL' to 'securityContext.capabilities.drop'  
> Link: [KSV003](https://avd.aquasec.com/misconfig/ksv003)

**Description:**
> Default capabilities: some containers do not drop all

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv003)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV004] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV004  
> Severity: LOW  
> Message: Container 'app' of 'deployment' 'app' in 'default' namespace should set securityContext.capabilities.drop  
> Link: [KSV004](https://avd.aquasec.com/misconfig/ksv004)

**Description:**
> Default capabilities: some containers do not drop any

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv004)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV011] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV011  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'resources.limits.cpu'  
> Link: [KSV011](https://avd.aquasec.com/misconfig/ksv011)

**Description:**
> CPU not limited

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv011)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟠 - Warning [KSV012] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV012  
> Severity: MEDIUM  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.runAsNonRoot' to true  
> Link: [KSV012](https://avd.aquasec.com/misconfig/ksv012)

**Description:**
> Runs as root user

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv012)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟠 - Warning [KSV013] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV013  
> Severity: MEDIUM  
> Message: Container 'app' of Deployment 'app' should specify an image tag  
> Link: [KSV013](https://avd.aquasec.com/misconfig/ksv013)

**Description:**
> Image tag ":latest" used

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv013)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🔴 - Error [KSV014] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV014  
> Severity: HIGH  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.readOnlyRootFilesystem' to true  
> Link: [KSV014](https://avd.aquasec.com/misconfig/ksv014)

**Description:**
> Root file system is not read-only

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv014)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV015] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV015  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'resources.requests.cpu'  
> Link: [KSV015](https://avd.aquasec.com/misconfig/ksv015)

**Description:**
> CPU requests not specified

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv015)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV016] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV016  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'resources.requests.memory'  
> Link: [KSV016](https://avd.aquasec.com/misconfig/ksv016)

**Description:**
> Memory requests not specified

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv016)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🔴 - Error [KSV017] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV017  
> Severity: HIGH  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.privileged' to false  
> Link: [KSV017](https://avd.aquasec.com/misconfig/ksv017)

**Description:**
> Privileged

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv017)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV018] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV018  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'resources.limits.memory'  
> Link: [KSV018](https://avd.aquasec.com/misconfig/ksv018)

**Description:**
> Memory not limited

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv018)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV020] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV020  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.runAsUser' > 10000  
> Link: [KSV020](https://avd.aquasec.com/misconfig/ksv020)

**Description:**
> Runs with UID <= 10000

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv020)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV021] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV021  
> Severity: LOW  
> Message: Container 'app' of Deployment 'app' should set 'securityContext.runAsGroup' > 10000  
> Link: [KSV021](https://avd.aquasec.com/misconfig/ksv021)

**Description:**
> Runs with GID <= 10000

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv021)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV030] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV030  
> Severity: LOW  
> Message: Either Pod or Container should set 'securityContext.seccompProfile.type' to 'RuntimeDefault'  
> Link: [KSV030](https://avd.aquasec.com/misconfig/ksv030)

**Description:**
> Runtime/Default Seccomp profile not set

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv030)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟠 - Warning [KSV104] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV104  
> Severity: MEDIUM  
> Message: container "app" of deployment "app" in "default" namespace should specify a seccomp profile  
> Link: [KSV104](https://avd.aquasec.com/misconfig/ksv104)

**Description:**
> Seccomp policies disabled

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv104)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV106] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV106  
> Severity: LOW  
> Message: container should drop all  
> Link: [KSV106](https://avd.aquasec.com/misconfig/ksv106)

**Description:**
> Container capabilities must only include NET_BIND_SERVICE

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv106)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🟡 - Note [KSV110] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV110  
> Severity: LOW  
> Message: deployment app in default namespace should set metadata.namespace to a non-default namespace  
> Link: [KSV110](https://avd.aquasec.com/misconfig/ksv110)

**Description:**
> Workloads in the default namespace

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv110)

**Locations:**
- `k8s/deployment.yaml`:4:1
</details>
<details>
<summary>
🔴 - Error [KSV118] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV118  
> Severity: HIGH  
> Message: deployment app in default namespace is using the default security context, which allows root privileges  
> Link: [KSV118](https://avd.aquasec.com/misconfig/ksv118)

**Description:**
> Default security context configured

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv118)

**Locations:**
- `k8s/deployment.yaml`:15:1
</details>
<details>
<summary>
🟠 - Warning [KSV0125] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: k8s/deployment.yaml  
> Type: kubernetes  
> Vulnerability KSV0125  
> Severity: MEDIUM  
> Message: Container app in deployment app (namespace: default) uses an image from an untrusted registry.  
> Link: [KSV0125](https://avd.aquasec.com/misconfig/ksv0125)

**Description:**
> Restrict container images to trusted registries

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/ksv0125)

**Locations:**
- `k8s/deployment.yaml`:16:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0086] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0086  
> Severity: HIGH  
> Message: No public access block so not blocking public acls  
> Link: [AVD-AWS-0086](https://avd.aquasec.com/misconfig/avd-aws-0086)

**Description:**
> S3 Access block should block public ACL

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0086)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0087] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0087  
> Severity: HIGH  
> Message: No public access block so not blocking public policies  
> Link: [AVD-AWS-0087](https://avd.aquasec.com/misconfig/avd-aws-0087)

**Description:**
> S3 Access block should block public policy

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0087)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0088] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0088  
> Severity: HIGH  
> Message: Bucket does not have encryption enabled  
> Link: [AVD-AWS-0088](https://avd.aquasec.com/misconfig/avd-aws-0088)

**Description:**
> Unencrypted S3 bucket.

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0088)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🟡 - Note [s3-bucket-logging] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability s3-bucket-logging  
> Severity: LOW  
> Message: Bucket has logging disabled  
> Link: [s3-bucket-logging](https://avd.aquasec.com/misconfig/s3-bucket-logging)

**Description:**
> S3 Bucket Logging

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/s3-bucket-logging)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🟠 - Warning [AVD-AWS-0090] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0090  
> Severity: MEDIUM  
> Message: Bucket does not have versioning enabled  
> Link: [AVD-AWS-0090](https://avd.aquasec.com/misconfig/avd-aws-0090)

**Description:**
> S3 Data should be versioned

**Tags:**
- `misconfiguration`
- `security`
- `MEDIUM`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0090)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0091] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0091  
> Severity: HIGH  
> Message: No public access block so not blocking public acls  
> Link: [AVD-AWS-0091](https://avd.aquasec.com/misconfig/avd-aws-0091)

**Description:**
> S3 Access Block should Ignore Public ACL

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0091)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0092] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0092  
> Severity: HIGH  
> Message: Bucket has a public ACL: "public-read"  
> Link: [AVD-AWS-0092](https://avd.aquasec.com/misconfig/avd-aws-0092)

**Description:**
> S3 Buckets not publicly accessible through ACL.

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0092)

**Locations:**
- `main.tf`:3:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0093] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0093  
> Severity: HIGH  
> Message: No public access block so not restricting public buckets  
> Link: [AVD-AWS-0093](https://avd.aquasec.com/misconfig/avd-aws-0093)

**Description:**
> S3 Access block should restrict public bucket to limit access

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0093)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🟡 - Note [AVD-AWS-0094] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0094  
> Severity: LOW  
> Message: Bucket does not have a corresponding public access block.  
> Link: [AVD-AWS-0094](https://avd.aquasec.com/misconfig/avd-aws-0094)

**Description:**
> S3 buckets should each define an aws_s3_bucket_public_access_block

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0094)

**Locations:**
- `main.tf`:1:1
</details>
<details>
<summary>
🟡 - Note [aws-vpc-add-description-to-security-group] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability aws-vpc-add-description-to-security-group  
> Severity: LOW  
> Message: Security group explicitly uses the default description.  
> Link: [aws-vpc-add-description-to-security-group](https://avd.aquasec.com/misconfig/aws-vpc-add-description-to-security-group)

**Description:**
> Missing description for security group.

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/aws-vpc-add-description-to-security-group)

**Locations:**
- `main.tf`:6:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0107] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0107  
> Severity: HIGH  
> Message: Security group rule allows unrestricted ingress from any IP address.  
> Link: [AVD-AWS-0107](https://avd.aquasec.com/misconfig/avd-aws-0107)

**Description:**
> Security groups should not allow unrestricted ingress to SSH or RDP from any IP address.

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0107)

**Locations:**
- `main.tf`:12:1
</details>
<details>
<summary>
🟡 - Note [aws-vpc-add-description-to-security-group-rule] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability aws-vpc-add-description-to-security-group-rule  
> Severity: LOW  
> Message: Security group rule does not have a description.  
> Link: [aws-vpc-add-description-to-security-group-rule](https://avd.aquasec.com/misconfig/aws-vpc-add-description-to-security-group-rule)

**Description:**
> Missing description for security group rule.

**Tags:**
- `misconfiguration`
- `security`
- `LOW`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/aws-vpc-add-description-to-security-group-rule)

**Locations:**
- `main.tf`:8:1
</details>
<details>
<summary>
🔴 - Error [AVD-AWS-0132] - Misconfiguration
</summary>

#### Details

**Message:**
> Artifact: main.tf  
> Type: terraform  
> Vulnerability AVD-AWS-0132  
> Severity: HIGH  
> Message: Bucket does not encrypt data with a customer managed key.  
> Link: [AVD-AWS-0132](https://avd.aquasec.com/misconfig/avd-aws-0132)

**Description:**
> S3 encryption should use Customer Managed Keys

**Tags:**
- `misconfiguration`
- `security`
- `HIGH`

**Documentation:** [View details](https://avd.aquasec.com/misconfig/avd-aws-0132)

**Locations:**
- `main.tf`:1:1
</details>
</details>

---

<sub>*Report generated on 2026-11-24 15:30:37 UTC*</sub>