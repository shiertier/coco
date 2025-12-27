{{- define "coco.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end -}}

{{- define "coco.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name (include "coco.name" .) | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}

{{- define "coco.labels" -}}
app.kubernetes.io/name: {{ include "coco.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
app.kubernetes.io/version: {{ .Chart.AppVersion }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end -}}

{{- define "coco.selectorLabels" -}}
app.kubernetes.io/name: {{ include "coco.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{- define "coco.postgresUrl" -}}
{{- if .Values.postgres.enabled -}}
{{- printf "postgres://%s:%s@%s-postgres:%d/%s" .Values.postgres.auth.user .Values.postgres.auth.password (include "coco.fullname" .) .Values.postgres.service.port .Values.postgres.auth.database -}}
{{- else -}}
{{- .Values.config.dbUrl -}}
{{- end -}}
{{- end -}}

{{- define "coco.vectorDbUrl" -}}
{{- if .Values.config.vectorDbUrl -}}
{{- .Values.config.vectorDbUrl -}}
{{- else if .Values.qdrant.enabled -}}
{{- printf "http://%s-qdrant:%d" (include "coco.fullname" .) .Values.qdrant.service.port -}}
{{- end -}}
{{- end -}}
