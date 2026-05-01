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

{{- define "python-app.waitForServiceHost" -}}
{{- if .Values.initContainers.waitForService.host }}
{{- .Values.initContainers.waitForService.host }}
{{- else }}
{{- printf "%s.%s.svc.cluster.local" (include "python-app.fullname" .) .Release.Namespace }}
{{- end }}
{{- end }}

{{- define "python-app.initContainers" -}}
- name: init-download
  image: {{ .Values.initContainers.download.image | quote }}
  command:
    - sh
    - -c
    - >-
      wget -q -O /work-dir/{{ .Values.initContainers.download.filename }}
      {{ .Values.initContainers.download.url | quote }}
      && echo "downloaded to /work-dir/{{ .Values.initContainers.download.filename }}"
  volumeMounts:
    - name: init-workdir
      mountPath: /work-dir/
- name: init-wait-service
  image: {{ .Values.initContainers.waitForService.image | quote }}
  command:
    - sh
    - -c
    - |
      set -eu
      HOST="{{ include "python-app.waitForServiceHost" . }}"
      echo "waiting for DNS: ${HOST}"
      until nslookup "${HOST}" >/dev/null 2>&1; do sleep 2; done
      echo "DNS ready for ${HOST}"
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
