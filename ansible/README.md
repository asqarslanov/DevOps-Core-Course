# Ansible Automation Suite

[![Ansible Deployment](https://github.com/ilyalinhnguyen/DevOps-Core-Course/actions/workflows/ansible-deploy.yml/badge.svg)](https://github.com/ilyalinhnguyen/DevOps-Core-Course/actions/workflows/ansible-deploy.yml)

This directory houses Ansible playbooks and roles for infrastructure
provisioning and application deployment, covering the material from Labs 5–6.

---

## Directory Structure

| Path                 | Description                                                           |
| -------------------- | --------------------------------------------------------------------- |
| `ansible.cfg`        | Configuration defaults (inventory, remote_user, privilege escalation) |
| `inventory/`         | Host definitions and group variables (Vault-encrypted secrets)        |
| `group_vars/all.yml` | Shared variables (protected with Ansible Vault)                       |
| `playbooks/`         | `provision.yml`, `deploy.yml`, `site.yml`                             |
| `roles/`             | `common`, `docker`, `web_app`                                         |
| `docs/`              | Lab documentation (`LAB05.md`, `LAB06.md`)                            |

---

## Available Roles

### common

Handles baseline system configuration: apt cache management, package
installation, timezone settings, and optional user creation.

**Available tags:** `packages`, `users`, `common`

### docker

Installs Docker CE and the Compose plugin, ensuring the daemon starts
automatically.

**Available tags:** `docker`, `docker_install`, `docker_config`

### web_app

Deploys the application using Docker Compose with templated configuration.
Includes optional environment cleanup capability.

**Available tags:** `app_deploy`, `compose`, `web_app_wipe`

**Prerequisite:** Requires the `docker` role to be applied first.

---

## Quick Start

### Provision Infrastructure

```bash
ansible-playbook playbooks/provision.yml --ask-vault-pass
```

Executes the common and docker roles to prepare a ready-to-use environment.

### Deploy Application

```bash
ansible-playbook playbooks/deploy.yml --ask-vault-pass
```

Runs the web_app role. Docker gets installed automatically if not already
present.

### Clean Redeployment

```bash
ansible-playbook playbooks/deploy.yml -e "web_app_wipe=true" --ask-vault-pass
```

Removes existing containers and redeploys with a fresh configuration.

---

## Selective Execution

Leverage tags to run specific portions of a playbook:

```bash
# Apply only Docker-related changes
ansible-playbook playbooks/provision.yml --tags "docker"

# Execute everything except common tasks
ansible-playbook playbooks/provision.yml --skip-tags "common"

# Display all available tags without making changes
ansible-playbook playbooks/provision.yml --list-tags
```

---

## Continuous Integration

The
[Ansible Deployment](https://github.com/ilyalinhnguyen/DevOps-Core-Course/actions/workflows/ansible-deploy.yml)
workflow executes the deploy job on a self-hosted runner (your own machine). On
push events (after lint passes), it deploys to localhost using
`inventory/hosts.local.ini`.

### Required Secret

Configure this in **Settings → Secrets and variables → Actions**:

| Secret                   | Purpose                                                 |
| ------------------------ | ------------------------------------------------------- |
| `ANSIBLE_VAULT_PASSWORD` | Unlocks the encrypted variables in `group_vars/all.yml` |
