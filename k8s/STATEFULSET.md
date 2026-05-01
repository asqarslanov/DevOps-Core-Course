# Lab 15: StatefulSets & Persistent Storage

This document explains StatefulSet guarantees, how the `python-app` Helm chart
switches between Argo Rollouts and a StatefulSet, and shows verification on a
live cluster.

---

## 1. Helm wiring (how the chart toggles modes)

The chart provides two mutually exclusive templates:

- **`templates/statefulset.yaml`** – A StatefulSet that mirrors the Rollout pod
  template. It includes `serviceName` pointing at a headless Service and
  `volumeClaimTemplates` when `persistence.enabled` is true.
- **`templates/service-headless.yaml`** – A headless Service; only rendered when
  `statefulset.enabled` is true.
- **`templates/rollout.yaml`** – Rendered only when `statefulset.enabled` is
  false (default Lab 14 behaviour).
- **`templates/pvc.yaml`** – A shared PVC for Rollout mode; omitted in
  StatefulSet mode (PVCs come from the StatefulSet's templates).

To switch to StatefulSet mode, use either `values.yaml`
(`statefulset.enabled: true`) or the overlay:

```bash
helm upgrade --install lab15 ./k8s/python-app \
  -n lab15 --create-namespace \
  -f k8s/python-app/values-statefulset.yaml
```

Relevant configuration knobs (in `values.yaml`):\
`statefulset.podManagementPolicy`, `statefulset.updateStrategy.type`
(`RollingUpdate` or `OnDelete`), and `rollingUpdate.partition` (when using
`RollingUpdate`).

---

## 2. What a StatefulSet guarantees (and why it exists)

A StatefulSet is designed for workloads that need stable identity and durable
data:

- **Stable network IDs:** Pods follow the naming pattern `<statefulset>-0`,
  `<statefulset>-1`, … and keep those ordinals across rescheduling.
- **Stable storage:** Using `volumeClaimTemplates`, each ordinal gets its own
  PersistentVolumeClaim (PVC) that stays bound to that identity (e.g.,
  `data-volume-<sts>-0`).
- **Ordered lifecycle:** The default `podManagementPolicy: OrderedReady` ensures
  pods are created, scaled, and rolled back in order (useful for clustered
  software). `Parallel` removes ordering constraints.

### Comparison: Deployment / Rollout vs. StatefulSet

| Concern            | Deployment / Rollout             | StatefulSet                      |
| ------------------ | -------------------------------- | -------------------------------- |
| Pod names          | Random suffix                    | Stable ordinal suffix            |
| Storage            | Typically one shared PVC or none | Per-pod PVCs via templates       |
| Scaling order      | Simultaneous / surge-based       | Ordered by default (0 -> 1 -> 2) |
| Stable DNS per pod | Not built-in                     | Via headless Service             |

**When to use which?**

- **Deployments or Rollouts** – Stateless HTTP services, progressive delivery.
- **StatefulSets** – When each replica needs its own disk and/or a predictable
  hostname (databases, Kafka, etcd-like patterns).

### Headless Services (`clusterIP: None`)

A headless Service does not allocate a single virtual IP for load balancing.
Instead, Kubernetes DNS publishes **one A/AAAA record per ready endpoint**
(pod). Clients can resolve individual pods directly.

The usual DNS pattern for a StatefulSet pod is:

```
<pod-name>.<headless-service>.<namespace>.svc.cluster.local
```

For this lab:

`lab15-python-app-1.lab15-python-app-headless.lab15.svc.cluster.local`

The chart keeps the existing **NodePort/ClusterIP Service** for normal traffic
while adding **`<fullname>-headless`** for stable per-pod DNS.

---

## 3. Resource verification after installation

After installing with StatefulSet mode enabled:

```bash
kubectl get po,sts,svc,pvc -n lab15
```

Example output:

```text
NAME                     READY   STATUS    RESTARTS   AGE
pod/lab15-python-app-0   1/1     Running   0          14s
pod/lab15-python-app-1   1/1     Running   0          75s
pod/lab15-python-app-2   1/1     Running   0          70s

NAME                                READY   AGE
statefulset.apps/lab15-python-app   3/3     80s

NAME                                TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)        AGE
service/lab15-python-app            NodePort    10.108.45.213    <none>        80:31234/TCP   80s
service/lab15-python-app-headless   ClusterIP   None             <none>        80/TCP         80s

NAME                                                   STATUS   VOLUME   CAPACITY   ACCESS MODES   STORAGECLASS
persistentvolumeclaim/data-volume-lab15-python-app-0   Bound    ...      100Mi      RWO            standard
persistentvolumeclaim/data-volume-lab15-python-app-1   Bound    ...      100Mi      RWO            standard
persistentvolumeclaim/data-volume-lab15-python-app-2   Bound    ...      100Mi      RWO            standard
```

Pods use ordinal names, and each pod has its own PVC bound to the `standard`
storage class.

---

## 4. Network identity – DNS resolution

From inside `lab15-python-app-0`, resolve another pod via the headless Service:

```bash
kubectl exec -n lab15 lab15-python-app-0 -- \
  getent hosts lab15-python-app-1.lab15-python-app-headless.lab15.svc.cluster.local
```

Output:

```text
10.244.1.45     lab15-python-app-1.lab15-python-app-headless.lab15.svc.cluster.local
```

This matches CoreDNS naming:
`<statefulset-pod>.<headless-svc>.<namespace>.svc.cluster.local`.

---

## 5. Per-pod storage and persistence demonstration

The application stores visit counts in `VISITS_FILE` (`/data/visits`, defined in
`_helpers.tpl`). The route `/` increments the counter, and `GET /visits` returns
the current count.

To generate different counts inside each pod (using localhost to bypass the
Service):

```bash
# Pod 0: 3 visits
kubectl exec -n lab15 lab15-python-app-0 -- python -c "
import urllib.request
for _ in range(3): urllib.request.urlopen('http://127.0.0.1:5000/')
print(urllib.request.urlopen('http://127.0.0.1:5000/visits').read().decode())
"

# Pod 1: 4 visits
kubectl exec -n lab15 lab15-python-app-1 -- python -c "
import urllib.request
for _ in range(4): urllib.request.urlopen('http://127.0.0.1:5000/')
print(urllib.request.urlopen('http://127.0.0.1:5000/visits').read().decode())
"

# Pod 2: 2 visits
kubectl exec -n lab15 lab15-python-app-2 -- python -c "
import urllib.request
for _ in range(2): urllib.request.urlopen('http://127.0.0.1:5000/')
print(urllib.request.urlopen('http://127.0.0.1:5000/visits').read().decode())
"
```

Observed JSON responses:

```text
{"visits":3}
{"visits":4}
{"visits":2}
```

Each replica maintains **isolated** persistent state.

### Pod deletion (StatefulSet keeps the PVC)

```bash
# Check current count on pod-0
kubectl exec -n lab15 lab15-python-app-0 -- cat /data/visits

# Delete the pod
kubectl delete pod -n lab15 lab15-python-app-0

# Wait for recreation
kubectl wait -n lab15 pod/lab15-python-app-0 --for=condition=Ready --timeout=120s

# Verify data survived
kubectl exec -n lab15 lab15-python-app-0 -- cat /data/visits
kubectl exec -n lab15 lab15-python-app-0 -- python -c "
import urllib.request
print(urllib.request.urlopen('http://127.0.0.1:5000/visits').read().decode())
"
```

Observed output before delete, after reschedule, and via HTTP:

```text
3
3
{"visits":3}
```

The PVC for ordinal 0 was reused, so the counter persisted across the pod
replacement.

---

## 6. Bonus – Update strategies for StatefulSets

### 6.1 Partitioned rolling update

With `updateStrategy.type: RollingUpdate` and `rollingUpdate.partition: N`, only
pods with **ordinal ≥ N** receive the new pod template during a rollout. Pods
with ordinal `< N` stay on the old revision until you lower the partition. This
is useful for canarying stateful upgrades (e.g., upgrade followers before the
leader).

Example in `values.yaml`:

```yaml
statefulset:
  updateStrategy:
    type: RollingUpdate
    rollingUpdate:
      partition: 1 # only pod-1 and above update first (assuming 3 replicas)
```

### 6.2 OnDelete

With `type: OnDelete`, the StatefulSet controller does **not** automatically
recreate pods when the template changes. You must delete pods manually to apply
the new spec – this gives maximum control. It is common for coordinated database
upgrades, version‑skew experiments, or when an operator handles pod deletion.

Example:

```yaml
statefulset:
  updateStrategy:
    type: OnDelete
```
