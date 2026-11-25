{%- import "macros.md" as sm -%}

{%- block header -%}
{%- if with_emoji -%}
# 🛡️ SARIF Security Report
{%- else -%}
# SARIF Security Report
{%- endif -%}
{%- endblock -%}

{%- block content -%}
{% for run in runs %}
{% if is_gfm -%}

{{ run.tool_name }}{% if let Some(version) = run.tool_version %} v{{ version }}{% endif %}
{%- else -%}
## {{ run.tool_name }}{% if let Some(version) = run.tool_version %} v{{ version }}{% endif %}
{%- endif %}

### {% if with_emoji %}📊{% endif %} Summary

| Severity | Count |
|----------|{% if is_gfm %}------:{% else %}-------|{% endif %}
{%- for severity in run.severity_counts %}
| {{ sm::format_severity(severity.level, with_emoji) }} | {{ severity.count }} |
{%- endfor %}
| **Total** | **{{ run.total_results }}** |
{%- if run.suppressed_count > 0 %}

**Suppressed:** {{ run.suppressed_count }} (not counted above{% if run.include_suppressed %}; listed below and marked as suppressed{% else %}; not shown{% endif %})
{%- endif %}

---

### {% if with_emoji %}🐛{% endif %} Detailed Results

{%- for result in run.results %}
{% if is_gfm -%}
<details>
<summary>
{{ sm::format_severity(result.level, with_emoji) }} [{{ result.rule_id }}]
{%- if let Some(metadata) = result.rule_metadata %}
{%- if let Some(name) = metadata.name %} - {{ name }}{%- endif %}
{%- endif %}
{%- if result.suppressed %} (suppressed){%- endif %}
</summary>

#### Details
{%- else %}
#### {{ sm::format_severity(result.level, with_emoji) }} - {{ result.rule_id }}{% if result.suppressed %} (suppressed){% endif %}
{%- endif %}

{%- if result.suppressed %}

**Suppressed:** {{ result.suppressions|join(", ") }}
{%- endif %}

**Message:**

{{ result.message.fence }}text
{{ result.message.text }}
{{ result.message.fence }}

{%- if let Some(metadata) = result.rule_metadata %}

{%- if let Some(description) = metadata.description %}

**Description:**

{{ description.fence }}text
{{ description.text }}
{{ description.fence }}
{%- endif %}

{%- if metadata.cwe_ids.len() > 0 %}

**CWE IDs:**
{%- for cwe in metadata.cwe_ids %}
- `{{ cwe }}`
{%- endfor %}
{%- endif %}

{%- if metadata.cve_ids.len() > 0 %}

**CVE IDs:**
{%- for cve in metadata.cve_ids %}
- `{{ cve }}`
{%- endfor %}
{%- endif %}

{%- if metadata.tags.len() > 0 %}

**Tags:**
{%- for tag in metadata.tags %}
- `{{ tag }}`
{%- endfor %}
{%- endif %}

{%- if let Some(help_uri) = metadata.help_uri %}

**Documentation:** [View details]({{ help_uri }})
{%- endif %}

{%- endif %}

{%- if let Some(properties) = result.properties %}

{%- if let Some(confidence) = properties.issue_confidence %}

**Issue Confidence:** {{ confidence }}
{%- endif %}

{%- if let Some(precision) = properties.precision %}

**Precision:** {{ precision }}
{%- endif %}

{%- if let Some(problem_severity) = properties.problem_severity %}

**Problem Severity:** {{ problem_severity }}
{%- endif %}

{%- if properties.custom_fields.len() > 0 %}

**Additional Properties:**
{%- for (key, value) in properties.custom_fields %}
- **{{ key }}:** {{ value }}
  {%- endfor %}
  {%- endif %}
{%- endif %}

{%- if result.locations.len() > 0 %}

**Locations:**
{%- for location in result.locations %}
- {{ sm::format_location(location) }}
{%- endfor %}
{%- endif %}

{%- if is_gfm %}
</details>
{%- endif %}
{%- endfor %}

---
{%- endfor %}
{%- endblock %}

{%- if is_gfm %}

<sub>*Report generated on {{ timestamp }}*</sub>
{%- else %}

*Report generated on {{ timestamp }}*
{%- endif %}