{%- import "sarif_macros.md" as sm -%}

{%- block header -%}
{%- if with_emoji -%}
# 🛡️ Security Vulnerability Report (SARIF)
{%- else -%}
# Security Vulnerability Report (SARIF)
{%- endif -%}
{%- endblock -%}

{%- block content -%}
{%- for run in runs %}
## Run {{ loop.index }} - {{ run.tool_name }}

{%- if let Some(tool_version) = run.tool_version %}
**Version:** {{ tool_version }}
{%- endif %}

### {% if with_emoji %}📊{% endif %} Summary

| Severity | Count |
|----------|-------|
{%- for severity in run.severity_counts %}
| {% call sm::format_severity(severity.level, with_emoji) %} | {{ severity.count }} |
{%- endfor %}
| **Total** | **{{ run.total_results }}** |

---

### {% if with_emoji %}🐛{% endif %} Detailed Results

{%- for result in run.results %}
{% include "sarif_result.md" %}
{% endfor %}

---

{%- endfor -%}
{%- endblock -%}

---

*Report generated on {{ timestamp }}*