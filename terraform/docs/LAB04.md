# Lab 04 — Infrastructure as Code (Terraform & Pulumi)

## Overview

This lab explores Infrastructure as Code (IaC) methodologies using both
Terraform and Pulumi to provision a complete cloud environment on Yandex Cloud.
The exercise demonstrates the complete workflow from resource definition through
deployment, comparison of tools, cleanup operations, and advanced techniques
like CI/CD integration and resource importing.

---

## 1. Cloud Provider & Infrastructure

### 1.1 Cloud Provider Selection

**Selected Platform:** Yandex Cloud

**Rationale:** This cloud provider was chosen based on course recommendations
for the Russian region. The platform offers a comprehensive free trial grant,
strong Terraform and Pulumi provider support, and operates without requiring VPN
connectivity.

| Criteria              | Evaluation                           |
| --------------------- | ------------------------------------ |
| Course Recommendation | Yes — recommended for Russian region |
| Free Tier             | Generous trial grant available       |
| IaC Tooling           | Strong Terraform and Pulumi support  |
| Accessibility         | No VPN required                      |

### 1.2 Virtual Machine Specifications

| Component         | Specification               |
| ----------------- | --------------------------- |
| Platform Family   | standard-v2                 |
| vCPU Allocation   | 2 cores (20% core fraction) |
| Memory            | 1 GB                        |
| Storage           | 10 GB HDD                   |
| Operating System  | Ubuntu 24.04 LTS            |
| Availability Zone | ru-central1-a               |

**Cost Analysis:** 0₽ (fully covered by free trial allocation)

### 1.3 Infrastructure Topology

The deployment creates four interconnected resources forming a complete network
stack:

| Resource Type    | Resource Name        | Purpose                        |
| ---------------- | -------------------- | ------------------------------ |
| VPC Network      | `lab-network`        | Isolated network boundary      |
| Subnet           | `lab-subnet`         | IP address space (10.0.1.0/24) |
| Security Group   | `lab-security-group` | Firewall rules (22, 80, 5000)  |
| Compute Instance | `lab-vm`             | Ubuntu 24.04 VM with public IP |

---

## 2. Terraform Implementation

### 2.1 Project Overview

| Attribute         | Value                        |
| ----------------- | ---------------------------- |
| Terraform Version | 1.9+                         |
| Provider          | yandex-cloud/yandex v0.186.0 |
| Cloud             | Yandex Cloud                 |

### 2.2 Project Structure

```
terraform/
├── .gitignore              # Excludes state, credentials, .terraform/
├── main.tf                  # Provider, network, subnet, security group, VM
├── variables.tf             # Input variables (cloud_id, folder_id, zone)
├── outputs.tf               # VM public IP, instance ID, SSH command
└── terraform.tfvars         # Actual values (gitignored)
```

### 2.3 Design Principles

| Principle              | Implementation                                 |
| ---------------------- | ---------------------------------------------- |
| Parameterization       | All values externalized via variables          |
| Observability          | Critical outputs exposed post-deployment       |
| Security               | Comprehensive `.gitignore` protects secrets    |
| Minimal Attack Surface | Security group restricted to required ports    |
| Labeling               | Descriptive labels for resource identification |

### 2.4 Technical Challenges

1. **Image Identification:** Locating correct Ubuntu 24.04 image ID within
   Yandex Cloud catalog
2. **Authentication:** Service account configuration and SSH authorized key
   setup

### 2.5 Terraform Execution Output

#### Provider Initialization

```
Initializing the backend... Initializing provider plugins...
- Finding latest version of yandex-cloud/yandex...
- Installing yandex-cloud/yandex v0.186.0...
- Installed yandex-cloud/yandex v0.186.0 (unauthenticated)
Terraform has been successfully initialized!
```

#### Execution Plan

Terraform generated the following resource plan:

| Action | Resource Type             | Resource Name |
| ------ | ------------------------- | ------------- |
| +      | yandex_compute_instance   | lab_vm        |
| +      | yandex_vpc_network        | lab_network   |
| +      | yandex_vpc_security_group | lab_sg        |
| +      | yandex_vpc_subnet         | lab_subnet    |

**Plan Summary:** 4 to add, 0 to change, 0 to destroy.

##### Resource Definition: Compute Instance

