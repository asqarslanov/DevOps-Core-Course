# Lab 5: Ansible Fundamentals

## Introduction

This document describes the Ansible-based infrastructure automation solution for
provisioning web servers and deploying containerized applications. The
implementation uses a role-based approach with proper separation of concerns,
demonstrating key Ansible concepts including idempotency, handlers, and secure
credential management.

**Environment:**

- Ansible version: 2.16+
- Target: Ubuntu 24.04 LTS (Yandex Cloud VM)
- Authentication: SSH key-based (`ubuntu` user)

---

## 1. Project Structure

The following directory layout follows Ansible best practices:

```
ansible/
├── ansible.cfg              # Ansible configuration
├── inventory/
│   └── hosts.ini            # Static inventory file
├── roles/
│   ├── common/              # System packages and timezone configuration
│   ├── docker/              # Docker Engine installation and setup
│   └── app_deploy/          # Application container deployment
├── playbooks/
│   ├── provision.yml        # System provisioning only
│   ├── deploy.yml           # Application deployment only
│   └── site.yml             # Combined provisioning and deployment
├── group_vars/
│   └── all.yml              # Encrypted variables (Ansible Vault)
├── .vault_pass              # Vault password file
└── docs/
    └── LAB05.md             # This documentation
```

### Why Role-Based Architecture?

Roles were chosen over monolithic playbooks for several reasons:

- **Modularity**: Each role handles a single responsibility, making the codebase
  easier to understand
- **Reusability**: Roles can be shared across different projects (e.g., the
  `docker` role works with any playbook)
- **Maintainability**: Changes to one component don't affect others
- **Testability**: Roles can be tested in isolation

---

## 2. Role Descriptions

### 2.1 Common Role

**Location:** `roles/common/`

**Responsibilities:**

- Updating apt package cache
- Installing essential system packages
- Configuring system timezone

**Variables:**

| Variable          | Description                     | Default                                  |
| ----------------- | ------------------------------- | ---------------------------------------- |
| `common_packages` | List of apt packages to install | `['curl', 'wget', 'git', 'vim', 'htop']` |
| `timezone`        | System timezone                 | `UTC`                                    |

**Idempotency:** Uses `cache_valid_time` to avoid redundant cache updates within
the specified time window.

---

### 2.2 Docker Role

**Location:** `roles/docker/`

**Responsibilities:**

- Installing Docker CE from the official repository
- Ensuring Docker service is running and enabled on boot
- Adding specified user to the docker group
- Installing Python Docker library for Ansible modules

**Variables:**

| Variable          | Description                     | Default                                                                                            |
| ----------------- | ------------------------------- | -------------------------------------------------------------------------------------------------- |
| `docker_user`     | Username to add to docker group | `ubuntu`                                                                                           |
| `docker_packages` | Docker packages to install      | `['docker-ce', 'docker-ce-cli', 'containerd.io', 'docker-buildx-plugin', 'docker-compose-plugin']` |

**Handlers:**

- `restart docker`: Restarts the Docker service (triggered when Docker packages
  are updated)

**Dependencies:** Requires the `common` role for prerequisite packages
(`ca-certificates`, `curl`, `gnupg`)

**Note:** The Docker GPG key is added using `get_url` module to
`/etc/apt/keyrings/`, as `apt_key` is deprecated in newer Ansible versions.

---

### 2.3 Application Deployment Role

**Location:** `roles/app_deploy/`

**Responsibilities:**

- Authenticating with Docker Hub
- Pulling the application Docker image
- Removing any existing container with the same name
- Running a new container with specified configuration
- Verifying application health

**Variables:**

| Variable             | Description              | Default                         |
| -------------------- | ------------------------ | ------------------------------- |
| `app_port`           | Host port for container  | `5000`                          |
| `app_container_name` | Container name           | `devops-python-app`             |
| `docker_image`       | Full image name          | `asqarslanov/devops-python-app` |
| `docker_image_tag`   | Image tag                | `latest`                        |
| `app_restart_policy` | Container restart policy | `always`                        |
| `app_env`            | Environment variables    | `{}`                            |

