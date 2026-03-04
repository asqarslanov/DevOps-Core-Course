# Lab 6: Advanced Ansible & CI/CD — Implementation Notes

---

## Overview

This lab extends the automation framework established in Lab 5 with several
significant enhancements:

- **Block constructs** with error handling mechanisms (rescue/always sections)
  and tag-based filtering in `common` and `docker` roles
- **Docker Compose integration**: the application deployment role was
  restructured from `app_deploy` to `web_app`, now leveraging Jinja2 templates
  for the Compose file and executing `docker compose up -d` on remote hosts
- **Controlled cleanup**: dual-gated wipe mechanism requiring both a variable
  (`web_app_wipe`) and a tag for execution
- **Automated pipelines**: GitHub Actions workflow that lints playbooks on
  pushes/PRs and optionally deploys when secrets are properly configured

**Technology Stack:** Ansible 2.16+, Docker Compose plugin, Jinja2 templating
engine, GitHub Actions

---

## Feature 1: Block Constructs and Tagging

### Block Implementation in the Common Role

The file `roles/common/tasks/main.yml` demonstrates block-based task
organization:

**Package Management Section:**

- Refreshes apt cache, installs configured packages, adjusts system timezone
- **Rescue handler:** Executes `apt-get update --fix-missing` if the primary
  operation fails
- **Always handler:** Records completion status to
  `/tmp/ansible-common-packages.log`
- Tagged with `packages` and `common`
- Privilege escalation (`become`) is applied at the block level

**User Management Section:**

- Provisions user accounts from the `common_users` list when items exist
- **Always handler:** Logs user-related operations to
  `/tmp/ansible-common-users.log`
- Tagged with `users` and `common`

### Block Implementation in the Docker Role

The file `roles/docker/tasks/main.yml` showcases advanced block patterns:

**Installation Section:**

- Handles dependency installation, GPG keyring setup, Docker repository
  configuration, and package installation
- **Rescue handler:** Introduces a 10-second delay, then retries apt update and
  GPG key retrieval to handle transient network issues
- **Always handler:** Ensures the Docker daemon is running and configured to
  start on boot
- Tagged with `docker`, `docker_install`

**Configuration Section:**

- Adds designated users to the docker group
- Grants docker access to the `ansible_user` account
- Installs the python3-docker Python library
- **Always handler:** Confirms Docker service is enabled
- Tagged with `docker`, `docker_config`

### Tagging Strategy

| Tag              | Function                                       |
| ---------------- | ---------------------------------------------- |
| `packages`       | Package installation within the common role    |
| `users`          | User account management within the common role |
| `common`         | Complete common role execution                 |
| `docker`         | Complete docker role execution                 |
| `docker_install` | Docker installation exclusively                |
| `docker_config`  | Docker configuration exclusively               |
| `app_deploy`     | web_app deployment tasks                       |
| `compose`        | Docker Compose operations                      |
| `web_app_wipe`   | Environment cleanup operations                 |

### Practical Execution

```bash
# Execute only docker-related operations
ansible-playbook playbooks/provision.yml --tags "docker"

# Omit common role from execution
ansible-playbook playbooks/provision.yml --skip-tags "common"

# Target package installation across all roles
ansible-playbook playbooks/provision.yml --tags "packages"

# Enumerate all available tags without making changes
ansible-playbook playbooks/provision.yml --list-tags
```

### Research Findings

- **Rescue block failure:** If the rescue section itself encounters an error,
  the entire play fails since rescue blocks cannot be nested within each other
- **Nested blocks:** Ansible supports hierarchical block structures, though
  excessive nesting reduces maintainability
- **Tag inheritance:** Applying a tag to a block propagates that tag to all
  contained tasks, including rescue and always sections

---

## Feature 2: Transition to Docker Compose

### Role Restructuring

The deployment role was renamed from `app_deploy` to `web_app` for improved
clarity. The deployment playbook now references this updated role identifier.

### Template Architecture

The template file `roles/web_app/templates/docker-compose.yml.j2` accepts the
following parameters:

- `app_name` — Application identifier
- `docker_image` — Container image reference
- `docker_tag` — Specific image version tag
- `app_port` — External port mapping
- `app_internal_port` — Container-internal port number
- `app_env` — Optional environment variables dictionary

