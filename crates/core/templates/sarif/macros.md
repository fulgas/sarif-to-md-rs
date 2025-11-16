{%- macro severity_emoji(severity) -%}
{%- match severity -%}
{%- when SarifLevel::Error -%}🔴
{%- when SarifLevel::Warning -%}🟠
{%- when SarifLevel::Note -%}🟡
{%- when SarifLevel::None -%}⚪
{%- endmatch -%}
{%- endmacro -%}

{%- macro format_severity(severity, with_emoji) -%}
{%- if with_emoji -%}
{% call severity_emoji(severity) %} - {{ severity }}
{%- else -%}
{{ severity }}
{%- endif -%}
{%- endmacro -%}

{%- macro format_location(location) -%}
{%- if let Some(file) = location.file -%}
`{{ file }}`
{%- if let Some(line) = location.line -%}
:{{ line }}
{%- if let Some(column) = location.column -%}
:{{ column }}
{%- endif -%}
{%- endif -%}
{%- endif -%}
{%- endmacro -%}