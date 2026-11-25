# SARIF Security Report
## KICS v2.2.1

###  Summary

| Severity | Count |
|----------|-------|
| Error | 6 |
| Warning | 16 |
| Note | 12 |
| None | 8 |
| **Total** | **42** |

---

###  Detailed Results

#### Error - 38c5ee0d-7f22-4260-ab72-5073048df100

**Message:**

```text
'acl' is equal 'public-read'
```

**Description:**

```text
S3 Bucket ACL Allows Read Or Write to All Users
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/s3_bucket)

**Locations:**
- `../../path/main.tf`:3

#### Error - dd29336b-fe57-445b-a26e-e6aa867ae609

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.privileged is true
```

**Description:**

```text
Container Is Privileged
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/workloads/pods/#privileged-mode-for-containers)

**Locations:**
- `../../path/k8s/deployment.yaml`:19

#### Error - fd54f200-402c-4333-a5a4-36ef6709af2f

**Message:**

```text
The 'Dockerfile' does not contain any 'USER' instruction
```

**Description:**

```text
Missing User Instruction
```

**Documentation:** [View details](https://docs.docker.com/engine/reference/builder/#user)

**Locations:**
- `../../path/Dockerfile`:1

#### Error - 5572cc5e-1e4c-4113-92a6-7a8a3bd25e6d

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.allowPrivilegeEscalation is undefined
```

**Description:**

```text
Privilege Escalation Allowed
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

**Locations:**
- `../../path/k8s/deployment.yaml`:18

#### Error - 381c3f2a-ef6f-4eff-99f7-b169cda3422c

**Message:**

```text
SSH (TCP:22) is allowed
```

**Description:**

```text
Sensitive Port Is Exposed To Entire Network
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group)

**Locations:**
- `../../path/main.tf`:8

#### Error - 4728cd65-a20c-49da-8b31-9c08b423e4db

**Message:**

```text
aws_security_group[ssh].ingress.cidr_blocks contains '0.0.0.0/0'
```

**Description:**

```text
Unrestricted Security Group Ingress
```

**Documentation:** [View details](https://www.terraform.io/docs/providers/aws/r/security_group.html)

**Locations:**
- `../../path/main.tf`:12

#### Warning - 9513a694-aa0d-41d8-be61-3271e056f36b

**Message:**

```text
'ADD' .
```

**Description:**

```text
Add Instead of Copy
```

**Documentation:** [View details](https://docs.docker.com/engine/reference/builder/#add)

**Locations:**
- `../../path/Dockerfile`:3

#### Warning - 965a08d7-ef86-4f14-8792-4a3b2098937e

**Message:**

```text
Package 'curl' does not have version defined
```

**Description:**

```text
Apt Get Install Pin Version Not Defined
```

**Documentation:** [View details](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)

**Locations:**
- `../../path/Dockerfile`:2

#### Warning - cf34805e-3872-4c08-bf92-6ff7bb0cfadb

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.runAsUser is 0 and 'runAsNonRoot' is false
```

**Description:**

```text
Container Running As Root
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Warning - 02323c00-cdc3-4fdc-a310-4f2b3e7a1660

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.runAsUser is undefined
```

**Description:**

```text
Container Running With Low UID
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Warning - f45ea400-6bbe-4501-9fc7-1c3d75c32067

**Message:**

```text
FROM python:latest'
```

**Description:**

```text
Image Version Using 'latest'
```

**Documentation:** [View details](https://docs.docker.com/develop/dev-best-practices/)

**Locations:**
- `../../path/Dockerfile`:1

#### Warning - b14d1bc4-a208-45db-92f0-e21f8e2588e9

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.resources.limits.memory is undefined
```

**Description:**

```text
Memory Limits Not Defined
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/assign-memory-resource/)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Warning - 229588ef-8fde-40c8-8756-f4f2b5825ded

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.resources.requests.memory is undefined
```

**Description:**

```text
Memory Requests Not Defined
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/assign-memory-resource/)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Warning - dbbc6705-d541-43b0-b166-dd4be8208b54

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.capabilities.drop is undefined
```

**Description:**

```text
NET_RAW Capabilities Not Being Dropped
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

**Locations:**
- `../../path/k8s/deployment.yaml`:18

#### Warning - 34945ae8-730d-424c-8279-b064d9f108de

**Message:**

```text
{{CMD python app.py}} isn't in proper exec form
```

**Description:**

```text
Not Using Proper Exec Form In CMD And ENTRYPOINT Arguments
```

**Documentation:** [View details](https://docs.docker.com/engine/reference/builder/#entrypoint)

**Locations:**
- `../../path/Dockerfile`:6

#### Warning - a659f3b5-9bf0-438a-bd9a-7d3a6427f1e3

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.readinessProbe is undefined
```

**Description:**

```text
Readiness Probe Is Not Configured
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/#define-readiness-probes)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Warning - f861041c-8c9f-4156-acfc-5e6e524f5884

**Message:**

```text
'logging' is undefined or null
```

**Description:**

```text
S3 Bucket Logging Disabled
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/s3_bucket)

**Locations:**
- `../../path/main.tf`:1

#### Warning - 568a4d22-3517-44a6-a7ad-6a7eed88722c

**Message:**

```text
'versioning' is undefined or null
```

**Description:**

```text
S3 Bucket Without Versioning
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/s3_bucket#versioning)

**Locations:**
- `../../path/main.tf`:1

#### Warning - f377b83e-bd07-4f48-a591-60c82b14a78b

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.seccompProfile.type is undefined
```

**Description:**

```text
Seccomp Profile Is Not Configured
```

**Documentation:** [View details](https://kubernetes.io/docs/tutorials/security/seccomp/#create-pod-that-uses-the-container-runtime-default-seccomp-profile)

**Locations:**
- `../../path/k8s/deployment.yaml`:18

#### Warning - 65905cec-d691-4320-b320-2000436cb696

**Message:**

```text
aws_security_group[ssh].ingress 'SSH' (Port:22) is open
```

**Description:**

```text
Security Group With Unrestricted Access To SSH
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group)

**Locations:**
- `../../path/main.tf`:8

#### Warning - 48471392-d4d0-47c0-b135-cdec95eb3eef

**Message:**

```text
metadata.name={{app}}.spec.template.spec.automountServiceAccountToken is undefined
```

**Description:**

```text
Service Account Token Automount Not Disabled
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/#use-the-default-service-account-to-access-the-api-server)

**Locations:**
- `../../path/k8s/deployment.yaml`:14

#### Warning - 611ab018-c4aa-4ba2-b0f6-a448337509a6

**Message:**

```text
metadata.namespace is undefined or null
```

**Description:**

```text
Using Unrecommended Namespace
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/overview/working-with-objects/kubernetes-objects/)