The rendered template produces a Compose configuration defining one service with
port mappings, conditional environment variables, and a restart policy set to
`unless-stopped`.

### Role Dependencies

The file `roles/web_app/meta/main.yml` declares:

```yaml
dependencies:
  - role: docker
```

This ensures Docker Engine and the Compose plugin are present before the web_app
role attempts deployment.

### Deployment Workflow

1. Creates the target directory (default: `/opt/devops-app`)
2. Renders the Compose template into that directory
3. Executes `docker compose pull && docker compose up -d` within the directory
4. Validates port availability and checks the `/health` endpoint

### Variable Sources

**Default values** (`roles/web_app/defaults/main.yml`):

- `app_name`, `docker_image`, `docker_tag`, `app_port`, `app_internal_port`
- `compose_project_dir`, `app_restart_policy`, `app_env`, `web_app_wipe`

**Sensitive data:** Docker Hub credentials and application-specific secrets are
stored in group_vars and encrypted using Ansible Vault.

### Comparative Analysis

| Aspect           | Lab 5 Approach            | Lab 6 Approach                   |
| ---------------- | ------------------------- | -------------------------------- |
| Authentication   | `docker_login` module     | Credentials embedded in template |
| Image retrieval  | `docker_image` module     | `docker compose pull`            |
| Container launch | `docker_container` module | `docker compose up -d`           |
| Configuration    | Direct module parameters  | Templated Compose file           |

This approach proves more declarative and accommodates multi-container
architectures more gracefully.

---

## Feature 3: Controlled Wipe Mechanism

### Implementation Details

The file `roles/web_app/tasks/wipe.yml` contains:

- A consolidated block that:
  - Stops Docker Compose services
  - Deletes the rendered Compose file
  - Removes the entire project directory
  - Outputs a success confirmation message
- Executes conditionally when `web_app_wipe | default(false) | bool` evaluates
  to true
- Tagged with `web_app_wipe`
- Uses `ignore_errors: true` on removal operations to gracefully handle
  already-deleted resources

### Integration Point

The wipe task is included as the first operation in
`roles/web_app/tasks/main.yml`:

```yaml
- include_tasks: wipe.yml
  tags: web_app_wipe
```

This positioning ensures cleanup executes before deployment when triggered,
enabling single-command environment rebuilding.

### Safety Mechanisms

The wipe functionality requires two conditions simultaneously:

1. **Variable:** The flag `web_app_wipe=true` must be explicitly passed
2. **Tag:** The tag `--tags web_app_wipe` enables wipe-only execution mode

This dual-verification approach prevents unintended data destruction.

### Test Scenarios

| Command                                      | Result                               |
| -------------------------------------------- | ------------------------------------ |
| `ansible-playbook playbooks/deploy.yml`      | Standard deployment without cleanup  |
| `-e "web_app_wipe=true" --tags web_app_wipe` | Only cleanup executes                |
| `-e "web_app_wipe=true"`                     | Cleanup followed by fresh deployment |

---

## Feature 4: CI/CD Pipeline Integration

### Workflow Configuration

The `.github/workflows/ansible-deploy.yml` workflow activates on:

- Push or pull request events targeting `main`, `master`, or `lab06` branches
- Modifications affecting `ansible/**` paths or the workflow file itself

### Pipeline Jobs

**Lint Job:**

- Ubuntu runner with Python 3.12 environment
- Installs Ansible and ansible-lint
- Executes `ansible-lint playbooks/*.yml` from the ansible directory

**Deploy Job:**

- Conditional execution: runs exclusively on push events when
  `ANSIBLE_VAULT_PASSWORD` is configured
- Retrieves repository code
- Installs Ansible
- Establishes SSH connectivity using `SSH_PRIVATE_KEY`
- Runs the deployment playbook with vault password
- Validates deployment by querying `VM_HOST:8000` and `VM_HOST:8000/health`

### Required Secrets

| Secret                   | Purpose                                                   |
| ------------------------ | --------------------------------------------------------- |
| `ANSIBLE_VAULT_PASSWORD` | Decrypts the encrypted group_vars/all.yml file            |
| `SSH_PRIVATE_KEY`        | Authenticates to the target virtual machine               |
| `VM_HOST`                | Target hostname or IP address for SSH and curl operations |

