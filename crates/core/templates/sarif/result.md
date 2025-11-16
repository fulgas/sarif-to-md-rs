{%- import "macros.md" as sm -%}

#### {% call sm::format_severity(result.level, with_emoji) %} [{{ result.rule_id }}](https://security.snyk.io/vuln/{{ result.rule_id }})

{%- if let Some(metadata) = result.rule_metadata %}

{%- if let Some(name) = metadata.name %}
**Rule:** {{ name }}
{%- endif %}

{%- if let Some(description) = metadata.description %}
**Description:** {{ description }}
{% endif %}

{%- if metadata.cwe_ids.len() > 0 %}
**CWE IDs:**
{%- for cwe in metadata.cwe_ids %}
- [{{ cwe }}](https://cwe.mitre.org/data/definitions/{{ cwe.trim_start_matches("CWE-") }}.html)
{%- endfor %}
{%- endif %}

{%- if metadata.tags.len() > 0 %}
**Tags:**
{%- for tag in metadata.tags %}
- `{{ tag }}` 
{%- endfor %}
{%- endif %}

{%- if let Some(help_uri) = metadata.help_uri %}
**More Info:** [Documentation]({{ help_uri }})
{%- endif %}

{%- endif %}

| Property | Value |
|----------|-------|
| **Level** | {% call sm::format_severity(result.level, with_emoji) %} |

**Message:** {{ result.message }}

{%- if result.locations.len() > 0 %}

**Locations:**
{%- for location in result.locations %}
- {% call sm::format_location(location) %}
{%- endfor %}
{%- endif %}