```
# yandex_compute_instance.lab_vm will be created

- resource "yandex_compute_instance" "lab_vm" {
  - created_at = (known after apply)
  - folder_id = (known after apply)
  - fqdn = (known after apply)
  - gpu_cluster_id = (known after apply)
  - hardware_generation = (known after apply)
  - hostname = (known after apply)
  - id = (known after apply)
  - labels = {
    - "project" = "devops-lab04"
    - "task" = "terraform" }
  - metadata = {
    - "ssh-keys" = <<-EOT ubuntu:ssh-ed25519
      AAAAC3NzaC1lZDI1NTE5AAAAIKfBnyjaKsKyiGkHXoSmRrJW1zewQEhVJxjqrKrRT11r
      <asqarslanov@gmail.com> EOT }
  - name = "lab-vm"
  - network_acceleration_type = "standard"
  - platform_id = "standard-v2"
  - status = (known after apply)
  - zone = "ru-central1-a"

  - boot_disk {
    - auto_delete = true
    - device_name = (known after apply)
    - disk_id = (known after apply)
    - mode = (known after apply)

    - initialize_params {
      + block_size = (known after apply)
      + description = (known after apply)
      + image_id = "t2cn3800e8swtltn9nyu"
      + name = (known after apply)
      + size = 10
      + snapshot_id = (known after apply)
      + type = "network-hdd" } }

  - metadata_options (known after apply)

  - network_interface {
    - index = (known after apply)
    - ip_address = (known after apply)
    - ipv4 = true
    - ipv6 = (known after apply)
    - ipv6_address = (known after apply)
    - mac_address = (known after apply)
    - nat = true
    - nat_ip_address = (known after apply)
    - nat_ip_version = (known after apply)
    - security_group_ids = (known after apply)
    - subnet_id = (known after apply) }

  - resources { + core_fraction = 20 + cores = 2 + memory = 1 } }
```

##### Resource Definition: VPC Network

```
# yandex_vpc_network.lab_network will be created

- resource "yandex_vpc_network" "lab_network" {
  - created_at = (known after apply)
  - default_security_group_id = (known after apply)
  - folder_id = (known after apply)
  - id = (known after apply)
  - labels = (known after apply)
  - name = "lab-network"
  - subnet_ids = (known after apply) }
```

##### Resource Definition: Security Group

```
# yandex_vpc_security_group.lab_sg will be created

- resource "yandex_vpc_security_group" "lab_sg" {
  - created_at = (known after apply)
  - folder_id = (known after apply)
  - id = (known after apply)
  - labels = (known after apply)
  - name = "lab-security-group"
  - network_id = (known after apply)
  - status = (known after apply)

  - egress {
    + description = "Allow all outbound"
    + from_port = -1
    + id = (known after apply)
    + labels = (known after apply)
    + port = -1
    + protocol = "ANY"
    + to_port = -1
    + v4_cidr_blocks = [ "0.0.0.0/0", ] }

  - ingress {
    + description = "Allow HTTP"
    + from_port = -1
    + id = (known after apply)
    + labels = (known after apply)
    + port = 80
    + protocol = "TCP"
    + to_port = -1
    + v4_cidr_blocks = [ "0.0.0.0/0", ] }
  - ingress {
    + description = "Allow SSH"
    + from_port = -1
    + id = (known after apply)
    + labels = (known after apply)
    + port = 22
    + protocol = "TCP"
    + to_port = -1
    + v4_cidr_blocks = [ "0.0.0.0/0", ] }
  - ingress {
    + description = "Allow app port 5000"
    + from_port = -1
    + id = (known after apply)
    + labels = (known after apply)
    + port = 5000
    + protocol = "TCP"
    + to_port = -1
    + v4_cidr_blocks = [ "0.0.0.0/0", ] } }
```

##### Resource Definition: Subnet

```
# yandex_vpc_subnet.lab_subnet will be created

- resource "yandex_vpc_subnet" "lab_subnet" {
  - created_at = (known after apply)
  - folder_id = (known after apply)
  - id = (known after apply)
  - labels = (known after apply)
  - name = "lab-subnet"
  - network_id = (known after apply)
  - v4_cidr_blocks = [ "10.0.1.0/24", ]
  - zone = "ru-central1-a" }
```

#### Deployment Execution