For self-hosted runner implementations, only `ANSIBLE_VAULT_PASSWORD` is
necessary.

### Verification Steps

The workflow provides:

- Pass/fail status indicators for lint and deploy jobs in the Actions interface
- Comprehensive logs for each pipeline step
- Optional status badge suitable for README documentation

---

## Feature 5: Documentation

Inline code documentation exists in the following locations:

- `roles/common/tasks/main.yml` — Block structure and tag definitions
- `roles/docker/tasks/main.yml` — Block logic with rescue and always handlers
- `roles/web_app/tasks/main.yml` and `wipe.yml` — Cleanup workflow
  implementation
- `roles/web_app/templates/docker-compose.yml.j2` — Template variable
  descriptions
- `roles/web_app/defaults/main.yml` — Wipe flag configuration
- `.github/workflows/ansible-deploy.yml` — Workflow triggers and step details

---

## Validation Outcomes

### Tagging Verification

`ansible-playbook playbooks/provision.yml --list-tags` correctly displays all
defined tags.

Selective execution functions as designed:

- `--tags "docker"` executes only Docker-related tasks
- `--skip-tags "common"` excludes common role tasks

### Docker Compose Behavior

- Initial deployment shows changes (directory creation, template rendering,
  compose up)
- Subsequent executions remain idempotent without spurious modifications

### Wipe Operations

All four test scenarios perform as expected:

1. Standard deployment proceeds without cleanup
2. Wipe-only mode completely removes the environment
3. Combined wipe-and-deploy creates a fresh installation
4. Tag-only execution respects variable conditions

### Application Verification

Following successful deployment:

- `curl http://<VM>:5000` returns the expected response
- `curl http://<VM>:5000/health` confirms health check functionality

---

## Obstacles Encountered and Resolutions

### Common Role Rescue Strategy

Initially attempted using the `apt` module with `cache_valid_time: 0` within the
rescue block. Switched to the `command` module executing
`apt-get update --fix-missing` to align with lab requirements.

### Docker Compose Execution via Ansible

Avoided dependency on the `community.docker.docker_compose_v2` collection
installed on the controller. Instead, templated the Compose file locally and
utilized the `command` module to execute
`docker compose pull && docker compose up -d` on remote hosts. This approach
works with the standard Docker Compose plugin installed on managed nodes.

### Compose Idempotency

Implemented `changed_when` conditional logic checking stdout for "Creating",
"Starting", "Pulling", or "Recreating" to accurately report only genuine
modifications.

### CI/CD Deployment Safety

Made the deploy job conditional on `secrets.ANSIBLE_VAULT_PASSWORD != ''` and
added `|| true` to verification steps. This prevents workflow failures when the
target VM is not configured or unreachable from GitHub's infrastructure.

---

## Technical Research Summary

### Restart Policy Differences

- **`always`:** Container restarts after any stop operation, including manual
  `docker stop` commands
- **`unless-stopped`:** Container does not restart after manual stop until
  system reboot or subsequent deployment

### Network Configuration Options

- **Default bridge:** Per-project network isolated from other Compose projects
- **Custom networks:** Enable service-to-service DNS resolution and stronger
  network isolation

### Vault-Encrypted Variables in Templates

Yes, Vault-encrypted variables resolve on the controller before template
rendering. Decrypted values appear in the generated file on target hosts. Use
Vault exclusively for values safe to exist on managed nodes, and implement
appropriate file permission restrictions.

### Rationale for Variable AND Tag Wipe Protection

- **Variable:** Prevents accidental wipe when the role is included by other
  playbooks inadvertently
- **Tag:** Ensures wipe does not execute unless explicitly requested by the
  operator
- Combined approach provides defense in depth against unintended data loss

### Wipe Placement Before Deployment in main.yml

Positioning wipe before deployment enables single-command clean reinstallation:
passing `-e "web_app_wipe=true"` performs wipe then deployment without requiring
multiple playbook invocations.

### GitHub Secrets Security Considerations

Secrets are encrypted at rest and excluded from workflow logs. Recommended
practices include regular key rotation, using deploy keys with minimal
permissions, and considering short-lived credentials where feasible.
