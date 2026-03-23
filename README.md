# DevOps Engineering: Core Practices

[![Labs](https://img.shields.io/badge/Labs-18-blue)](#weekly-breakdown)
[![Exam](https://img.shields.io/badge/Exam-Optional-green)](#exam-replacement)
[![Duration](https://img.shields.io/badge/Duration-18%20Weeks-lightgrey)](#weekly-breakdown)

A hands-on, lab-driven course covering the full DevOps lifecycle — from writing code and building containers to deploying on Kubernetes and observing production systems.

---

## Getting Started

1. **Fork** this repository to your own GitHub account
2. **Clone** your fork to your local machine
3. **Work through labs in order**, starting from Lab 1
4. **Open Pull Requests** to submit each lab (see [Submission Guide](#submission-guide))

---

## Weekly Breakdown

| Week | Lab | Topic | Technologies |
|------|-----|-------|-------------|
| 1 | 1 | Web App Development | Python / Go |
| 2 | 2 | Containerization | Docker, Multi-stage Builds |
| 3 | 3 | Continuous Integration | GitHub Actions, Snyk |
| 4 | 4 | Infrastructure as Code | Terraform |
| 5 | 5 | Configuration Management | Ansible (Fundamentals) |
| 6 | 6 | Continuous Deployment | Ansible (Advanced) |
| 7 | 7 | Centralized Logging | Promtail, Loki, Grafana |
| 8 | 8 | Application Monitoring | Prometheus, Grafana |
| 9 | 9 | Kubernetes Fundamentals | Minikube / kind, Deployments, Services |
| 10 | 10 | Helm Package Management | Charts, Templating, Hooks |
| 11 | 11 | Secrets Management | K8s Secrets, HashiCorp Vault |
| 12 | 12 | Config & Persistent Storage | ConfigMaps, PVCs |
| 13 | 13 | GitOps with ArgoCD | Declarative CD |
| 14 | 14 | Progressive Delivery | Argo Rollouts |
| 15 | 15 | Stateful Workloads | StatefulSets, Headless Services |
| 16 | 16 | Cluster-Level Monitoring | Kube-Prometheus Stack, Init Containers |
| — | — | *Exam Alternative* | — |
| 17 | 17 | Edge & Global Deployment | Fly.io |
| 18 | 18 | Decentralized Storage | 4EVERLAND, IPFS |

---

## Point System

### How Grades Work

| Component | Weight | Points |
|-----------|--------|--------|
| 16 Core Labs | 80% | 160 pts |
| Final Exam | 20% | 40 pts |
| Bonus Tasks | extra credit | up to +40 pts |
| **Maximum** | — | **200 pts** |

### Exam Replacement Option

If you prefer not to sit the exam, complete **both** alternative labs:

| Lab | Topic | Points |
|-----|-------|--------|
| Lab 17 | Fly.io Edge Deployment | 20 pts |
| Lab 18 | 4EVERLAND & IPFS Storage | 20 pts |

**Conditions:**
- Both labs must be completed (40 pts total, substitutes the exam)
- Each lab must score at least 16/20
- Submit by **one week before the exam date**
- You may still take the exam afterward if you want to boost your total

<details>
<summary>📊 Grading Scale</summary>

| Letter | Points | Percentage |
|--------|--------|------------|
| A | 180–200+ | 90–100% |
| B | 150–179 | 75–89% |
| C | 120–149 | 60–74% |
| D | 0–119 | 0–59% |

**Passing threshold:** 120 points (60%)

</details>

<details>
<summary>📈 Worked Examples</summary>

**Path A — Labs plus Exam:**
```
Core labs (16 × 9 avg):     144 pts
Bonus tasks (5 × 2.5 avg):  12.5 pts
Exam score:                   35 pts
                             -------
Total:                     191.5 pts → 96% → A
```

**Path B — Labs plus Exam Alternative:**
```
Core labs (16 × 9 avg):     144 pts
Bonus tasks (8 × 2.5 avg):  20 pts
Lab 17:                       18 pts
Lab 18:                       17 pts
                             -------
Total:                      199 pts → 99.5% → A
```

</details>

---

## Lab Format

Each lab carries **10 points** for the main deliverables and up to **2.5 bonus points**.

- **Passing score per lab:** 6/10
- **Late (within 1 week):** capped at 6/10
- **Late (beyond 1 week):** not graded

<details>
<summary>📋 Labs by Theme</Foundational></summary>

**Foundational (1–2)**
- Application development basics
- Docker & containerization

**CI/CD & Infra (3–4)**
- Pipeline automation with GitHub Actions
- Provisioning with Terraform

**Config Management (5–6)**
- Ansible playbooks, roles, and advanced patterns

**Observability (7–8)**
- Log aggregation (Loki + Promtail)
- Metrics & alerting (Prometheus + Grafana)

**Kubernetes Core (9–12)**
- Cluster basics, Deployments, Services
- Helm, Secrets, ConfigMaps, Storage

**Advanced Kubernetes (13–16)**
- GitOps (ArgoCD)
- Canary/blue-green deployments (Argo Rollouts)
- Stateful workloads, cluster monitoring

**Exam Alternative (17–18)**
- Fly.io edge computing
- IPFS / 4EVERLAND decentralized hosting

</details>

---

## Submission Guide

```bash
# Create a feature branch for the lab
git checkout -b lab1

# ... complete the lab tasks ...

# Stage, commit, and push
git add .
git commit -m "lab1: initial solution"
git push -u origin lab1

# Then open two Pull Requests:
#   PR 1 → course-repo:master  (for grading)
#   PR 2 → your-fork:master    (your own history)
```

<details>
<summary>✅ Submission Checklist</summary>

- [ ] All mandatory tasks implemented
- [ ] README / documentation written
- [ ] Screenshots attached where specified
- [ ] Code runs without errors
- [ ] Markdown passes [linting](https://dlaa.me/markdownlint/)
- [ ] Both PRs opened

</details>

---

## Tooling

<details>
<summary>🛠️ Software You Will Need</summary>

| Tool | Role |
|------|------|
| Git | Source control |
| Docker | Building & running containers |
| kubectl | Interacting with Kubernetes |
| Helm | Packaging K8s applications |
| Minikube / kind | Local Kubernetes cluster |
| Terraform | Declarative infrastructure |
| Ansible | Configuration automation |

</details>

<details>
<summary>📚 Official Docs</summary>

**Containers & Orchestration:**
- [Docker Docs](https://docs.docker.com/)
- [Kubernetes Docs](https://kubernetes.io/docs/)
- [Helm Docs](https://helm.sh/docs/)

**Automation & CI/CD:**
- [GitHub Actions](https://docs.github.com/en/actions)
- [Terraform Docs](https://www.terraform.io/docs)
- [Ansible Docs](https://docs.ansible.com/)

**Monitoring & Logging:**
- [Prometheus](https://prometheus.io/docs/)
- [Grafana](https://grafana.com/docs/)

**GitOps & Delivery:**
- [ArgoCD](https://argo-cd.readthedocs.io/)
- [Argo Rollouts](https://argoproj.github.io/argo-rollouts/)
- [HashiCorp Vault](https://developer.hashicorp.com/vault/docs)

</details>

<details>
<summary>💡 Study Tips</summary>

1. Don't procrastinate — start each lab as soon as it is assigned
2. Read the entire lab description before writing any code
3. Verify your work end-to-end before opening a PR
4. Write documentation incrementally, not at the last minute
5. Reach out for help early if you are stuck
6. Follow a clean Git workflow: branches, meaningful commits, PRs

</details>

<details>
<summary>🔧 Troubleshooting</summary>

**Docker issues:**
- *Daemon not running* → launch Docker Desktop or `systemctl start docker`
- *Permission denied* → add your user to the `docker` group and re-login

**Minikube issues:**
- *Fails to start* → try `minikube start --driver=docker`
- *Out of resources* → increase allocated memory/CPU in your VM settings

**Kubernetes issues:**
- *ImagePullBackOff* → verify image name/tag and registry access
- *CrashLoopBackOff* → inspect with `kubectl logs <pod>` and `kubectl describe pod <pod>`

</details>

---

## What You Will Gain

By the end of this course you will be able to:

- Build and containerize applications from scratch
- Set up CI/CD pipelines that test, scan, and deploy automatically
- Provision and manage infrastructure using code
- Deploy, scale, and update workloads on Kubernetes
- Instrument systems with logging, metrics, and alerting
- Apply GitOps principles for declarative delivery

---

**First step:** open [Lab 1](labs/lab01.md) and begin.

For questions, visit the course Moodle page or attend office hours.