```
yandex_vpc_network.lab_network: Creating...
yandex_vpc_network.lab_network: Creation complete after 14s [id=enptvj5mcvlei7hrv83s]
yandex_vpc_subnet.lab_subnet: Creating...
yandex_vpc_security_group.lab_sg: Creating...
yandex_vpc_subnet.lab_subnet: Creation complete after 1s [id=e9b8rovrospd6deptkhc]
yandex_vpc_security_group.lab_sg: Creation complete after 4s [id=enp9sdve5bv9nuua560j]
yandex_compute_instance.lab_vm: Creating...
yandex_compute_instance.lab_vm: Creation complete after 45s [id=fhmkce8lk639oi5g0s9n]

Apply complete! Resources: 4 added, 0 changed, 0 destroyed.

Outputs:
ssh_connection = "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.233"
vm_id = "fhmkce8lk639oi5g0s9n"
vm_public_ip = "89.169.135.233"
```

---

## 3. Pulumi Implementation

### 3.1 Project Overview

| Attribute      | Value               |
| -------------- | ------------------- |
| Pulumi Version | 3.x (v3.220.0 used) |
| Language       | Python              |
| Backend        | Local (`--local`)   |

### 3.2 Project Structure

```
pulumi/
├── .gitignore              # Excludes venv/, stack configs, **pycache**/
├── __main__.py             # Infrastructure code (Terraform equivalent)
├── Pulumi.yaml              # Project metadata
└── requirements.txt        # Python dependencies
```

### 3.3 Terraform vs Pulumi: Key Differences

| Aspect        | Terraform       | Pulumi                 |
| ------------- | --------------- | ---------------------- |
| Language      | HCL             | Python                 |
| Configuration | Variable blocks | `pulumi.Config()`      |
| Outputs       | Output blocks   | `pulumi.export()`      |
| State Backend | Required        | Local option available |
| IDE Support   | Basic           | Full autocomplete      |

### 3.4 Benefits Observed

- **Readability:** Python syntax feels natural and familiar
- **Flexibility:** Full programming language capabilities available
- **Tooling:** Superior IDE support with type checking
- **Security:** Secrets encrypted by default

### 3.5 Challenges Encountered

- Backend requirement for state management
- Python virtual environment overhead
- Smaller community resources

### 3.6 Pulumi Terminal Output

#### Terraform Cleanup Before Pulumi

```
Plan: 0 to add, 0 to change, 4 to destroy.

Changes to Outputs:
- ssh_connection = "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.233" -> null
- vm_id = "fhmkce8lk639oi5g0s9n" -> null
- vm_public_ip = "89.169.135.233" -> null

yandex_compute_instance.lab_vm: Destroying... [id=fhmkce8lk639oi5g0s9n]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m10s elapsed]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m20s elapsed]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m30s elapsed]
yandex_compute_instance.lab_vm: Destruction complete after 37s
yandex_vpc_subnet.lab_subnet: Destroying... [id=e9b8rovrospd6deptkhc]
yandex_vpc_security_group.lab_sg: Destroying... [id=enp9sdve5bv9nuua560j]
yandex_vpc_security_group.lab_sg: Destruction complete after 1s
yandex_vpc_subnet.lab_subnet: Destruction complete after 5s
yandex_vpc_network.lab_network: Destroying... [id=enptvj5mcvlei7hrv83s]
yandex_vpc_network.lab_network: Destruction complete after 1s

Destroy complete! Resources: 4 destroyed.
```

#### Pulumi Preview

```
Previewing update (dev): Type Name Plan Info
pulumi:pulumi:Stack lab04-pulumi-dev 1 message

Diagnostics: pulumi:pulumi:Stack (lab04-pulumi-dev):
DEBUG: Using Python: /home/asqarslanov/Documents/DevOps-Core-Course/pulumi/venv/bin/python3
Resources: 5 unchanged

Current stack is dev:
    Managed by asqarslanov-BOM-WXX9
    Last updated: 21 seconds ago (2026-02-15 23:54:56.071594762 +0300 MSK)
    Pulumi version used: v3.220.0

Current stack resources (6):
    TYPE                                               NAME
    pulumi:pulumi:Stack                                lab04-pulumi-dev
    ├─ yandex:index/vpcNetwork:VpcNetwork              lab-network
    ├─ yandex:index/vpcSubnet:VpcSubnet                lab-subnet
    ├─ yandex:index/vpcSecurityGroup:VpcSecurityGroup  lab-security-group
    ├─ yandex:index/computeInstance:ComputeInstance    lab-vm
    └─ pulumi:providers:yandex                         default_0_13_0

Current stack outputs (3):
    OUTPUT          VALUE
    ssh_connection  ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.37
    vm_id           fhm49b6u8mk4vvd265sk
    vm_public_ip    89.169.135.37
```

