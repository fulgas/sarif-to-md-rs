# 🛡️ SARIF Security Report
defsec

### 📊 Summary

| Severity | Count |
|----------|------:
| 🔴 - Error | 8 |
| 🟠 - Warning | 2 |
| 🟡 - Note | 3 |
| **Total** | **13** |

---

### 🐛 Detailed Results
<details>
<summary>
🟡 - Note [aws-ec2-add-description-to-security-group]
</summary>

#### Details

**Message:**
> Security group explicitly uses the default description.

**Description:**
> Missing description for security group.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/ec2/add-description-to-security-group/)

**Locations:**
- `main.tf`:6
</details>
<details>
<summary>
🟡 - Note [aws-ec2-add-description-to-security-group-rule]
</summary>

#### Details

**Message:**
> Security group rule does not have a description.

**Description:**
> Missing description for security group rule.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/ec2/add-description-to-security-group-rule/)

**Locations:**
- `main.tf`:8
</details>
<details>
<summary>
🔴 - Error [aws-ec2-no-public-ingress-sgr]
</summary>

#### Details

**Message:**
> Security group rule allows ingress from public internet.

**Description:**
> An ingress security group rule allows traffic from /0.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/ec2/no-public-ingress-sgr/)

**Locations:**
- `main.tf`:12
</details>
<details>
<summary>
🔴 - Error [aws-s3-block-public-acls]
</summary>

#### Details

**Message:**
> No public access block so not blocking public acls

**Description:**
> S3 Access block should block public ACL

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/block-public-acls/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🔴 - Error [aws-s3-block-public-policy]
</summary>

#### Details

**Message:**
> No public access block so not blocking public policies

**Description:**
> S3 Access block should block public policy

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/block-public-policy/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🔴 - Error [aws-s3-enable-bucket-encryption]
</summary>

#### Details

**Message:**
> Bucket does not have encryption enabled

**Description:**
> Unencrypted S3 bucket.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/enable-bucket-encryption/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🟠 - Warning [aws-s3-enable-bucket-logging]
</summary>

#### Details

**Message:**
> Bucket does not have logging enabled

**Description:**
> S3 Bucket does not have logging enabled.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/enable-bucket-logging/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🟠 - Warning [aws-s3-enable-versioning]
</summary>

#### Details

**Message:**
> Bucket does not have versioning enabled

**Description:**
> S3 Data should be versioned

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/enable-versioning/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🔴 - Error [aws-s3-encryption-customer-key]
</summary>

#### Details

**Message:**
> Bucket does not encrypt data with a customer managed key.

**Description:**
> S3 encryption should use Customer Managed Keys

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/encryption-customer-key/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🔴 - Error [aws-s3-ignore-public-acls]
</summary>

#### Details

**Message:**
> No public access block so not ignoring public acls

**Description:**
> S3 Access Block should Ignore Public Acl

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/ignore-public-acls/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🔴 - Error [aws-s3-no-public-access-with-acl]
</summary>

#### Details

**Message:**
> Bucket has a public ACL: 'public-read'.

**Description:**
> S3 Buckets not publicly accessible through ACL.

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/no-public-access-with-acl/)

**Locations:**
- `main.tf`:3
</details>
<details>
<summary>
🔴 - Error [aws-s3-no-public-buckets]
</summary>

#### Details

**Message:**
> No public access block so not restricting public buckets

**Description:**
> S3 Access block should restrict public bucket to limit access

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/no-public-buckets/)

**Locations:**
- `main.tf`:1
</details>
<details>
<summary>
🟡 - Note [aws-s3-specify-public-access-block]
</summary>

#### Details

**Message:**
> Bucket does not have a corresponding public access block.

**Description:**
> S3 buckets should each define an aws_s3_bucket_public_access_block

**Documentation:** [View details](https://aquasecurity.github.io/tfsec/v1.28.14/checks/aws/s3/specify-public-access-block/)

**Locations:**
- `main.tf`:1
</details>
</details>

---

<sub>*Report generated on 2026-11-24 15:30:35 UTC*</sub>