{{/*
Override common templates to use common-lib definitions.
This file bridges the rust-app chart to the common-lib library.
*/}}

{{- define "rust-app.name" -}}
{{- include "common.name" . }}
{{- end }}

{{- define "rust-app.fullname" -}}
{{- include "common.fullname" . }}
{{- end }}

{{- define "rust-app.chart" -}}
{{- include "common.chart" . }}
{{- end }}

{{- define "rust-app.labels" -}}
{{- include "common.labels" . }}
{{- end }}

{{- define "rust-app.selectorLabels" -}}
{{- include "common.selectorLabels" . }}
{{- end }}