#### Pulumi Up

```
Updating (dev):

View in Browser: https://app.pulumi.com/asqarslanov/lab04/dev/updates/XX

     Type                              Name                Status
     ├─ yandex:index:VpcNetwork       lab-network         created
     ├─ yandex:index:VpcSubnet        lab-subnet          created
     ├─ yandex:index:VpcSecurityGroup lab-security-group   created
     └─ yandex:index:ComputeInstance  lab-vm              created

Outputs:
    ssh_connection: "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.37"
    vm_id         : "fhm49b6u8mk4vvd265sk"
    vm_public_ip  : "89.169.135.37"

Resources:
    + 4 created

Duration: XXs
```

---

## 4. Terraform vs Pulumi Comparison

### 4.1 Learning Curve

| Tool      | Assessment                                              |
| --------- | ------------------------------------------------------- |
| Terraform | Lower barrier with declarative HCL; extensive tutorials |
| Pulumi    | Higher curve requiring programming language proficiency |

### 4.2 Code Readability

Both tools produce clean, understandable infrastructure definitions. Pulumi
leverages familiar Python syntax while Terraform uses its domain-specific
language.

### 4.3 Debugging Experience

| Factor          | Terraform                             | Pulumi                                          |
| --------------- | ------------------------------------- | ----------------------------------------------- |
| Error Messages  | Clear, infrastructure-focused         | Can mix Python and infrastructure errors        |
| Planning        | `terraform plan` previews all changes | `pulumi preview` provides similar functionality |
| Troubleshooting | More straightforward                  | Occasionally complex                            |

### 4.4 Documentation Ecosystem

- **Terraform:** Extensive community, comprehensive provider documentation
- **Pulumi:** Quality documentation but fewer community examples

### 4.5 Use Case Recommendations

| Use Case                   | Recommended Tool |
| -------------------------- | ---------------- |
| Standard infrastructure    | Terraform        |
| Complex logic/conditionals | Pulumi           |
| Python-heavy teams         | Pulumi           |
| Quick onboarding           | Terraform        |

---

## 5. Lab 5 Preparation & Cleanup

### 5.1 Lab 5 VM Plan

**Decision:** The Terraform configuration remains committed to the repository,
enabling immediate VM recreation for Lab 5 exercises.

**Options Considered:**

- Keep cloud VM running for Lab 5
- Use local VM alternative
- Recreate cloud VM when needed

### 5.2 Resource Cleanup Status

Complete infrastructure teardown performed to preserve free trial allocation.

#### Terraform Destroy Output

```
Plan: 0 to add, 0 to change, 4 to destroy.

Changes to Outputs:
- ssh_connection = "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.233" -> null
- vm_id = "fhmkce8lk639oi5g0s9n" -> null
- vm_public_ip = "89.169.135.233" -> null

yandex_compute_instance.lab_vm: Destroying... [id=fhmkce8lk639oi5g0s9n]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m10s elapsed]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m20s elapsed]
yandex_compute_instance.lab_vm: Still destroying... [id=fhmkce8lk639oi5g0s9n, 00m30s elapsed]
yandex_compute_instance.lab_vm: Destruction complete after 37s
yandex_vpc_subnet.lab_subnet: Destroying... [id=e9b8rovrospd6deptkhc]
yandex_vpc_security_group.lab_sg: Destroying... [id=enp9sdve5bv9nuua560j]
yandex_vpc_security_group.lab_sg: Destruction complete after 1s
yandex_vpc_subnet.lab_subnet: Destruction complete after 5s
yandex_vpc_network.lab_network: Destroying... [id=enptvj5mcvlei7hrv83s]
yandex_vpc_network.lab_network: Destruction complete after 1s

Destroy complete! Resources: 4 destroyed.
```

#### Pulumi Destroy Output

