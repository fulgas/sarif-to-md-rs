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
</summary>

#### Details
{% else -%}
#### {{ sm::format_severity(result.level, with_emoji) }} - {{ result.rule_id }}
{%- endif %}

{%- if is_gfm %}
**Message:**
> {{ result.message }}
{% else %}
**Message:** {{ result.message }}
{% endif %}

{%- if let Some(metadata) = result.rule_metadata %}

{%- if let Some(description) = metadata.description %}
{%- if is_gfm %}
**Description:**
> {{ description }}
{%- else %}
**Description:** {{ description }}
{%- endif %}
{% endif %}

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
{%- if is_gfm %}
```
{%- for location in result.locations %}
{{ sm::format_location(location) }}
{%- endfor %}
```
{%- else %}
{%- for location in result.locations %}
- {{ sm::format_location(location) }}
  {%- endfor %}
  {%- endif %}
  {%- endif %}

{%- if is_gfm -%}
</details>
{%- endif %}
{%- endfor %}

{%- if is_gfm -%}
</details>
{%- endif %}

---

{%- endfor %}
{%- endblock %}

{% if is_gfm -%}
<sub>*Report generated on {{ timestamp }}*</sub>
{%- else -%}
*Report generated on {{ timestamp }}*
{%- endif -%}