**Vault-Encrypted Variables:**

- `dockerhub_username`: Docker Hub username
- `dockerhub_password`: Docker Hub access token

**Handlers:**

- `restart app container`: Restarts the application container

**Dependencies:** Requires the `docker` role to be run first (Docker must be
installed)

---

## 3. Idempotency Verification

Idempotency means running a playbook multiple times produces the same result as
running it once. The following tests demonstrate this property.

### 3.1 Initial Playbook Execution

```
PLAY [Provision web servers] *********************************************

TASK [Gathering Facts] ***************************************************
ok: [lab4-vm]

TASK [common : Update apt cache] ******************************************
changed: [lab4-vm]

TASK [common : Install common packages] **********************************
changed: [lab4-vm]

TASK [common : Set timezone] **********************************************
changed: [lab4-vm]

TASK [docker : Add Docker GPG key] ****************************************
changed: [lab4-vm]

TASK [docker : Add Docker repository] ************************************
changed: [lab4-vm]

TASK [docker : Install Docker packages] **********************************
changed: [lab4-vm]

TASK [docker : Ensure Docker service is running and enabled] ************
ok: [lab4-vm]

TASK [docker : Add user to docker group] **********************************
changed: [lab4-vm]

TASK [docker : Install python3-docker] ************************************
changed: [lab4-vm]

RUNNING HANDLER [docker : restart docker] ********************************
changed: [lab4-vm]

PLAY RECAP ****************************************************************
lab4-vm : ok=13   changed=9    unreachable=0    failed=0
```

**Result:** 9 tasks made changes (installed packages, added repository,
configured Docker)

### 3.2 Subsequent Playbook Execution

```
PLAY [Provision web servers] *********************************************

TASK [Gathering Facts] ***************************************************
ok: [lab4-vm]

TASK [common : Update apt cache] ******************************************
ok: [lab4-vm]

TASK [common : Install common packages] ************************************
ok: [lab4-vm]

TASK [common : Set timezone] **********************************************
ok: [lab4-vm]

TASK [docker : Add Docker GPG key] ****************************************
ok: [lab4-vm]

TASK [docker : Add Docker repository] ************************************
ok: [lab4-vm]

TASK [docker : Install Docker packages] ***********************************
ok: [lab4-vm]

TASK [docker : Ensure Docker service is running and enabled] *************
ok: [lab4-vm]

TASK [docker : Add user to docker group] **********************************
ok: [lab4-vm]

TASK [docker : Install python3-docker] ************************************
ok: [lab4-vm]

PLAY RECAP ****************************************************************
lab4-vm : ok=12   changed=0    unreachable=0    failed=0
```

**Result:** 0 tasks made changes — all resources already in desired state

### 3.3 Analysis

The second run shows zero changes because:

