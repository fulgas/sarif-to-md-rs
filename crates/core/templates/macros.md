{%- macro severity_emoji(severity) -%}
{%- match severity -%}
{%- when ReportSeverity::Critical -%}🔴
{%- when ReportSeverity::High -%}🟠
{%- when ReportSeverity::Medium -%}🟡
{%- when ReportSeverity::Low -%}🟢
{%- endmatch -%}
{%- endmacro -%}

{%- macro project_type_emoji(project_type) -%}
{%- match project_type -%}
{%- when ReportProjectType::DockerImage -%}🐳
{%- when ReportProjectType::Application -%}📦
{%- endmatch -%}
{%- endmacro -%}

{%- macro format_severity(severity, with_emoji) -%}
{%- if with_emoji -%}
{% call severity_emoji(severity) %} {{ severity }}
{%- else -%}
{{ severity }}
{%- endif -%}
{%- endmacro -%}

{%- macro format_project_type(project_name, project_type, with_emoji) -%}
{%- if with_emoji -%}
{% call project_type_emoji(project_type) %} {{ project_name }}
{%- else -%}
{{ project_name }}
{%- endif -%}
{%- endmacro -%}