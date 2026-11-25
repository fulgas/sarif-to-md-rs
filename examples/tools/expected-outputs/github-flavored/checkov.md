# 🛡️ SARIF Security Report
Checkov v3.2.490

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 25 |
| **Total** | **25** |

---

### 🐛 Detailed Results
<details>
<summary>
🔴 - Error [CKV_AWS_24] - Ensure no security groups allow ingress from 0.0.0.0:0 to port 22
</summary>

#### Details

**Message:**

```text
Ensure no security groups allow ingress from 0.0.0.0:0 to port 22
```

**Description:**

```text
Ensure no security groups allow ingress from 0.0.0.0:0 to port 22
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/aws-policies/aws-networking-policies/networking-1-port-security)

**Locations:**
- `main.tf`:6
</details>
<details>
<summary>
🔴 - Error [CKV_AWS_23] - Ensure every security group and rule has a description
</summary>

#### Details

**Message:**

```text
Ensure every security group and rule has a description
```

**Description:**

```text
Ensure every security group and rule has a description
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/aws-policies/aws-networking-policies/networking-31)

**Locations:**
- `main.tf`:6
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_37] - Minimize the admission of containers with capabilities assigned
</summary>

#### Details

**Message:**

```text
Minimize the admission of containers with capabilities assigned
```

**Description:**

```text
Minimize the admission of containers with capabilities assigned
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-34)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_31] - Ensure that the seccomp profile is set to docker/default or runtime/default
</summary>

#### Details

**Message:**

```text
Ensure that the seccomp profile is set to docker/default or runtime/default
```

**Description:**

```text
Ensure that the seccomp profile is set to docker/default or runtime/default
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-29)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_8] - Liveness Probe Should be Configured
</summary>

#### Details

**Message:**

```text
Liveness Probe Should be Configured
```

**Description:**

```text
Liveness Probe Should be Configured
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-7)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_12] - Memory requests should be set
</summary>

#### Details

**Message:**

```text
Memory requests should be set
```

**Description:**

```text
Memory requests should be set
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-11)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_20] - Containers should not run with allowPrivilegeEscalation
</summary>

#### Details

**Message:**

```text
Containers should not run with allowPrivilegeEscalation
```

**Description:**

```text
Containers should not run with allowPrivilegeEscalation
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-19)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_13] - Memory limits should be set
</summary>

#### Details

**Message:**

```text
Memory limits should be set
```

**Description:**

```text
Memory limits should be set
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-12)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_40] - Containers should run as a high UID to avoid host conflict
</summary>

#### Details

**Message:**

```text
Containers should run as a high UID to avoid host conflict
```

**Description:**

```text
Containers should run as a high UID to avoid host conflict
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-37)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_10] - CPU requests should be set
</summary>

#### Details

**Message:**

```text
CPU requests should be set
```

**Description:**

```text
CPU requests should be set
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-9)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_16] - Container should not be privileged
</summary>

#### Details

**Message:**

```text
Container should not be privileged
```

**Description:**

```text
Container should not be privileged
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-15)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_22] - Use read-only filesystem for containers where possible
</summary>

#### Details

**Message:**

```text
Use read-only filesystem for containers where possible
```

**Description:**

```text
Use read-only filesystem for containers where possible
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-21)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_9] - Readiness Probe Should be Configured
</summary>

#### Details

**Message:**

```text
Readiness Probe Should be Configured
```

**Description:**

```text
Readiness Probe Should be Configured
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-8)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_28] - Minimize the admission of containers with the NET_RAW capability
</summary>

#### Details

**Message:**

```text
Minimize the admission of containers with the NET_RAW capability
```

**Description:**

```text
Minimize the admission of containers with the NET_RAW capability
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-27)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_29] - Apply security context to your pods and containers
</summary>

#### Details

**Message:**

```text
Apply security context to your pods and containers
```

**Description:**

```text
Apply security context to your pods and containers
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/ensure-securitycontext-is-applied-to-pods-and-containers)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_14] - Image Tag should be fixed - not latest or blank
</summary>

#### Details

**Message:**

```text
Image Tag should be fixed - not latest or blank
```

**Description:**

```text
Image Tag should be fixed - not latest or blank
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-13)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_38] - Ensure that Service Account Tokens are only mounted where necessary
</summary>

#### Details

**Message:**

```text
Ensure that Service Account Tokens are only mounted where necessary
```

**Description:**

```text
Ensure that Service Account Tokens are only mounted where necessary
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-35)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_21] - The default namespace should not be used
</summary>

#### Details

**Message:**

```text
The default namespace should not be used
```

**Description:**

```text
The default namespace should not be used
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-20)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_23] - Minimize the admission of root containers
</summary>

#### Details

**Message:**

```text
Minimize the admission of root containers
```

**Description:**

```text
Minimize the admission of root containers
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-22)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_43] - Image should use digest
</summary>

#### Details

**Message:**

```text
Image should use digest
```

**Description:**

```text
Image should use digest
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-39)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_K8S_11] - CPU limits should be set
</summary>

#### Details

**Message:**

```text
CPU limits should be set
```

**Description:**

```text
CPU limits should be set
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/kubernetes-policies/kubernetes-policy-index/bc-k8s-10)

**Locations:**
- `k8s/deployment.yaml`:1
</details>
<details>
<summary>
🔴 - Error [CKV_DOCKER_7] - Ensure the base image uses a non latest version tag
</summary>

#### Details

**Message:**

```text
Ensure the base image uses a non latest version tag
```

**Description:**

```text
Ensure the base image uses a non latest version tag
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/docker-policies/docker-policy-index/ensure-the-base-image-uses-a-non-latest-version-tag)

**Locations:**
- `Dockerfile`:1
</details>
<details>
<summary>
🔴 - Error [CKV_DOCKER_4] - Ensure that COPY is used instead of ADD in Dockerfiles
</summary>

#### Details

**Message:**

```text
Ensure that COPY is used instead of ADD in Dockerfiles
```

**Description:**

```text
Ensure that COPY is used instead of ADD in Dockerfiles
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/docker-policies/docker-policy-index/ensure-that-copy-is-used-instead-of-add-in-dockerfiles)

**Locations:**
- `Dockerfile`:3
</details>
<details>
<summary>
🔴 - Error [CKV_DOCKER_2] - Ensure that HEALTHCHECK instructions have been added to container images
</summary>

#### Details

**Message:**

```text
Ensure that HEALTHCHECK instructions have been added to container images
```

**Description:**

```text
Ensure that HEALTHCHECK instructions have been added to container images
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/docker-policies/docker-policy-index/ensure-that-healthcheck-instructions-have-been-added-to-container-images)

**Locations:**
- `Dockerfile`:1
</details>
<details>
<summary>
🔴 - Error [CKV_DOCKER_3] - Ensure that a user for the container has been created
</summary>

#### Details

**Message:**

```text
Ensure that a user for the container has been created
```

**Description:**

```text
Ensure that a user for the container has been created
```

**Documentation:** [View details](https://docs.prismacloud.io/en/enterprise-edition/policy-reference/docker-policies/docker-policy-index/ensure-that-a-user-for-the-container-has-been-created)

**Locations:**
- `Dockerfile`:1
</details>

---

<sub>*Report generated on 1970-01-01 00:00:00 UTC*</sub>