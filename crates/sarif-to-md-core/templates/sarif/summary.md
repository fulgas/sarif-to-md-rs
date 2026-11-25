{%- import "macros.md" as sm -%}
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
