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

{% include "summary.md" %}

---

### {% if with_emoji %}🐛{% endif %} Detailed Results

{%- for result in run.results %}
{% include "result.md" %}
{%- endfor %}

---
{%- endfor %}
{%- endblock %}

{%- if is_gfm %}

<sub>*Report generated on {{ timestamp }}*</sub>
{%- else %}

*Report generated on {{ timestamp }}*
{%- endif %}
