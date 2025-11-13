{%- extends "base.md" -%}
{%- import "macros.md" as m -%}
{%- block header -%}
{%- if with_emoji -%}
# 🛡️ Security Vulnerability Report
{%- else -%}
# Security Vulnerability Report
{%- endif -%}
{%- endblock -%}

{%- block content -%}
{%- for project in projects -%}
## {% call m::format_project_type(project.name, project.project_type, with_emoji) %}

**Organization:** {{ project.organization }}
**Target:** `{{ project.target_file }}`

### {% if with_emoji %}📊{% endif %} Vulnerability Summary

| Severity                                                             | Count                                  |
|----------------------------------------------------------------------|----------------------------------------|
| {% call m::format_severity(ReportSeverity::Critical, with_emoji) %}  | {{ project.summary.critical }}         |
| {% call m::format_severity(ReportSeverity::High, with_emoji) %}      | {{ project.summary.high }}             |
| {% call m::format_severity(ReportSeverity::Medium, with_emoji) %}    | {{ project.summary.medium }}           |
| {% call m::format_severity(ReportSeverity::Low, with_emoji) %}       | {{ project.summary.low }}              |
| **Total Unique**                                                     | **{{ project.summary.unique_count }}** |

---

### {% if with_emoji %}🐛{% endif %} Detailed Vulnerabilities

{% for vuln in project.vulnerabilities %}
{%- include "vulnerability.md" %}
{% endfor %}
---

{% endfor %}
{% endblock %}