1. **APT cache** is still valid (within `cache_valid_time` window)
2. **Packages** are already installed (`state: present` doesn't reinstall)
3. **Docker service** is running and enabled (`state: started`, `enabled: yes`)
4. **User** is already in the docker group
5. **Docker repository** is already configured

**Techniques ensuring idempotency:**

- Using declarative modules (`apt`, `service`, `user`) instead of shell commands
- Setting `state: present` for packages, `state: started` for services
- Implementing `cache_valid_time` to skip redundant cache updates
- Checking current state before making changes

---

## 4. Secure Credential Management

Sensitive data (Docker Hub credentials) is stored using Ansible Vault.

### 4.1 Vault Setup

The vault password is stored in a separate file (excluded from version control
via `.gitignore`):

```bash
# ansible.cfg
[defaults]
vault_password_file = .vault_pass
```

### 4.2 Creating Encrypted Variables

```bash
ansible-vault create group_vars/all.yml
ansible-vault edit group_vars/all.yml
ansible-vault view group_vars/all.yml
```

### 4.3 Encrypted File Structure

The `group_vars/all.yml` file contains:

```yaml
---
# Public variables
docker_image: asqarslanov/devops-python-app
docker_image_tag: latest
app_port: 5000
app_container_name: devops-python-app

# Encrypted vault variables
$ANSIBLE_VAULT;1.1;AES256
62383338633566343434313433653932613136383832326438...
```

### 4.4 Why Ansible Vault?

- **Encryption at rest**: Credentials are AES256-encrypted on disk
- **Version control safe**: Encrypted files can be committed to git
- **Runtime decryption**: Ansible automatically decrypts using the password file
- **No plaintext secrets**: Eliminates the risk of accidentally committing
  credentials

---

## 5. Deployment Verification

### 5.1 Deployment Playbook Execution

```
PLAY [Deploy application] ***********************************************

TASK [Gathering Facts] ***************************************************
ok: [lab4-vm]

TASK [app_deploy : Log in to Docker Hub] ********************************
ok: [lab4-vm]

TASK [app_deploy : Pull Docker image] ************************************
ok: [lab4-vm]

TASK [app_deploy : Remove old container] ********************************
changed: [lab4-vm]

TASK [app_deploy : Run application container] ****************************
changed: [lab4-vm]

TASK [app_deploy : Wait for application to start] ***********************
ok: [lab4-vm]

TASK [app_deploy : Verify health endpoint] *******************************
ok: [lab4-vm]

PLAY RECAP ****************************************************************
lab4-vm : ok=8    changed=2    unreachable=0    failed=0
```

### 5.2 Container Status

```
CONTAINER ID   IMAGE                                          COMMAND         CREATED        STATUS                    PORTS
816f4c38415c   asqarslanov/devops-python-app:latest          python app.py  10 min ago    Up 10 min (healthy)      0.0.0.0:5000->5000/tcp
```

### 5.3 Health Check Response

```json
{
  "status": "healthy",
  "timestamp": "2026-02-23T14:40:04.087099+00:00",
  "uptime_seconds": 638
}
```

Full service information:

- **Framework:** Flask
- **Python version:** 3.13.12
- **Architecture:** x86_64
- **CPU cores:** 2

---

## 6. Design Decisions

### 6.1 Role Separation

**Decision:** Split functionality into three distinct roles (`common`, `docker`,
`app_deploy`)

**Rationale:** Each role represents a distinct layer of the stack. This
separation allows:

- Independent updates to each layer
- Reusing Docker role in other projects
- Testing each role in isolation

### 6.2 Handler Usage

**Decision:** Use handlers for service restarts

**Rationale:** Handlers run only when notified, preventing unnecessary service
restarts. The Docker service only restarts when packages are actually updated,
not on every run.

### 6.3 Static Inventory

**Decision:** Use static inventory file instead of dynamic inventory

**Rationale:** For a single VM environment, static inventory is simpler and
sufficient. Dynamic inventory would add unnecessary complexity for this use
case.

### 6.4 Vault for Credentials

**Decision:** Encrypt credentials with Ansible Vault

**Rationale:** Vault provides enterprise-grade encryption while maintaining
Ansible's ability to use the variables at runtime. This is superior to
environment variables or external secrets management for this lab's scope.

---

## 7. Technical Challenges Encountered

### Challenge 1: Deprecated APT Key Module

**Issue:** The `apt_key` module is deprecated in Ansible 2.16+

**Solution:** Used `get_url` module to download the GPG key directly to
`/etc/apt/keyrings/docker.gpg`

```yaml
- name: Add Docker GPG key
  get_url:
    url: https://download.docker.com/linux/ubuntu/gpg
    dest: /etc/apt/keyrings/docker.asc
    mode: "0644"
```

### Challenge 2: Python Docker Module on Target

**Issue:** Ansible's `docker_container` and `docker_image` modules require
`python3-docker` package on the target host

**Solution:** Added a task in the Docker role to install `python3-docker` via
pip after Docker is installed

---

## 8. Conclusion

This implementation demonstrates core Ansible concepts:

- **Idempotency** through declarative modules and proper state management
- **Role-based organization** for maintainable and reusable playbooks
- **Secure credential handling** via Ansible Vault
- **Handler-driven service management** for efficient restarts

The solution is production-ready and can serve as a foundation for more complex
infrastructure automation tasks.
