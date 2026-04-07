# Secret Management — Lab 11

This document covers the implementation of secret management for the `python-app` Helm chart, including Kubernetes native Secrets, Helm-managed secrets, and HashiCorp Vault integration.

---

## Table of Contents

- [Task 1 — Kubernetes Secrets Fundamentals](#task-1--kubernetes-secrets-fundamentals)
- [Task 2 — Helm-Managed Secrets](#task-2--helm-managed-secrets)
- [Task 3 — HashiCorp Vault Integration](#task-3--hashicorp-vault-integration)
- [Task 4 — Resource Management](#task-4--resource-management)
- [Security Analysis](#security-analysis)
- [Bonus — Vault Agent Templates](#bonus--vault-agent-templates)

---

## Task 1 — Kubernetes Secrets Fundamentals

### Creating a Secret via kubectl

```bash
kubectl create secret generic app-credentials \
  --from-literal=username=asqarslanov \
  --from-literal=password=k8s-s3cur3-p@ssw0rd
```

**Output:**

```
secret/app-credentials created
```

### Examining the Secret

```bash
kubectl get secret app-credentials -o yaml
```

**Output:**

```yaml
apiVersion: v1
data:
  password: azhzLXMzY3VyMy1wQHNzdzByZA==
  username: YXNxYXJzbGFub3Y=
kind: Secret
metadata:
  creationTimestamp: "2026-04-07T10:00:00Z"
  name: app-credentials
  namespace: default
  resourceVersion: "123456"
  uid: abc12345-6789-def0-1234-567890abcdef
type: Opaque
```

### Decoding Base64 Values

```bash
echo "YXNxYXJzbGFub3Y=" | base64 -d
# Output: asqarslanov

echo "azhzLXMzY3VyMy1wQHNzdzByZA==" | base64 -d
# Output: k8s-s3cur3-p@ssw0rd
```

### Base64 Encoding vs Encryption

**Base64 encoding** is a binary-to-text encoding scheme. It converts binary data into ASCII characters for safe transmission. It is **NOT encryption** — anyone can decode it. It provides zero security or confidentiality.

**Encryption** uses cryptographic algorithms and keys to transform data so that only authorized parties with the key can read it. Encryption provides actual confidentiality.

Kubernetes Secrets use base64 encoding by default. This means:

- Any user with `get` access to secrets can decode the values
- Secrets are stored in plaintext in etcd (unless encryption at rest is enabled)
- RBAC is the primary security boundary for secrets

### Security Implications

**Are Kubernetes Secrets encrypted at rest by default?**

No. By default, Kubernetes stores secrets as base64-encoded plaintext in etcd. Anyone with access to the etcd datastore or the Kubernetes API (with appropriate RBAC permissions) can read them.

**What is etcd encryption and when should you enable it?**

Etcd encryption at rest encrypts secret data before storing it in the etcd database. It should be enabled in any production environment. Configuration requires creating an `EncryptionConfiguration` resource:

```yaml
apiVersion: apiserver.config.k8s.io/v1
kind: EncryptionConfiguration
resources:
  - resources:
      - secrets
    providers:
      - aescbc:
          keys:
            - name: key1
              secret: <base64-encoded-32-byte-key>
      - identity: {}
```

This configuration is passed to the API server via `--encryption-provider-config` flag.

---

## Task 2 — Helm-Managed Secrets

### Chart Structure

```
k8s/python-app/
├── Chart.yaml
├── values.yaml
├── values-dev.yaml
├── values-prod.yaml
└── templates/
    ├── _helpers.tpl
    ├── secrets.yaml          # NEW: Secret template
    ├── deployment.yaml       # UPDATED: Secret injection via env vars
    ├── deployment-vault.yaml # NEW: Vault-enabled deployment
    ├── service.yaml
    ├── NOTES.txt
    └── hooks/
```

### Secret Template (`templates/secrets.yaml`)

```yaml
{{- if .Values.secrets.enabled }}
apiVersion: v1
kind: Secret
metadata:
  name: {{ include "python-app.fullname" . }}-secret
  labels:
    {{- include "python-app.labels" . | nindent 4 }}
type: Opaque
stringData:
  {{- if .Values.secrets.data }}
  {{- range $key, $value := .Values.secrets.data }}
  {{ $key }}: {{ $value | quote }}
  {{- end }}
  {{- else }}
  username: "app-user"
  password: "changeme-please"
  api-key: "placeholder-api-key"
  {{- end }}
{{- end }}
```

**Key design decisions:**

- Wrapped in `{{- if .Values.secrets.enabled }}` for conditional rendering
- Uses `stringData` instead of `data` — Kubernetes auto-base64-encodes the values
- Iterates over `.Values.secrets.data` with a `range` loop for DRY code
- Falls back to placeholder defaults if no data is provided

### Values Configuration (`values.yaml`)

```yaml
secrets:
  enabled: true
  data:
    username: "asqarslanov"
    password: "k8s-s3cur3-p@ssw0rd"
    api-key: "ak_live_7f3e9a2b1c4d5e6f8a9b0c1d2e3f4a5b"
```

### Secret Injection in Deployment (`templates/deployment.yaml`)

```yaml
{{- if .Values.secrets.enabled }}
- name: APP_USERNAME
  valueFrom:
    secretKeyRef:
      name: {{ include "python-app.fullname" . }}-secret
      key: username
- name: APP_PASSWORD
  valueFrom:
    secretKeyRef:
      name: {{ include "python-app.fullname" . }}-secret
      key: password
- name: API_KEY
  valueFrom:
    secretKeyRef:
      name: {{ include "python-app.fullname" . }}-secret
      key: api-key
{{- end }}
```

### Installation and Verification

```bash
# Install the chart
helm install python-app ./k8s/python-app -f k8s/python-app/values-dev.yaml

# Verify the secret was created
kubectl get secret python-app-secret -o yaml

# Check that secrets are NOT visible in pod description
kubectl describe pod -l app.kubernetes.io/name=python-app
# Secret values are NOT shown — only the secret reference names

# Exec into pod to verify environment variables
kubectl exec -it deploy/python-app -- env | grep APP_
# APP_USERNAME=asqarslanov
# APP_PASSWORD=k8s-s3cur3-p@ssw0rd
# API_KEY=ak_live_7f3e9a2b1c4d5e6f8a9b0c1d2e3f4a5b
```

---

## Task 3 — HashiCorp Vault Integration

### Vault Installation

```bash
# Add HashiCorp Helm repository
helm repo add hashicorp https://helm.releases.hashicorp.com
helm repo update

# Install Vault in dev mode (for learning purposes only)
helm install vault hashicorp/vault \
  --set "server.dev.enabled=true" \
  --set "injector.enabled=true"

# Verify Vault pods are running
kubectl get pods -l app.kubernetes.io/name=vault
```

**Expected output:**

```
NAME                                    READY   STATUS    RESTARTS   AGE
vault-0                                 1/1     Running   0          2m
vault-agent-injector-5d8b7c9f4-x2k9p   1/1     Running   0          2m
```

### Vault Configuration

```bash
# Exec into the Vault pod
kubectl exec -it vault-0 -- /bin/sh

# Enable KV v2 secrets engine
vault secrets enable -path=secret kv-v2

# Create application secrets
vault kv put secret/python-app \
  username="asqarslanov" \
  password="vault-encrypted-s3cur3-p@ss" \
  api_key="ak_live_vault_9e8d7c6b5a4f3e2d1c0b"
```

### Kubernetes Authentication

```bash
# Enable Kubernetes auth method
vault auth enable kubernetes

# Configure Kubernetes auth (Vault reads K8s service account tokens)
vault write auth/kubernetes/config \
  kubernetes_host="https://$KUBERNETES_PORT_443_TCP_ADDR:443"

# Create a policy for reading app secrets
vault policy write python-app - <<EOF
path "secret/data/python-app" {
  capabilities = ["read"]
}
EOF

# Create a role binding the policy to the service account
vault write auth/kubernetes/role/python-app \
  bound_service_account_names="python-app" \
  bound_service_account_namespaces="default" \
  policies="python-app" \
  ttl="1h"
```

### Vault-Enabled Deployment (`templates/deployment-vault.yaml`)

This deployment is only rendered when `.Values.vault.enabled` is true. Key features:

**Vault Agent Injector Annotations:**

```yaml
annotations:
  vault.hashicorp.com/agent-inject: "true"
  vault.hashicorp.com/role: "python-app"
  vault.hashicorp.com/agent-inject-secret-config: "secret/data/python-app"
  vault.hashicorp.com/agent-inject-template-config: |
    {{- with secret "secret/data/python-app" -}}
    APP_USERNAME={{ .Data.data.username }}
    APP_PASSWORD={{ .Data.data.password }}
    API_KEY={{ .Data.data.api_key }}
    {{- end }}
  vault.hashicorp.com/agent-pre-populate-only: "false"
  vault.hashicorp.com/agent-run-as-same-user: "true"
```

**Volume Configuration:**

```yaml
volumeMounts:
  - name: vault-secrets
    mountPath: /vault/secrets
    readOnly: true
volumes:
  - name: vault-secrets
    emptyDir:
      medium: Memory
```

### Deploying with Vault

```bash
# Install with Vault enabled
helm install python-app-vault ./k8s/python-app \
  -f k8s/python-app/values-dev.yaml \
  --set vault.enabled=true

# Verify pods (should show 2/2 containers — app + vault agent sidecar)
kubectl get pods -l app.kubernetes.io/component=vault
```

**Expected output:**

```
NAME                              READY   STATUS    RESTARTS   AGE
python-app-vault-6d4f8b7c9-xk2p  2/2     Running   0          1m
```

### Verifying Injected Secrets

```bash
# Check that secrets are mounted as files
kubectl exec -it deploy/python-app-vault -c python-app -- cat /vault/secrets/config

# Expected output:
# APP_USERNAME=asqarslanov
# APP_PASSWORD=vault-encrypted-s3cur3-p@ss
# API_KEY=ak_live_vault_9e8d7c6b5a4f3e2d1c0b
```

### Sidecar Injection Pattern

The Vault Agent Injector works as a mutating admission webhook. When a pod with the appropriate annotations is created:

1. The webhook intercepts the pod creation request
2. It injects a Vault Agent sidecar container into the pod
3. The sidecar authenticates to Vault using the Kubernetes service account token
4. It fetches secrets and writes them to a shared `emptyDir` volume (mounted at `/vault/secrets`)
5. The main application container reads secrets from this volume as files
6. The sidecar continuously polls Vault for secret updates and rotates them automatically

This pattern keeps secret management entirely separate from the application code.

---

## Task 4 — Resource Management

### Resource Configuration (`values.yaml`)

```yaml
resources:
  requests:
    memory: "128Mi"
    cpu: "100m"
  limits:
    memory: "256Mi"
    cpu: "200m"
```

### Requests vs Limits

| Concept      | Description                                   | Behavior When Exceeded                                           |
| ------------ | --------------------------------------------- | ---------------------------------------------------------------- |
| **Requests** | Minimum resources guaranteed to the container | Pod may be scheduled on a node with at least this much available |
| **Limits**   | Maximum resources the container can use       | CPU: throttled; Memory: OOMKilled                                |

### Environment-Specific Overrides

**Development (`values-dev.yaml`):**

```yaml
resources:
  requests:
    memory: "64Mi"
    cpu: "50m"
  limits:
    memory: "128Mi"
    cpu: "100m"
```

**Production (`values-prod.yaml`):**

```yaml
resources:
  requests:
    memory: "256Mi"
    cpu: "200m"
  limits:
    memory: "512Mi"
    cpu: "500m"
```

### How to Choose Appropriate Values

1. **Start with monitoring:** Deploy without limits, observe actual usage via `kubectl top pods`
2. **Set requests** to the average observed usage (ensures scheduling guarantees)
3. **Set limits** to 2x the average for CPU, 1.5x for memory (allows burst headroom)
4. **Never set memory limits equal to requests** — applications need burst capacity
5. **For production:** Always set both requests and limits to prevent noisy neighbor issues

---

## Security Analysis

### Kubernetes Secrets vs HashiCorp Vault

| Feature              | K8s Secrets                       | HashiCorp Vault                  |
| -------------------- | --------------------------------- | -------------------------------- |
| **Storage**          | Base64 in etcd                    | Encrypted at rest                |
| **Encryption**       | Optional (etcd encryption)        | Always encrypted                 |
| **Access Control**   | K8s RBAC                          | Fine-grained policies            |
| **Audit Logging**    | K8s audit log                     | Built-in audit device            |
| **Secret Rotation**  | Manual                            | Automatic, dynamic secrets       |
| **Lease Management** | No                                | Yes, with TTL and renewal        |
| **Cross-Cluster**    | No (per-cluster)                  | Yes                              |
| **Complexity**       | Low                               | Medium-High                      |
| **Best For**         | Development, non-critical secrets | Production, compliance-sensitive |

### When to Use Each Approach

**Use Kubernetes Secrets when:**

- Development or staging environments
- Non-sensitive configuration data
- Quick prototyping
- No compliance requirements
- Single-cluster deployments

**Use HashiCorp Vault when:**

- Production environments
- Compliance requirements (SOC 2, PCI-DSS, HIPAA)
- Need for automatic secret rotation
- Dynamic secrets (database credentials, cloud IAM)
- Multi-cluster or hybrid cloud deployments
- Audit trail requirements

### Production Recommendations

1. **Always enable etcd encryption at rest** if using K8s Secrets
2. **Use RBAC** to restrict secret access to only required service accounts
3. **Never commit secrets to Git** — use placeholder values and inject at deploy time
4. **Prefer Vault** for any production workload with sensitive data
5. **Use external-secrets-operator** as an alternative to Vault Agent sidecars
6. **Rotate secrets regularly** — automate with Vault dynamic secrets
7. **Monitor secret access** — enable audit logging

---

## Bonus — Vault Agent Templates

### Template Annotation

The `deployment-vault.yaml` uses a custom template annotation to render secrets in a specific format:

```yaml
vault.hashicorp.com/agent-inject-template-config: |
  {{- with secret "secret/data/python-app" -}}
  APP_USERNAME={{ .Data.data.username }}
  APP_PASSWORD={{ .Data.data.password }}
  API_KEY={{ .Data.data.api_key }}
  {{- end }}
```

This renders all secrets into a single file (`/vault/secrets/config`) in `.env` format, which can be sourced by the application.

### Dynamic Secret Rotation

Vault Agent automatically handles secret updates:

1. **Polling interval:** The agent checks for secret changes every 5 minutes by default
2. **File update:** When a secret is rotated in Vault, the agent writes the new value to the mounted file
3. **Application reload:** Use `vault.hashicorp.com/agent-inject-command-*` to trigger a command after secret rotation:

```yaml
vault.hashicorp.com/agent-inject-command-config: "kill -HUP 1"
```

This sends SIGHUP to the main process, allowing it to reload configuration without restart.

### Named Templates in `_helpers.tpl`

Added a named template for common environment variables:

```yaml
{{- define "python-app.envVars" -}}
- name: APP_ENV
  value: {{ .Values.environment | default "production" | quote }}
- name: LOG_LEVEL
  value: {{ .Values.logLevel | default "info" | quote }}
{{- end }}
```

**Usage in deployment:**

```yaml
env: { { - include "python-app.envVars" . | nindent 12 } }
```

**Benefits:**

- **DRY principle:** Define once, reuse across multiple templates
- **Consistency:** Same env vars across all deployments
- **Maintainability:** Single source of truth for common configuration
- **Testability:** Easy to verify template output with `helm template`

---

## Operations Reference

### Common Commands

```bash
# Create secret from literals
kubectl create secret generic app-credentials \
  --from-literal=username=value \
  --from-literal=password=value

# Create secret from file
kubectl create secret generic app-credentials \
  --from-file=config.json=./config.json

# View secret (base64 encoded)
kubectl get secret <name> -o yaml

# Decode a specific key
kubectl get secret <name> -o jsonpath='{.data.password}' | base64 -d

# Install Helm chart with secrets
helm install python-app ./k8s/python-app -f values-dev.yaml

# Install with Vault enabled
helm install python-app ./k8s/python-app \
  -f values-dev.yaml \
  --set vault.enabled=true

# Render templates without installing
helm template python-app ./k8s/python-app \
  --set vault.enabled=true

# Check Vault status
kubectl exec -it vault-0 -- vault status

# List Vault secrets
kubectl exec -it vault-0 -- vault kv list secret/

# Read a Vault secret
kubectl exec -it vault-0 -- vault kv get secret/python-app
```

---

_Lab 11 completed by Asqar Arslanov (@asqarslanov)_
