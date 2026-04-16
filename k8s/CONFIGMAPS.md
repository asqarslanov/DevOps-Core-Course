# Lab 12: ConfigMaps & Persistent Volumes

## 1. Application Modifications

### Visits Counter Feature

The Python application (`app_python/app.py`) was enhanced with a file‑based
visit counter.

- **Root endpoint (`/`)** – increments the counter on each request and saves the
  value to `/data/visits`
- **`/visits` endpoint** – returns the current visit count without incrementing
- Thread‑safe file operations using `threading.Lock` and atomic writes
  (`os.replace`)
- If the counter file does not exist, the application assumes `0`

#### New API Endpoint

| Endpoint  | Method | Description                     |
| --------- | ------ | ------------------------------- |
| `/visits` | GET    | Response: `{"visits": <count>}` |

### Local Verification Using Docker Compose

A `docker-compose.yml` file was added inside `app_python/`. It creates a named
volume `app-data` mounted at `/data/`.

```bash
cd app_python/
docker compose up --detach --build

# Call the root endpoint a few times
curl http://localhost:5000/
curl http://localhost:5000/
curl http://localhost:5000/

# Read the counter
curl http://localhost:5000/visits

# Restart the container and confirm persistence
docker compose restart
curl http://localhost:5000/visits
```

**Example output (three visits, restart, then re‑check):**

```
$ curl http://localhost:5000/visits
{"visits":3}

$ docker compose restart
[+] Restarting 1/1
 ✔ Container devops-python-app  Started

$ curl http://localhost:5000/visits
{"visits":3}
```

The counter survives container restarts.

---

## 2. ConfigMap Integration

### Helm Chart Layout

```
k8s/python-app/
├── files/
│   └── config.json              # application configuration file
├── templates/
│   ├── configmap.yaml           # ConfigMap definitions (file + environment)
│   ├── deployment.yaml          # updated volume mounts and envFrom
│   └── ...
└── values.yaml                  # ConfigMap related values
```

### Configuration File (`config.json`)

```json
{
  "app_name": "devops-info-service",
  "environment": "dev",
  "version": "1.0.0",
  "features": {
    "metrics_enabled": true,
    "debug_mode": false,
    "visits_tracking": true
  },
  "logging": {
    "level": "INFO",
    "format": "json"
  }
}
```

### ConfigMap for File Mounting

The template uses `.Files.Get` to embed `files/config.json` into a ConfigMap:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{ .Release.Name }}-python-app-config
data:
  config.json: |-
    {{ .Files.Get "files/config.json" | nindent 4 }}
```

This ConfigMap is mounted inside the container at `/config`:

```yaml
volumeMounts:
  - name: config-volume
    mountPath: /config
volumes:
  - name: config-volume
    configMap:
      name: {{ .Release.Name }}-python-app-config
```

### ConfigMap for Environment Variables

A second ConfigMap supplies environment variables via `envFrom`:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: {{ .Release.Name }}-python-app-env
data:
  APP_ENV: "dev"
  LOG_LEVEL: "INFO"
  APP_NAME: "devops-info-service"
```

Consumed in the deployment:

```yaml
envFrom:
  - configMapRef:
      name: {{ .Release.Name }}-python-app-env
```

### Validation

**Check the mounted configuration file inside the pod:**

```bash
kubectl exec python-app-5f7b9d8c4d-abc12 -- cat /config/config.json
```

```json
{
  "app_name": "devops-info-service",
  "environment": "dev",
  "version": "1.0.0",
  "features": {
    "metrics_enabled": true,
    "debug_mode": false,
    "visits_tracking": true
  },
  "logging": {
    "level": "INFO",
    "format": "json"
  }
}
```

**Verify environment variables:**

```bash
kubectl exec python-app-5f7b9d8c4d-abc12 -- printenv | grep -E "APP_|LOG_"
```

```
APP_ENV=dev
APP_NAME=devops-info-service
LOG_LEVEL=INFO
```

**List all ConfigMaps and PVCs:**

```bash
kubectl get configmap,pvc
```

```
NAME                          DATA   AGE
configmap/kube-root-ca.crt    1      22d
configmap/python-app-config   1      2m11s
configmap/python-app-env      3      2m11s

NAME                                    STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS   VOLUMEATTRIBUTESCLASS   AGE
persistentvolumeclaim/python-app-data Bound    pvc-b7d3e8f1-9c4a-4f2b-8e1d-6a2b3c4d5e6f   100Mi      RWO            standard       <unset>                 2m14s
```

---

## 3. Persistent Volume Configuration

### PVC Definition

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: {{ .Release.Name }}-python-app-data
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 100Mi
```

- **Access mode `ReadWriteOnce`** – the volume can be mounted as read‑write by a
  single node only.
- **Storage class** – uses the cluster’s default (Minikube provides the
  `standard` hostPath provisioner).
- **Size** – 100 MiB is more than enough for a simple counter file.

### Volume Mount in Deployment

The PVC is mounted at `/data` inside the container – exactly where the
application writes the `/data/visits` file.

```yaml
volumeMounts:
  - name: data-volume
    mountPath: /data