**Locations:**
- `../../path/k8s/deployment.yaml`:4

#### Note - 4ac0e2b7-d2d2-4af7-8799-e8de6721ccda

**Message:**

```text
spec.template.spec.containers.name=app doesn't have resources defined
```

**Description:**

```text
CPU Limits Not Set
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Note - ca469dd4-c736-448f-8ac1-30a642705e0a

**Message:**

```text
spec.template.spec.containers.name=app doesn't have resources defined
```

**Description:**

```text
CPU Requests Not Set
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/#)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### Note - b03a748a-542d-44f4-bb86-9199ab4fd2d5

**Message:**

```text
Dockerfile doesn't contain instruction 'HEALTHCHECK'
```

**Description:**

```text
Healthcheck Instruction Missing
```

**Documentation:** [View details](https://docs.docker.com/engine/reference/builder/#healthcheck)

**Locations:**
- `../../path/Dockerfile`:1

#### Note - e592a0c5-5bdb-414c-9066-5dba7cdea370

**Message:**

```text
'aws_accessanalyzer_analyzer' is undefined
```

**Description:**

```text
IAM Access Analyzer Not Enabled
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/accessanalyzer_analyzer)

**Locations:**
- `../../path/main.tf`:1

#### Note - 7c81d34c-8e5a-402b-9798-9f442630e678

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.image does not include an image digest
```

**Description:**

```text
Image Without Digest
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/containers/images/#updating-images)

**Locations:**
- `../../path/k8s/deployment.yaml`:17

#### Note - 583053b7-e632-46f0-b989-f81ff8045385

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.image tag is not provided or latest
```

**Description:**

```text
Invalid Image Tag
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/containers/images/#updating-images)

**Locations:**
- `../../path/k8s/deployment.yaml`:17

#### Note - 8b36775e-183d-4d46-b0f7-96a6f34a723f

**Message:**

```text
metadata.name={{app}}.spec.template.metadata.annotations does not specify an AppArmor profile for container {{app}}
```

**Description:**

```text
Missing AppArmor Profile
```

**Documentation:** [View details](https://kubernetes.io/docs/tutorials/clusters/apparmor/)

**Locations:**
- `../../path/k8s/deployment.yaml`:11

#### Note - 268ca686-7fb7-4ae9-b129-955a2a89064e

**Message:**

```text
metadata.name={{app}}.spec.containers.name={{app}}.securityContext.capabilities is undefined
```

**Description:**

```text
No Drop Capabilities for Containers
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/workloads/pods/init-containers/)

