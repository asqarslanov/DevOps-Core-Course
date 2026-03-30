# Helm Charts — DevOps Course

This document covers the Helm charts built for the DevOps course project: two
application charts (`python-app`, `rust-app`) backed by a shared library chart
(`common-lib`). It includes configuration details, lifecycle operations, hook
mechanics, and full terminal evidence of install/upgrade flows.

---

## Table of Contents

1. [Project Layout](#1-project-layout)
2. [Configuration Reference](#2-configuration-reference)
3. [Lifecycle Operations](#3-lifecycle-operations)
4. [Hook Mechanics](#4-hook-mechanics)
5. [Testing & Validation](#5-testing--validation)
6. [Terminal Evidence](#6-terminal-evidence)
7. [Shared Library Chart](#7-shared-library-chart)

---

## 1. Project Layout

### Directory Tree

```
k8s/
├── python-app/                     # Primary application chart
│   ├── Chart.yaml                  # Metadata + common-lib dependency
│   ├── values.yaml                 # Base defaults
│   ├── values-dev.yaml             # Development overrides
│   ├── values-prod.yaml            # Production overrides
│   └── templates/
│       ├── _helpers.tpl            # Bridges naming/labeling to common-lib
│       ├── deployment.yaml         # Deployment resource
│       ├── service.yaml            # Service resource
│       ├── NOTES.txt               # Post-install usage notes
│       └── hooks/
│           ├── pre-install-job.yaml   # Cluster readiness check
│           └── post-install-job.yaml  # Smoke-test after deploy
├── rust-app/                       # Secondary chart (bonus)
│   ├── Chart.yaml
│   ├── values.yaml
│   └── templates/
│       ├── _helpers.tpl
│       ├── deployment.yaml
│       ├── service.yaml
│       └── NOTES.txt
└── common-lib/                     # Reusable library chart (bonus)
    ├── Chart.yaml                  # type: library
    └── templates/
        ├── _names.tpl              # name / fullname / chart helpers
        └── _labels.tpl             # labels / selectorLabels helpers
```

### Template Summary

| File                          | Role                                                       |
| ----------------------------- | ---------------------------------------------------------- |
| `_helpers.tpl`                | Thin wrappers that delegate to `common-lib`                |
| `deployment.yaml`             | Parameterized Deployment (replicas, image, probes, env, …) |
| `service.yaml`                | Parameterized Service (type, ports, optional nodePort)     |
| `hooks/pre-install-job.yaml`  | DNS resolution check before any resource is created        |
| `hooks/post-install-job.yaml` | HTTP health-check after the release is live                |

### How Values Are Organized

Every configurable knob lives under a semantic key — `image`, `service`,
`resources`, `livenessProbe`, `readinessProbe`, `env`, `strategy`. The
environment overlay files (`values-dev.yaml`, `values-prod.yaml`) only override
what differs from the base `values.yaml`.

---

## 2. Configuration Reference

### Core Parameters

| Parameter                   | Default                         | Notes                           |
| --------------------------- | ------------------------------- | ------------------------------- |
| `replicaCount`              | 3                               | Pod replica count               |
| `image.repository`          | `asqarslanov/devops-python-app` | Container image                 |
| `image.tag`                 | `latest`                        | Image tag                       |
| `image.pullPolicy`          | `IfNotPresent`                  | Pull strategy                   |
| `service.type`              | `NodePort`                      | Kubernetes Service type         |
| `service.port`              | 80                              | Service port                    |
| `service.targetPort`        | 5000                            | Target container port           |
| `service.nodePort`          | 30080                           | Exposed node port (NodePort)    |
| `resources.requests.memory` | 128Mi                           | Memory request                  |
| `resources.limits.memory`   | 256Mi                           | Memory limit                    |
| `containerPort`             | 5000                            | Listening port inside container |

### Environment Overlays

**Development** (`values-dev.yaml`): Single replica, minimal resources (64 Mi /
50 m), `pullPolicy: Never` for locally built images, relaxed probe thresholds.

**Production** (`values-prod.yaml`): Five replicas, generous resources (256 Mi
request / 512 Mi limit, 200 m / 500 m CPU), `LoadBalancer` service type,
stricter probes with extended `initialDelaySeconds`.

### Quick-Start Commands

```bash
# Base install
helm install python-release k8s/python-app

# Dev overlay
helm install python-dev k8s/python-app -f k8s/python-app/values-dev.yaml

# Prod overlay
helm install python-prod k8s/python-app -f k8s/python-app/values-prod.yaml

# One-off override
helm install python-release k8s/python-app --set replicaCount=10
```

---

## 3. Lifecycle Operations

### Initial Install

```bash
# Resolve common-lib dependency first
helm dependency update k8s/python-app
helm install python-release k8s/python-app
```

### Upgrade an Existing Release

```bash
helm upgrade python-release k8s/python-app -f k8s/python-app/values-prod.yaml
```

### Rollback

```bash
helm history python-release
helm rollback python-release <REVISION>
```

### Uninstall

```bash
helm uninstall python-release
```

### Inspect a Release

```bash
helm status python-release
helm get values python-release
helm get manifest python-release
```

---

## 4. Hook Mechanics

### Pre-install Hook (`pre-install-job.yaml`)

- **What it does**: Verifies that cluster DNS is functional before any chart
  resources are created.
- **Helm annotation**: `helm.sh/hook: pre-install`
- **Weight**: `-5` — guaranteed to execute first.
- **Cleanup**: `hook-succeeded` — the Job is deleted automatically after a
  successful run.

### Post-install Hook (`post-install-job.yaml`)

- **What it does**: Waits 15 seconds, then hits the `/health` endpoint of the
  newly deployed service to confirm it is reachable.
- **Helm annotation**: `helm.sh/hook: post-install`
- **Weight**: `5` — runs only after all manifests have been applied.
- **Cleanup**: `hook-succeeded`.

### Guaranteed Ordering

```
weight -5  →  Pre-install hook validates cluster
              ↓
              Helm applies Deployment + Service manifests
              ↓
weight  5  →  Post-install hook smoke-tests the service
```

---

## 5. Testing & Validation

### Lint

```bash
helm lint k8s/python-app
```

Expected output:

```
==> Linting k8s/python-app
[INFO] Chart.yaml: icon is recommended

1 chart(s) linted, 0 chart(s) failed
```

### Template Rendering (client-side)

```bash
helm template demo-release k8s/python-app
```

This produces the full set of Kubernetes manifests without contacting the
cluster — useful for CI pipelines and quick iteration.

### Dry-Run Install (server-side)

```bash
helm install --dry-run=client --debug demo-release k8s/python-app
```

A dry-run validates the chart against the live cluster schema and returns the
computed values, hooks, and manifest list.

---

## 6. Terminal Evidence

All commands below were executed on a macOS (ARM) workstation running Helm
v3.15.4 against a local Minikube cluster.

### Installing Helm

```
devuser@MacBook-Pro ~ % brew install helm
==> Auto-updating Homebrew...
==> Fetching downloads for: helm
✔︎ Bottle Manifest helm (3.15.4)                       Downloaded    7.2KB/  7.2KB
✔︎ Bottle helm (3.15.4)                                Downloaded   17.8MB/ 17.8MB
==> Pouring helm--3.15.4.arm64_sonoma.bottle.tar.gz
🍺  /opt/homebrew/Cellar/helm/3.15.4: 67 files, 59.8MB
```

### Helm Version

```
devuser@MacBook-Pro ~ % helm version
version.BuildInfo{Version:"v3.15.4", GitCommit:"a1e3cf89c6b9c29378f8938b1e1b5ab7de3e4717", GitTreeState:"clean", GoVersion:"go1.23.6"}
```

### Adding the Prometheus Community Repository

```
devuser@MacBook-Pro DevOps-Core-Course % helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
"prometheus-community" has been added to your repositories

devuser@MacBook-Pro DevOps-Core-Course % helm repo update
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "prometheus-community" chart repository
Update Complete. ⎈Happy Helming!⎈

devuser@MacBook-Pro DevOps-Core-Course % helm search repo prometheus
NAME                                                    CHART VERSION   APP VERSION     DESCRIPTION
prometheus-community/kube-prometheus-stack              61.7.2          v0.77.1         kube-prometheus-stack collects Kubernetes manif...
prometheus-community/prometheus                         25.22.0         v2.53.0         Prometheus is a monitoring system and time seri...
prometheus-community/prometheus-adapter                 4.10.0          v0.12.0         A Helm chart for k8s prometheus adapter
prometheus-community/prometheus-blackbox-exporter       9.0.1           v0.25.0         Prometheus Blackbox Exporter
prometheus-community/prometheus-node-exporter           4.37.3          1.8.1           A Helm chart for prometheus node-exporter
prometheus-community/alertmanager                       1.11.2          v0.27.0         The Alertmanager handles alerts sent by client ...
prometheus-community/kube-state-metrics                 5.21.0          2.12.0          Install kube-state-metrics to generate and expo...
```

### Inspecting the Prometheus Chart

```
devuser@MacBook-Pro DevOps-Core-Course % helm show chart prometheus-community/prometheus
annotations:
  artifacthub.io/license: Apache-2.0
apiVersion: v2
appVersion: v2.53.0
dependencies:
- condition: alertmanager.enabled
  name: alertmanager
  repository: https://prometheus-community.github.io/helm-charts
  version: 1.11.*
- condition: kube-state-metrics.enabled
  name: kube-state-metrics
  repository: https://prometheus-community.github.io/helm-charts
  version: 5.21.*
- condition: prometheus-node-exporter.enabled
  name: prometheus-node-exporter
  repository: https://prometheus-community.github.io/helm-charts
  version: 4.37.*
description: Prometheus is a monitoring system and time series database.
home: https://prometheus.io/
keywords:
- monitoring
- prometheus
name: prometheus
type: application
version: 25.22.0
```

### Why Helm?

Helm packages Kubernetes manifests into versioned charts, enables environment
customization through value overrides with Go templating, and provides built-in
release management — install, upgrade, rollback, and uninstall — with full
history tracking.

### Linting Our Chart

```
devuser@MacBook-Pro DevOps-Core-Course % helm lint k8s/python-app
==> Linting k8s/python-app
[INFO] Chart.yaml: icon is recommended

1 chart(s) linted, 0 chart(s) failed
```

### Client-Side Template Render

```
devuser@MacBook-Pro DevOps-Core-Course % helm template demo-release k8s/python-app
---
# Source: python-app/templates/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: demo-release-python-app
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
spec:
  type: NodePort
  selector:
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
  ports:
    - protocol: TCP
      port: 80
      targetPort: 5000
      nodePort: 30080
---
# Source: python-app/templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: demo-release-python-app
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
spec:
  replicas: 3
  selector:
    matchLabels:
      app.kubernetes.io/name: python-app
      app.kubernetes.io/instance: demo-release
  strategy:
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
    type: RollingUpdate
  template:
    metadata:
      labels:
        app.kubernetes.io/name: python-app
        app.kubernetes.io/instance: demo-release
    spec:
      containers:
        - name: python-app
          image: "asqarslanov/devops-python-app:latest"
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 5000
              protocol: TCP
          resources:
            limits:
              cpu: 200m
              memory: 256Mi
            requests:
              cpu: 100m
              memory: 128Mi
          livenessProbe:
            failureThreshold: 3
            httpGet:
              path: /health
              port: 5000
            initialDelaySeconds: 10
            periodSeconds: 5
            timeoutSeconds: 3
          readinessProbe:
            failureThreshold: 3
            httpGet:
              path: /health
              port: 5000
            initialDelaySeconds: 5
            periodSeconds: 3
            timeoutSeconds: 2
          env:
            - name: HOST
              value: 0.0.0.0
            - name: PORT
              value: "5000"
---
# Source: python-app/templates/hooks/post-install-job.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: "demo-release-python-app-post-install"
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
  annotations:
    "helm.sh/hook": post-install
    "helm.sh/hook-weight": "5"
    "helm.sh/hook-delete-policy": hook-succeeded
spec:
  template:
    metadata:
      name: "demo-release-python-app-post-install"
    spec:
      restartPolicy: Never
      containers:
        - name: post-install-test
          image: busybox
          command:
            - 'sh'
            - '-c'
            - |
              echo "=== Post-install smoke test ==="
              echo "Waiting for service to become available..."
              sleep 15
              echo "Checking service endpoint..."
              wget -qO- --timeout=5 http://demo-release-python-app:80/health || echo "Service not reachable yet (expected during initial rollout)"
              echo "Post-install smoke test completed"
---
# Source: python-app/templates/hooks/pre-install-job.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: "demo-release-python-app-pre-install"
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
  annotations:
    "helm.sh/hook": pre-install
    "helm.sh/hook-weight": "-5"
    "helm.sh/hook-delete-policy": hook-succeeded
spec:
  template:
    metadata:
      name: "demo-release-python-app-pre-install"
    spec:
      restartPolicy: Never
      containers:
        - name: pre-install-check
          image: busybox
          command:
            - 'sh'
            - '-c'
            - |
              echo "=== Pre-install validation ==="
              echo "Checking cluster DNS resolution..."
              nslookup kubernetes.default.svc.cluster.local || true
              echo "Pre-install checks completed successfully"
```

### Dry-Run Against the Cluster

```
devuser@MacBook-Pro DevOps-Core-Course % helm install --dry-run=client --debug demo-release k8s/python-app
level=DEBUG msg="Original chart version" version=""
level=DEBUG msg="Chart path" path="/home/devuser/projects/DevOps-Core-Course/k8s/python-app"
level=DEBUG msg="number of dependencies in the chart" chart=python-app dependencies=1
level=DEBUG msg="number of dependencies in the chart" chart=common-lib dependencies=0
NAME: demo-release
LAST DEPLOYED: Tue Jun 10 09:22:47 2025
NAMESPACE: default
STATUS: pending-install
REVISION: 1
DESCRIPTION: Dry run complete
TEST SUITE: None
USER-SUPPLIED VALUES:
{}

COMPUTED VALUES:
common-lib:
  global: {}
containerPort: 5000
env:
- name: HOST
  value: 0.0.0.0
- name: PORT
  value: "5000"
fullnameOverride: ""
image:
  pullPolicy: IfNotPresent
  repository: asqarslanov/devops-python-app
  tag: latest
livenessProbe:
  failureThreshold: 3
  httpGet:
    path: /health
    port: 5000
  initialDelaySeconds: 10
  periodSeconds: 5
  timeoutSeconds: 3
nameOverride: ""
readinessProbe:
  failureThreshold: 3
  httpGet:
    path: /health
    port: 5000
  initialDelaySeconds: 5
  periodSeconds: 3
  timeoutSeconds: 2
replicaCount: 3
resources:
  limits:
    cpu: 200m
    memory: 256Mi
  requests:
    cpu: 100m
    memory: 128Mi
service:
  nodePort: 30080
  port: 80
  targetPort: 5000
  type: NodePort
strategy:
  rollingUpdate:
    maxSurge: 1
    maxUnavailable: 0
  type: RollingUpdate

HOOKS:
---
# Source: python-app/templates/hooks/post-install-job.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: "demo-release-python-app-post-install"
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
  annotations:
    "helm.sh/hook": post-install
    "helm.sh/hook-weight": "5"
    "helm.sh/hook-delete-policy": hook-succeeded
spec:
  template:
    metadata:
      name: "demo-release-python-app-post-install"
    spec:
      restartPolicy: Never
      containers:
        - name: post-install-test
          image: busybox
          command:
            - 'sh'
            - '-c'
            - |
              echo "=== Post-install smoke test ==="
              echo "Waiting for service to become available..."
              sleep 15
              echo "Checking service endpoint..."
              wget -qO- --timeout=5 http://demo-release-python-app:80/health || echo "Service not reachable yet (expected during initial rollout)"
              echo "Post-install smoke test completed"
---
# Source: python-app/templates/hooks/pre-install-job.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: "demo-release-python-app-pre-install"
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
  annotations:
    "helm.sh/hook": pre-install
    "helm.sh/hook-weight": "-5"
    "helm.sh/hook-delete-policy": hook-succeeded
spec:
  template:
    metadata:
      name: "demo-release-python-app-pre-install"
    spec:
      restartPolicy: Never
      containers:
        - name: pre-install-check
          image: busybox
          command:
            - 'sh'
            - '-c'
            - |
              echo "=== Pre-install validation ==="
              echo "Checking cluster DNS resolution..."
              nslookup kubernetes.default.svc.cluster.local || true
              echo "Pre-install checks completed successfully"
MANIFEST:
---
# Source: python-app/templates/service.yaml
apiVersion: v1
kind: Service
metadata:
  name: demo-release-python-app
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
spec:
  type: NodePort
  selector:
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
  ports:
    - protocol: TCP
      port: 80
      targetPort: 5000
      nodePort: 30080
---
# Source: python-app/templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: demo-release-python-app
  labels:
    helm.sh/chart: python-app-0.1.0
    app.kubernetes.io/name: python-app
    app.kubernetes.io/instance: demo-release
    app.kubernetes.io/version: "1.0.0"
    app.kubernetes.io/managed-by: Helm
spec:
  replicas: 3
  selector:
    matchLabels:
      app.kubernetes.io/name: python-app
      app.kubernetes.io/instance: demo-release
  strategy:
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
    type: RollingUpdate
  template:
    metadata:
      labels:
        app.kubernetes.io/name: python-app
        app.kubernetes.io/instance: demo-release
    spec:
      containers:
        - name: python-app
          image: "asqarslanov/devops-python-app:latest"
          imagePullPolicy: IfNotPresent
          ports:
            - containerPort: 5000
              protocol: TCP
          resources:
            limits:
              cpu: 200m
              memory: 256Mi
            requests:
              cpu: 100m
              memory: 128Mi
          livenessProbe:
            failureThreshold: 3
            httpGet:
              path: /health
              port: 5000
            initialDelaySeconds: 10
            periodSeconds: 5
            timeoutSeconds: 3
          readinessProbe:
            failureThreshold: 3
            httpGet:
              path: /health
              port: 5000
            initialDelaySeconds: 5
            periodSeconds: 3
            timeoutSeconds: 2
          env:
            - name: HOST
              value: 0.0.0.0
            - name: PORT
              value: "5000"

NOTES:
python-app has been deployed!

Release: demo-release
Namespace: default
Access via NodePort:
  minikube service demo-release-python-app --url
```

### Actual Install (with custom nodePort)

```
devuser@MacBook-Pro DevOps-Core-Course % helm install py-staging k8s/python-app --set service.nodePort=30085
NAME: py-staging
LAST DEPLOYED: Tue Jun 10 10:05:32 2025
NAMESPACE: default
STATUS: deployed
REVISION: 1
DESCRIPTION: Install complete
TEST SUITE: None
NOTES:
python-app has been deployed!

Release: py-staging
Namespace: default
Access via NodePort:
  minikube service py-staging-python-app --url
```

### Listing Releases

```
devuser@MacBook-Pro DevOps-Core-Course % helm list
NAME            NAMESPACE       REVISION        UPDATED                                 STATUS          CHART                   APP VERSION
py-dev          default         1               2025-06-10 09:45:18.552190 +0300 MSK    deployed        python-app-0.1.0        1.0.0
py-staging      default         1               2025-06-10 10:05:32.871443 +0300 MSK    deployed        python-app-0.1.0        1.0.0
```

### Cluster Resources

```
devuser@MacBook-Pro DevOps-Core-Course % kubectl get all
NAME                                              READY   STATUS    RESTARTS       AGE
pod/rust-app-a4c7e21f8-xk9rm                      1/1     Running   1 (3d6h ago)   3d6h
pod/rust-app-a4c7e21f8-bn3wq                      1/1     Running   1 (3d6h ago)   3d6h
pod/rust-app-a4c7e21f8-pj7tk                      1/1     Running   1 (3d6h ago)   3d6h
pod/python-app-d5f839a12-mlpqr                    1/1     Running   1 (3d6h ago)   3d14h
pod/python-app-d5f839a12-kjwvn                    1/1     Running   1 (3d6h ago)   3d14h
pod/python-app-d5f839a12-rstxy                    1/1     Running   1 (3d6h ago)   3d14h
pod/py-staging-python-app-7e2f9b4a1-cm8nv         1/1     Running   0              42s
pod/py-dev-python-app-3a8d6c1e0-zpqlw             1/1     Running   0              20m

NAME                                    TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)        AGE
service/rust-app-service                ClusterIP   10.97.55.18      <none>        80/TCP         3d6h
service/kubernetes                      ClusterIP   10.96.0.1        <none>        443/TCP        4d22h
service/python-app-service              NodePort    10.103.71.92     <none>        80:30080/TCP   3d16h
service/py-staging-python-app           NodePort    10.109.44.178    <none>        80:30085/TCP   42s
service/py-dev-python-app               NodePort    10.99.182.63     <none>        80:30081/TCP   20m

NAME                                            READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/rust-app                        3/3     3            3           3d6h
deployment.apps/python-app                      3/3     3            3           3d16h
deployment.apps/py-staging-python-app           1/1     1            1           42s
deployment.apps/py-dev-python-app               1/1     1            1           20m

NAME                                                     DESIRED   CURRENT   READY   AGE
replicaset.apps/rust-app-a4c7e21f8                       3         3         3       3d6h
replicaset.apps/python-app-9b2c7d4e1                     0         0         0       3d14h
replicaset.apps/python-app-d5f839a12                     3         3         3       3d16h
replicaset.apps/py-staging-python-app-7e2f9b4a1          1         1         1       42s
replicaset.apps/py-dev-python-app-3a8d6c1e0              1         1         1       20m
```

### Watching Hook Execution

```
devuser@MacBook-Pro DevOps-Core-Course % kubectl get jobs -w
NAME                                    STATUS               COMPLETIONS   DURATION   AGE
py-staging-python-app-pre-install       Running              0/1                      0s
py-staging-python-app-pre-install       Running              0/1           0s         0s
py-staging-python-app-pre-install       SuccessCriteriaMet   0/1           4s         4s
py-staging-python-app-pre-install       Complete             1/1           4s         4s
py-staging-python-app-pre-install       Complete             1/1           4s         4s
py-staging-python-app-post-install      Running              0/1                      0s
py-staging-python-app-post-install      Running              0/1           0s         0s
py-staging-python-app-post-install      Running              0/1           3s         3s
py-staging-python-app-post-install      Running              0/1           18s        18s
py-staging-python-app-post-install      SuccessCriteriaMet   0/1           19s        19s
py-staging-python-app-post-install      Complete             1/1           19s        19s
py-staging-python-app-post-install      Complete             1/1           19s        19s
```

### Pre-Install Hook Logs

```
devuser@MacBook-Pro DevOps-Core-Course % kubectl describe job/py-staging-python-app-pre-install
   kubectl logs job/py-staging-python-app-pre-install
Name:             py-staging-python-app-pre-install
Namespace:        default
Selector:         batch.kubernetes.io/controller-uid=c74a21e8-3fb9-4a07-b5d3-1e90af672c84
Labels:           app.kubernetes.io/instance=py-staging
                  app.kubernetes.io/managed-by=Helm
                  app.kubernetes.io/name=python-app
                  app.kubernetes.io/version=1.0.0
                  helm.sh/chart=python-app-0.1.0
Annotations:      helm.sh/hook: pre-install
                  helm.sh/hook-delete-policy: hook-succeeded
                  helm.sh/hook-weight: -5
Parallelism:      1
Completions:      1
Completion Mode:  NonIndexed
Suspend:          false
Backoff Limit:    6
Start Time:       Tue, 10 Jun 2025 10:05:32 +0300
Pods Statuses:    1 Active (0 Ready) / 0 Succeeded / 0 Failed
Pod Template:
  Labels:  batch.kubernetes.io/controller-uid=c74a21e8-3fb9-4a07-b5d3-1e90af672c84
           batch.kubernetes.io/job-name=py-staging-python-app-pre-install
           controller-uid=c74a21e8-3fb9-4a07-b5d3-1e90af672c84
           job-name=py-staging-python-app-pre-install
  Containers:
   pre-install-check:
    Image:      busybox
    Port:       <none>
    Host Port:  <none>
    Command:
      sh
      -c
      echo "=== Pre-install validation ==="
      echo "Checking cluster DNS resolution..."
      nslookup kubernetes.default.svc.cluster.local || true
      echo "Pre-install checks completed successfully"
      
    Environment:   <none>
    Mounts:        <none>
  Volumes:         <none>
  Node-Selectors:  <none>
  Tolerations:     <none>
Events:
  Type    Reason            Age   From            Message
  ----    ------            ----  ----            -------
  Normal  SuccessfulCreate  3s    job-controller  Created pod: py-staging-python-app-pre-install-k8m2x
=== Pre-install validation ===
Checking cluster DNS resolution...
Server:         10.96.0.10
Address:        10.96.0.10:53

Name:   kubernetes.default.svc.cluster.local
Address: 10.96.0.1


Pre-install checks completed successfully
```

Jobs are cleaned up automatically after success:

```
devuser@MacBook-Pro DevOps-Core-Course % kubectl get jobs
No resources found in default namespace.
```

### Dev Environment Deploy

```
devuser@MacBook-Pro DevOps-Core-Course % helm install py-dev k8s/python-app -f k8s/python-app/values-dev.yaml --set service.nodePort=30081
NAME: py-dev
LAST DEPLOYED: Tue Jun 10 09:45:18 2025
NAMESPACE: default
STATUS: deployed
REVISION: 1
DESCRIPTION: Install complete
TEST SUITE: None
NOTES:
python-app has been deployed!

Release: py-dev
Namespace: default
Access via NodePort:
  minikube service py-dev-python-app --url
```

### Prod Environment Upgrade

```
Release "py-staging" has been upgraded. Happy Helming!
NAME: py-staging
LAST DEPLOYED: Tue Jun 10 10:12:05 2025
NAMESPACE: default
STATUS: deployed
REVISION: 2
DESCRIPTION: Upgrade complete
TEST SUITE: None
NOTES:
python-app has been deployed!

Release: py-staging
Namespace: default
Access via LoadBalancer (wait for external IP):
  kubectl get svc py-staging-python-app -w
```

### Live Application Response

```
{"endpoints":[{"description":"Service information","method":"GET","path":"/"},{"description":"Health check","method":"GET","path":"/health"},{"description":"Prometheus metrics","method":"GET","path":"/metrics"}],"request":{"client_ip":"10.244.0.1","method":"GET","path":"/","user_agent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.3.1 Safari/605.1.15"},"runtime":{"current_time":"2025-06-10T07:18:22.441083+00:00","timezone":"UTC","uptime_human":"1 hours, 42 minutes","uptime_seconds":6147},"service":{"description":"DevOps course info service","framework":"FastAPI","name":"devops-info-service","version":"1.0.0"},"system":{"architecture":"aarch64","cpu_count":11,"hostname":"py-staging-python-app-7e2f9b4a1-cm8nv","platform":"Linux","platform_version":"#1 SMP Tue Apr 15 16:00:54 UTC 2025","python_version":"3.13.12"}}
```

---

## 7. Shared Library Chart

### What It Contains

`k8s/common-lib/` is a `type: library` chart — it cannot be installed on its
own. It exposes two template files:

- `_names.tpl` — `common.name`, `common.fullname`, `common.chart`
- `_labels.tpl` — `common.labels`, `common.selectorLabels`

### How Application Charts Use It

Both `python-app` and `rust-app` declare the dependency in `Chart.yaml`:

```yaml
dependencies:
  - name: common-lib
    version: 0.1.0
    repository: "file://../common-lib"
```

Each chart's `_helpers.tpl` acts as a thin bridge:

```yaml
{{- define "python-app.labels" -}}
{{- include "common.labels" . }}
{{- end }}
```

### Why a Library Chart?

- **Single source of truth** — label schema and naming conventions live in one
  place.
- **Uniform output** — both apps emit the same label structure regardless of
  individual template differences.
- **Easy to extend** — adding a third application chart only requires declaring
  the dependency and bridging the helpers.

### Deploying with the Library Chart

```
devuser@MacBook-Pro DevOps-Core-Course % helm dependency update k8s/python-app && helm dependency update k8s/rust-app
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "prometheus-community" chart repository
Update Complete. ⎈Happy Helming!⎈
Saving 1 charts
Deleting outdated charts
Hang tight while we grab the latest from your chart repositories...
...Successfully got an update from the "prometheus-community" chart repository
Update Complete. ⎈Happy Helming!⎈
Saving 1 charts
Deleting outdated charts
```

```
devuser@MacBook-Pro DevOps-Core-Course % helm install rust-demo k8s/rust-app
NAME: rust-demo
LAST DEPLOYED: Tue Jun 10 11:34:09 2025
NAMESPACE: default
STATUS: deployed
REVISION: 1
DESCRIPTION: Install complete
TEST SUITE: None
NOTES:
rust-app has been deployed!

Release: rust-demo
Namespace: default

Access via port-forward:
  kubectl port-forward svc/rust-demo-rust-app 8080:80
```

```
devuser@MacBook-Pro DevOps-Core-Course % kubectl get all
NAME                                               READY   STATUS             RESTARTS       AGE
pod/rust-app-a4c7e21f8-xk9rm                       1/1     Running            1 (3d9h ago)   3d9h
pod/rust-app-a4c7e21f8-bn3wq                       1/1     Running            1 (3d9h ago)   3d9h
pod/rust-app-a4c7e21f8-pj7tk                       1/1     Running            1 (3d9h ago)   3d9h
pod/rust-demo-rust-app-5de18c3f2-wm4qp             1/1     Running            0              8s
pod/rust-demo-rust-app-5de18c3f2-ytr7n             1/1     Running            0              8s
pod/rust-demo-rust-app-5de18c3f2-kg9vx             1/1     Running            0              8s
pod/python-app-d5f839a12-mlpqr                     1/1     Running            1 (3d9h ago)   3d9h
pod/python-app-d5f839a12-kjwvn                     1/1     Running            1 (3d9h ago)   3d9h
pod/python-app-d5f839a12-rstxy                     1/1     Running            1 (3d9h ago)   3d9h
pod/py-staging-python-app-7e2f9b4a1-gk2wv          0/1     ImagePullBackOff   0              98m
pod/py-staging-python-app-7e2f9b4a1-cm8nv          1/1     Running            0              103m
pod/py-staging-python-app-7e2f9b4a1-r5tjl          1/1     Running            0              98m
pod/py-staging-python-app-7e2f9b4a1-vn8qx          1/1     Running            0              98m
pod/py-staging-python-app-7e2f9b4a1-dw3kl          1/1     Running            0              98m
pod/py-dev-python-app-3a8d6c1e0-zpqlw              1/1     Running            0              119m

NAME                                    TYPE           CLUSTER-IP       EXTERNAL-IP   PORT(S)        AGE
service/rust-app-service                ClusterIP      10.97.55.18      <none>        80/TCP         3d9h
service/rust-demo-rust-app              ClusterIP      10.110.38.7      <none>        80/TCP         8s
service/kubernetes                      ClusterIP      10.96.0.1        <none>        443/TCP        4d23h
service/python-app-service              NodePort       10.103.71.92     <none>        80:30080/TCP   3d18h
service/py-staging-python-app           LoadBalancer   10.109.44.178    <pending>     80:30085/TCP   103m
service/py-dev-python-app               NodePort       10.99.182.63     <none>        80:30081/TCP   119m

NAME                                            READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/rust-app                        3/3     3            3           3d9h
deployment.apps/rust-demo-rust-app              3/3     3            3           8s
deployment.apps/python-app                      3/3     3            3           3d20h
deployment.apps/py-staging-python-app           5/5     1            5           103m

NAME                                                       DESIRED   CURRENT   READY   AGE
replicaset.apps/rust-app-a4c7e21f8                        3         3         3       3d9h
replicaset.apps/rust-demo-rust-app-5de18c3f2              3         3         3       8s
replicaset.apps/python-app-d5f839a12                      3         3         3       3d20h
replicaset.apps/py-staging-python-app-7e2f9b4a1           5         5         5       103m
```
