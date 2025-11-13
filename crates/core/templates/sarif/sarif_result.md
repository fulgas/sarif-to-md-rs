{%- import "sarif_macros.md" as sm -%}

#### {% call sm::format_severity(result.level, with_emoji) %} {{ result.rule_id }}

| Property | Value |
|----------|-------|
| **Rule ID** | `{{ result.rule_id }}` |
| **Level** | {% call sm::format_severity(result.level, with_emoji) %} |

**Message:** {{ result.message }}

{%- if result.locations.len() > 0 %}

**Locations:**
{%- for location in result.locations %}
- {% call sm::format_location(location) %}
{%- endfor %}
{%- endif %}