**Locations:**
- `../../path/k8s/deployment.yaml`:18

#### Note - f2f903fb-b977-461e-98d7-b3e2185c6118

**Message:**

```text
The '--no-cache-dir' flag isn't set when running 'pip/pip3 install'
```

**Description:**

```text
Pip install Keeping Cached Packages
```

**Documentation:** [View details](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)

**Locations:**
- `../../path/Dockerfile`:5

#### Note - 4a20ebac-1060-4c81-95d1-1f7f620e983b

**Message:**

```text
metadata.name={{app}} does not have a 'LimitRange' policy associated
```

**Description:**

```text
Pod or Container Without LimitRange
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/policy/limit-range/)

**Locations:**
- `../../path/k8s/deployment.yaml`:4

#### Note - 48a5beba-e4c0-4584-a2aa-e6894e4cf424

**Message:**

```text
metadata.name={{app}} does not have a 'ResourceQuota' policy associated
```

**Description:**

```text
Pod or Container Without ResourceQuota
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/policy/resource-quotas/)

**Locations:**
- `../../path/k8s/deployment.yaml`:4

#### Note - a9c2f49d-0671-4fc9-9ece-f4e261e128d0

**Message:**

```text
metadata.name={{app}}.spec.template.spec.containers.name={{app}}.securityContext.readOnlyRootFilesystem is undefined
```

**Description:**

```text
Root Container Not Mounted Read-only
```

**Documentation:** [View details](https://kubernetes.io/docs/tasks/configure-pod-container/security-context/)

**Locations:**
- `../../path/k8s/deployment.yaml`:18

#### None - 7384dfb2-fcd1-4fbf-91cd-6c44c318c33c

**Message:**

```text
'RUN apt-get update && apt-get install -y curl' does not use '--no-install-recommends' flag to avoid installing additional packages
```

**Description:**

```text
APT-GET Not Avoiding Additional Packages
```

**Documentation:** [View details](https://docs.docker.com/engine/reference/builder/#run)

**Locations:**
- `../../path/Dockerfile`:2

#### None - df746b39-6564-4fed-bf85-e9c44382303c

**Message:**

```text
After using apt-get install, the apt-get lists were not deleted
```

**Description:**

```text
Apt Get Install Lists Were Not Deleted
```

**Documentation:** [View details](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)

**Locations:**
- `../../path/Dockerfile`:2

#### None - ade74944-a674-4e00-859e-c6eab5bde441

**Message:**

```text
metadata.name={{app}}.spec.containers.name={{app}}.livenessProbe is undefined
```

**Description:**

```text
Liveness Probe Is Not Defined
```

**Documentation:** [View details](https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle/#when-should-you-use-a-liveness-probe)

**Locations:**
- `../../path/k8s/deployment.yaml`:16

#### None - e38a8e0a-b88b-4902-b3fe-b0fcb17d5c10

**Message:**

```text
aws_s3_bucket[{{data}}].tags is undefined or null
```

**Description:**

```text
Resource Not Using Tags
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/guides/resource-tagging)

**Locations:**
- `../../path/main.tf`:1

#### None - e38a8e0a-b88b-4902-b3fe-b0fcb17d5c10

**Message:**

```text
aws_security_group[{{ssh}}].tags is undefined or null
```

**Description:**

```text
Resource Not Using Tags
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/guides/resource-tagging)

**Locations:**
- `../../path/main.tf`:6

#### None - 4849211b-ac39-479e-ae78-5694d506cb24

**Message:**

```text
'aws_security_group[ssh]' is not used
```

**Description:**

```text
Security Group Not Used
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group)

**Locations:**
- `../../path/main.tf`:6

#### None - 68eb4bf3-f9bf-463d-b5cf-e029bb446d2e

**Message:**

```text
aws_security_group[ssh].ingress.description is undefined or null
```

**Description:**

```text
Security Group Rule Without Description
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group#description)

**Locations:**
- `../../path/main.tf`:8

#### None - cb3f5ed6-0d18-40de-a93d-b3538db31e8c

**Message:**

```text
aws_security_group[ssh] description is undefined or null
```

**Description:**

```text
Security Group Without Description
```

**Documentation:** [View details](https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group#description)

**Locations:**
- `../../path/main.tf`:6

---

*Report generated on 1970-01-01 00:00:00 UTC*