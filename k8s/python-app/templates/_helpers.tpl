{{/*
Bridge to common-lib templates.
All naming and labeling logic is delegated to the shared library chart.
*/}}

{{- define "python-app.name" -}}
{{- include "common.name" . }}
{{- end }}

{{- define "python-app.fullname" -}}
{{- include "common.fullname" . }}
{{- end }}

{{- define "python-app.chart" -}}
{{- include "common.chart" . }}
{{- end }}

{{- define "python-app.labels" -}}
{{- include "common.labels" . }}
{{- end }}

{{- define "python-app.selectorLabels" -}}
{{- include "common.selectorLabels" . }}
{{- end }}

{{/*
Common environment variables for the application.
Usage: {{ include "python-app.envVars" . }}
*/}}
{{- define "python-app.envVars" -}}
- name: APP_ENV
  value: {{ .Values.environment | default "production" | quote }}
- name: LOG_LEVEL
  value: {{ .Values.logLevel | default "info" | quote }}
{{- end }}