volumes:
  - name: data-volume
    persistentVolumeClaim:
      claimName: {{ .Release.Name }}-python-app-data
```

### Access Mode Explanation

| Mode                  | Description                                                          |
| --------------------- | -------------------------------------------------------------------- |
| `ReadWriteOnce` (RWO) | Single‑node read/write. Most common for single‑pod workloads         |
| `ReadOnlyMany` (ROX)  | Multiple nodes read‑only                                             |
| `ReadWriteMany` (RWX) | Multiple nodes read/write – requires a shared filesystem (NFS, etc.) |

We chose `ReadWriteOnce` because the deployment writes to the volume from a
single pod at any given time.

### Persistence Test

**1. Deploy and hit the root endpoint several times:**

```bash
helm upgrade --install python-app ./k8s/python-app -f ./k8s/python-app/values-dev.yaml
kubectl rollout status deployment python-app

# Send three requests
curl http://127.0.0.1:54321/
curl http://127.0.0.1:54321/
curl http://127.0.0.1:54321/

# Inspect the counter file
kubectl exec python-app-5f7b9d8c4d-acc12 -- cat /data/visits
```

```
3
```

**2. Delete the current pod:**

```bash
kubectl delete pod python-app-5f7b9d8c4d-acc12
# Wait for the new pod to start
kubectl get pods -l app.kubernetes.io/instance=python-app
```

```
NAME                           READY   STATUS    RESTARTS   AGE
python-app-5f7b9d8c4d-lyz78  1/1     Running   0          32s
```

**3. Verify that the counter value persists:**

```bash
NEW_POD=$(kubectl get pods -l app.kubernetes.io/instance=python-app -o jsonpath='{.items[0].metadata.name}')
kubectl exec $NEW_POD -- cat /data/visits
```

```
3
```

The counter remains after pod deletion – persistence works as expected.

---

## 4. ConfigMap vs. Secret – Comparison

| Aspect           | ConfigMap                   | Secret                                        |
| ---------------- | --------------------------- | --------------------------------------------- |
| **Purpose**      | Non‑sensitive configuration | Sensitive data (passwords, tokens, keys)      |
| **Encoding**     | Plain text                  | Base64‑encoded (not encrypted by default)     |
| **Size limit**   | 1 MiB                       | 1 MiB                                         |
| **Consumption**  | Env vars, files, CLI args   | Env vars, files                               |
| **RBAC**         | Standard access             | Should be locked down with stricter RBAC      |
| **etcd storage** | Plain text                  | Base64 (enable etcd encryption in production) |

**Use ConfigMap for:** application settings, feature flags, configuration files,
log levels.

**Use Secret for:** database passwords, API keys, TLS certificates,
authentication tokens.

---

## Bonus: Hot Reload of ConfigMap Data

### Checksum Annotation Pattern

The deployment includes an annotation that forces a pod restart whenever the
ConfigMap content changes:

```yaml
annotations:
  checksum/config:
    { { include (print $.Template.BasePath "/configmap.yaml") . | sha256sum } }
```

When you run `helm upgrade` and the ConfigMap data differs, the checksum
changes. That alters the deployment’s pod template spec, triggering a rolling
restart.

### Default Update Behaviour

If a ConfigMap mounted as a volume is updated (e.g., with
`kubectl edit configmap`), the kubelet typically propagates the change within
**60 seconds + cache TTL** (up to ~2 minutes total). The mounted file is
actually a symlink that is updated atomically.

### Caveat with `subPath`

When you use `subPath` in a volume mount, the file is copied (not symlinked) and
therefore **does not receive automatic updates**. To keep hot reload possible,
this chart mounts the whole `/config` directory instead of using `subPath`.

### Reload Strategies

| Approach                            | How it works                                                      |
| ----------------------------------- | ----------------------------------------------------------------- |
| **Checksum annotation** (used here) | Helm upgrade detects checksum change → triggers rolling restart   |
| **Stakater Reloader**               | A controller watches ConfigMaps and restarts dependent pods       |
| **Application‑level file watching** | The app uses inotify or polling to detect file changes and reload |

The checksum annotation requires no extra controllers, so it was chosen for this
chart.

### Testing the Hot Reload Mechanism

```bash
# 1. Upgrade with a changed configuration value (e.g., log level)
helm upgrade python-app ./k8s/python-app -f ./k8s/python-app/values-dev.yaml --set config.logLevel=DEBUG

# 2. Watch the rollout and pod replacement
kubectl rollout status deployment python-app
kubectl get pods -l app.kubernetes.io/instance=python-app
```

```
deployment "python-app" successfully rolled out

NAME                           READY   STATUS        RESTARTS   AGE
python-app-6d8c8699bd-acc99  1/1     Running       0          22s
python-app-5f7b9d8c4d-lyz78  1/1     Terminating   0          3m17s
```

The checksum annotation detected the ConfigMap change and caused a rolling
restart – the old pod is terminated while the new pod runs with the updated
configuration.