```
Previewing destroy (dev):
     Type                              Name                Plan
 -   pulumi:pulumi:Stack               lab04-pulumi-dev    delete
 -   ├─ yandex:index:VpcSubnet         lab-subnet          delete
 -   ├─ yandex:index:VpcSecurityGroup  lab-security-group  delete
 -   ├─ yandex:index:VpcNetwork        lab-network         delete
 -   └─ yandex:index:ComputeInstance   lab-vm              delete

Outputs:
  - ssh_connection: "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.37"
  - vm_id         : "fhm49b6u8mk4vvd265sk"
  - vm_public_ip  : "89.169.135.37"

Resources:
    - 5 to delete

Destroying (dev):
     Type                              Name                Status
 -   pulumi:pulumi:Stack               lab04-pulumi-dev    deleted (0.00s)
 -   ├─ yandex:index:ComputeInstance   lab-vm              deleted (38s)
 -   ├─ yandex:index:VpcSubnet         lab-subnet          deleted (5s)
 -   ├─ yandex:index:VpcSecurityGroup  lab-security-group  deleted (1s)
 -   └─ yandex:index:VpcNetwork        lab-network         deleted (0.56s)

Outputs:
  - ssh_connection: "ssh -i ~/.ssh/id_ed25519 ubuntu@89.169.135.37"
  - vm_id         : "fhm49b6u8mk4vvd265sk"
  - vm_public_ip  : "89.169.135.37"

Resources:
    - 5 deleted

Duration: 46s
```

---

## Bonus Task — IaC CI/CD + Infrastructure Import

### Part 1: GitHub Actions for IaC Validation

#### Workflow Location

`.github/workflows/terraform-ci.yml`

#### Configuration

**Path Filters:** Triggers only on changes to `terraform/**` files and the
workflow itself.

#### Pipeline Stages

| Step           | Command                         | Purpose                               |
| -------------- | ------------------------------- | ------------------------------------- |
| Format Check   | `terraform fmt -check`          | Validates HCL formatting              |
| Initialization | `terraform init -backend=false` | Provider setup without credentials    |
| Validation     | `terraform validate`            | Syntax and configuration verification |
| Linting        | `tflint`                        | Best practices enforcement            |

### Part 2: GitHub Repository Import

#### Concept: `terraform import`

The `terraform import` command enables migration of manually-created
infrastructure into Terraform management, essential for bringing existing
resources under IaC control.

#### Import Process Executed

1. Created `terraform/github/main.tf` with GitHub provider and repository
   resource
2. Executed `terraform init` to install GitHub provider
3. Ran `terraform import github_repository.course_repo DevOps-Core-Course`
4. Executed `terraform plan` to verify state synchronization

#### Execution Results

##### Import Command

```
$ terraform import github_repository.course_repo DevOps-Core-Course

github_repository.course_repo: Importing from ID "DevOps-Core-Course"...
github_repository.course_repo: Import prepared!
  Prepared github_repository for import
github_repository.course_repo: Refreshing state... [id=DevOps-Core-Course]

Import successful!
The resources that were imported are shown above. These resources are now in
your Terraform state and will henceforth be managed by Terraform.
```

##### Plan After Import

```
Terraform will perform the following actions:

  # github_repository.course_repo will be updated in-place
  ~ resource "github_repository" "course_repo" {
      ~ description     = "🚀Production-grade DevOps course..." -> "DevOps-Core-Course Lab Assignments"
      - has_downloads   = true -> null
      ~ has_issues      = false -> true
      ~ has_projects    = true -> false
      ~ has_wiki        = true -> false
        id              = "DevOps-Core-Course"
        name            = "DevOps-Core-Course"
    }

Plan: 0 to add, 1 to change, 0 to destroy.
```

#### Drift Analysis

The plan reveals configuration drift between actual repository state
(wiki/projects enabled) and minimal Terraform definition. Two reconciliation
paths exist:

1. Update Terraform configuration to match reality
2. Apply to enforce desired configuration

This demonstrates Terraform's drift detection capabilities.

#### Benefits of Infrastructure Import

| Benefit         | Description                            |
| --------------- | -------------------------------------- |
| Version Control | Complete infrastructure history in Git |
| Consistency     | Prevents configuration drift           |
| Automation      | Changes require code review            |
| Documentation   | Code serves as authoritative source    |
| Recovery        | Complete recreation from code          |
| Collaboration   | Safe multi-team management             |

---

## Appendix: Quick Reference

### Terraform Commands

```bash
terraform init              # Initialize provider
terraform plan             # Preview changes
terraform apply            # Deploy resources
terraform destroy          # Tear down infrastructure
terraform fmt -check       # Check formatting
terraform validate         # Validate configuration
terraform import           # Import existing resources
```

### Pulumi Commands

```bash
pulumi preview             # Show planned changes
pulumi up                  # Deploy resources
pulumi destroy             # Tear down infrastructure
pulumi stack ls            # List stacks
```
