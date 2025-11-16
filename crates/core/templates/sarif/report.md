{%- import "macros.md" as sm -%}

{%- block header -%}
{%- if with_emoji -%}
# 🛡️ Security Vulnerability Report (SARIF)
{%- else -%}
# Security Vulnerability Report (SARIF)
{%- endif -%}
{%- endblock -%}

{%- block content -%}
{% for run in runs %}
{% if is_gfm -%}
<details open>
<summary><h2>Run {{ loop.index }} - {{ run.tool_name }}</h2></summary>
{%- else -%}
## Run {{ loop.index }} - {{ run.tool_name }}
{%- endif %}

{% if let Some(tool_version) = run.tool_version %}
**Version:** {% if is_gfm %}`{{ tool_version }}`{% else %}{{ tool_version }}{% endif %}
{%- endif %}

### {% if with_emoji %}📊{% endif %} Summary

| Severity | Count |
|----------|{% if is_gfm %}------:{% else %}-------|{% endif %}
{%- for severity in run.severity_counts %}
| {% call sm::format_severity(severity.level, with_emoji) %} | {{ severity.count }} |
{%- endfor %}
| **Total** | **{{ run.total_results }}** |

---

### {% if with_emoji %}🐛{% endif %} Detailed Results

{%- for result in run.results %}
{% if is_gfm -%}
<details>
<summary>
{% call sm::format_severity(result.level, with_emoji) %} <strong>[{{ result.rule_id }}]((https://security.snyk.io/vuln/{{ result.rule_id }})</strong>
{%- if let Some(metadata) = result.rule_metadata %}
{%- if let Some(name) = metadata.name %} - {{ name }}{%- endif %}
{%- endif %}
</summary>

#### Details
{% else -%}
#### {% call sm::format_severity(result.level, with_emoji) %} - [{{ result.rule_id }}](https://security.snyk.io/vuln/{{ result.rule_id }})
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

{%- if result.locations.len() > 0 %}

**Locations:**
{%- if is_gfm %}
```
{%- for location in result.locations %}
{% call sm::format_location(location) %}
{%- endfor %}
```
{%- else %}
{%- for location in result.locations %}
- {% call sm::format_location(location) %}
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