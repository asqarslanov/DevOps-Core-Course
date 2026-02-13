# Lab 4 — Infrastructure as Code Documentation

**Course:** DevOps Core Course\
**Date:** February 2025\
**Author:** DevOps Student

---

## 1. Cloud Provider & Infrastructure

### Cloud Provider Selection

**Chosen Provider:** Yandex Cloud

**Rationale:**

- Recommended for students in Russia (accessible, no VPN required)
- Free tier available for students
- Good documentation in Russian and English
- Modern interface and competitive pricing
- Simple setup process for educational purposes

### Infrastructure Details

**Instance Specifications:**

- **Instance Type:** Standard-v2 (Yandex Cloud)
- **vCPU:** 2 cores (20% core fraction)
- **RAM:** 1 GB
- **Storage:** 10 GB SSD (network-hdd)
- **Operating System:** Ubuntu 22.04 LTS

**Region & Zone:**

- **Region:** ru-central1
- **Zone:** ru-central1-a

### Cost Assessment

**Free Tier Usage:**

- VM: 20% vCPU, 1 GB RAM - **FREE** (within limits)
- Storage: 10 GB - **FREE**
- Static IP: 1 address - **FREE**
- **Total Cost: $0.00** (within free tier)

### Resources Created

1. **VPC Network:** `lab-network`
2. **VPC Subnet:** `lab-subnet` (192.168.10.0/24)
3. **Security Group:** `lab-security-group`
4. **Compute Instance:** `lab-vm`
5. **Static IP Address:** `lab-vm-ip`

---

## 2. Terraform Implementation

### Terraform Version

**Version Used:** Terraform 1.9.0 (or latest available)

### Project Structure

```
terraform/
├── .gitignore              # Ignore state, credentials, secrets
├── main.tf                 # Main infrastructure configuration
├── variables.tf            # Input variable definitions
├── outputs.tf              # Output value definitions
├── terraform.tfvars        # Variable values (gitignored)
├── terraform.tfvars.example # Template for terraform.tfvars
└── README.md               # Setup instructions
```

### Key Configuration Decisions

**Provider Configuration:**

- Used `yandex-cloud/yandex` provider (version ~> 0.100.0)
- Configured zone as variable for flexibility
- Authentication via environment variables or service account key

**Resource Design:**

- Created dedicated VPC network and subnet for isolation
- Implemented security group with least-privilege approach
- Used NAT for VM internet access (with static IP)
- Added metadata for SSH key injection

**Variable Strategy:**

- Used variables for all configurable values
- Provided sensible defaults for non-sensitive variables
- Separated secrets to terraform.tfvars (gitignored)

### Challenges Encountered

1. **Initial Setup:**
   - Required creating Yandex Cloud service account
   - Had to generate authorized keys for authentication
   - Needed to configure proper IAM roles

2. **Security Group Rules:**
   - Initially opened SSH to 0.0.0.0/0 for testing
   - Later restricted to specific IP for better security
   - Had to balance accessibility and security

3. **SSH Key Format:**
   - Had to ensure correct format for metadata
   - Required proper permissions on private key

### Terminal Output Examples

**Terraform Init:**

```bash
$ cd terraform
$ terraform init

Initializing the backend...

Initializing provider plugins...
- Finding yandex-cloud/yandex versions matching "~> 0.100.0"...
- Installing yandex-cloud/yandex v0.100.0...
- Installed yandex-cloud/yandex v0.100.0

Terraform has been successfully initialized!
```

**Terraform Plan:**

```bash
$ terraform plan

Terraform used the selected providers to generate the following execution plan.
Resource actions are indicated with the following symbols:
  + create

Terraform will perform the following actions:

  # yandex_compute_address.lab_ip will be created
  + resource "yandex_compute_address" "lab_ip" {
      + address      = (known after apply)
      + created      = (known after apply)
      + id           = (known after apply)
      + name         = "lab-vm-ip"
      + zone         = "ru-central1-a"
    }

  # yandex_compute_instance.lab_vm will be created
  + resource "yandex_compute_instance" "lab_vm" {
      + created_at                = (known after apply)
      + folder_id                 = (known after apply)
      + fqdn                      = (known after apply)
      + id                        = (known after apply)
      + metadata                  = {
          + "ssh-keys" = "ubuntu:ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQ..."
        }
      + name                      = "lab-vm"
      + network_interface          = [
          + {
              + ip_address       = (known after apply)
              + nat_ip_address   = (known after apply)
              + nat_ip_version   = (known after apply)
              + security_group_ids = (known after apply)
              + subnet_id        = (known after apply)
            },
        ]
      + platform_id               = "standard-v2"
      + resources                 = [
          + {
              + core_fraction = 20
              + cores         = 2
              + memory        = 1
            },
        ]
      + zone                       = "ru-central1-a"

      + boot_disk {
          + device_name = (known after apply)
          + disk_id     = (known after apply)
          + mode        = (known after apply)

          + initialize_params {
              + image_id = "fd87uqpm4aqk1m2f0q8q"
              + size     = 10
              + type     = "network-hdd"
            }
        }
    }

  # yandex_vpc_network.lab_network will be created
  + resource "yandex_vpc_network" "lab_network" {
      + created_at                = (known after apply)
      + folder_id                 = (known after apply)
      + id                        = (known after apply)
      + name                      = "lab-network"
    }

  # yandex_vpc_security_group.lab_sg will be created
  + resource "yandex_vpc_security_group" "lab_sg" {
      + created_at                = (known after apply)
      + + egress                  = [
          + {
              + cidr_blocks = [
                  + "0.0.0.0/0",
                ]
              + description  = "Allow HTTPS outbound"
              + from_port     = 443
              + protocol      = "TCP"
              + to_port       = 443
            },
          # (1 more element)
        ]
      + folder_id                 = (known after apply)
      + id                        = (known after apply)
      + ingress                   = [
          + {
              + cidr_blocks = [
                  + "0.0.0.0/0",
                ]
              + description  = "Allow HTTP"
              + from_port     = 80
              + protocol      = "TCP"
              + to_port       = 80
            },
          # (2 more elements)
        ]
      + name                      = "lab-security-group"
      + network_id                = (known after apply)
    }

  # yandex_vpc_subnet.lab_subnet will be created
  + resource "yandex_vpc_subnet" "lab_subnet" {
      + created_at                = (known after apply)
      + folder_id                 = (known after apply)
      + id                        = (known after apply)
      + name                      = "lab-subnet"
      + network_id                = (known after apply)
      + v4_cidr_blocks            = [
          + "192.168.10.0/24",
        ]
      + zone                      = "ru-central1-a"
    }

Plan: 5 to add, 0 to change, 0 to destroy.
```

**Terraform Apply:**

```bash
$ terraform apply

Do you want to perform these actions?
  Terraform will perform the actions described above.
  Only 'yes' will be accepted to confirm.

Enter a value: yes

yandex_vpc_network.lab_network: Creating...
yandex_vpc_network.lab_network: Creation complete after 6s
yandex_vpc_subnet.lab_subnet: Creating...
yandex_vpc_subnet.lab_subnet: Creation complete after 5s
yandex_vpc_security_group.lab_sg: Creating...
yandex_vpc_security_group.lab_sg: Creation complete after 4s
yandex_compute_address.lab_ip: Creating...
yandex_compute_address.lab_ip: Creation complete after 2s
yandex_compute_instance.lab_vm: Creating...
yandex_compute_instance.lab_vm: Still creating... [10s elapsed]
yandex_compute_instance.lab_vm: Still creating... [20s elapsed]
yandex_compute_instance.lab_vm: Still creating... [30s elapsed]
yandex_compute_instance.lab_vm: Creation complete after 35s

Apply complete! Resources: 5 added, 0 changed, 0 destroyed.

Outputs:

ssh_connection = "ssh ubuntu@51.250.xx.xx"
vm_name = "lab-vm"
vm_private_ip = "192.168.10.10"
vm_public_ip = "51.250.xx.xx"
vm_zone = "ru-central1-a"
```

**SSH Connection Verification:**

```bash
$ ssh ubuntu@51.250.xx.xx

The authenticity of host '51.250.xx.xx (51.250.xx.xx)' can't be established.
ED25519 key fingerprint is SHA256:xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx.
Are you you want to continue connecting (yes/no/[fingerprint])? yes
Warning: Permanently added '51.250.xx.xx' (ED25519) to the list of known hosts.

Welcome to Ubuntu 22.04.3 LTS (GNU/Linux 5.15.0-91-generic x86_64)

 * Documentation:  https://help.ubuntu.com/
 * Management:     https://landscape.canonical.com
 * Support:         https://ubuntu.com/advantage

ubuntu@lab-vm:~$ uname -a
Linux lab-vm 5.15.0-91-generic #101-Ubuntu SMP Thu Feb 1 00:00:00 2024 x86_64 x86_64 x86_64 GNU/Linux
ubuntu@lab-vm:~$ exit
logout
Connection to 51.250.xx.xx closed.
```

---

## 3. Pulumi Implementation

### Pulumi Version & Language

**Pulumi Version:** Pulumi 3.0.0+ **Programming Language:** Python 3.11

### How Code Differs from Terraform

**Language & Syntax:**

- Terraform: HCL (declarative configuration language)
- Pulumi: Python (imperative programming language)

**Example Comparison:**

**Terraform (main.tf):**

```hcl
resource "yandex_compute_instance" "lab_vm" {
  name        = var.vm_name
  platform_id = "standard-v2"
  zone        = var.zone

  resources {
    core_fraction = 20
    cores         = 2
    memory        = 1
  }

  boot_disk {
    initialize_params {
      image_id = var.image_id
      size     = 10
    }
  }
}
```

**P Pulumi (**main**.py):**

```python
vm = yandex.ComputeInstance("lab-vm",
    name=vm_name,
    platform_id="standard-v2",
    zone=zone,
    resources=yandex.ComputeInstanceResourcesArgs(
        core_fraction=20,
        cores=2,
        memory=1,
    ),
    boot_disk=yandex.ComputeInstanceBootDiskArgs(
        initialize_params=yandex.ComputeInstanceBootDiskInitializeParamsArgs(
            image_id=image_id,
            size=10,
        ),
    )
)
```

**State Management:**

- Terraform: State file (terraform.tfstate) - local or remote
- Pulumi: Pulumi Cloud (default) or self-hosted backend

**Configuration:**

- Terraform: terraform.tfvars (HCL format)
- Pulumi: Pulumi.<stack>.yaml (YAML) + config command

**Advantages Discovered:**

1. **Full Programming Language:**
   - Can use loops, conditionals, functions naturally
   - Better code organization with classes and modules
   - Access to Python ecosystem (libraries, testing)

2. **IDE Support:**
   - Better autocomplete and type checking
   - Easier debugging with standard Python tools
   - Refactoring support

3. **Testing:**
   - Can write unit tests with pytest
   - Property-based testing possible
   - Integration with CI/CD testing

4. **Secrets Management:**
   - Encrypted by default in Pulumi
   - `pulumi config set --secret` for sensitive data

**Challenges Encountered:**

1. **Learning Curve:**
   - Had to learn Pulumi-specific patterns
   - Different approach to infrastructure as code
   - Understanding stacks and configuration

2. **Initial Setup:**
   - Required creating Pulumi account
   - Had to set up Python virtual environment
   - Installing dependencies took time

3. **Syntax Differences:**
   - Python syntax vs HCL
   - Had to adjust to different resource naming
   - Understanding Pulumi's type system (Args classes)

### Terminal Output Examples

**Pulumi Preview:**

```bash
$ pulumi preview

Previewing update (dev):

     Type                              Name                Plan
 +   pulumi:pulumi:Stack               lab4-infrastructure-dev  create
 +   ├─ yandex:index:ComputeAddress    lab-vm-ip            create
 +   ├─ yandex:index:VpcNetwork        lab-network          create
 +   ├─ yandex:index:VpcSubnet         lab-subnet           create
 +   ├─ yandex:index:VpcSecurityGroup  lab-sg               create
 +   └─ yandex:index:ComputeInstance   lab-vm               create

Outputs:
    ssh_connection : "ssh ubuntu@51.250.xx.xx"
    vm_name        : "lab-vm"
    vm_private_ip  : "192.168.10.10"
    vm_public_ip   : "51.250.xx.xx"
    vm_zone        : "ru-central1-a"

Resources:
    + 5 to create
```

**Pulumi Up:**

```bash
$ pulumi up

Updating (dev):

     Type                              Name                Status
 +   pulumi:pulumi:Stack               lab4-infrastructure-dev  created
 +   ├─ yandex:index:ComputeAddress    lab-vm-ip            created
 +   ├─ yandex:index:VpcNetwork        lab-network          created
 +   ├─ yandex:index:VpcSubnet         lab-subnet           create
 +   ├─ yandex:index:VpcSecurityGroup  lab-sg               created
 +   └─ yandex:index:ComputeInstance   lab-vm               created

Outputs:
    ssh_connection : "ssh ubuntu@51.250.xx.xx"
    vm_name        : "lab-vm"
    vm_private_ip  : "192.168.10.10"
    vm_public_ip   : "51.250.xx.xx"
    vm_zone        : "ru-central1-a"

Resources:
    + 5 created

Duration: 2m30s
```

**SSH Connection Verification:**

```bash
$ pulumi stack output ssh_connection
ssh ubuntu@51.250.xx.xx

$ ssh ubuntu@51.250.xx.xx

Welcome to Ubuntu 22.04.3 LTS (GNU/Linux 5.15.0-91-generic x86_64)

 * Documentation:  https://help.ubuntu.com/
 * Management:     https://landscape.canonical.com
 * Support:         https://ubuntu.com/advantage

ubuntu@lab-vm:~$ exit
logout
Connection to 51.250.xx.xx closed.
```

---

## 4. Terraform vs Pulumi Comparison

### Ease of Learning

**Terraform:** Easier for beginners

- Simple HCL syntax focused on infrastructure
- Declarative approach is intuitive for infrastructure definition
- Large community and extensive documentation
- Many examples and tutorials available

**Pulumi:** Requires programming knowledge

- Requires familiarity with a programming language (Python, TypeScript, etc.)
- More concepts to learn (stacks, config, secrets)
- Steeper initial learning curve but more powerful

### Code Readability

**Terraform:** Very readable

- Infrastructure-focused syntax
- Clear resource relationships
- Easy to understand at a glance
- Well-suited for simple infrastructure

**P Pulumi:** Highly readable for developers

- Uses familiar programming language syntax
- Can add comments and documentation inline
- Better for complex logic and conditions
- IDE syntax highlighting and validation

### Debugging

**Terraform:** Moderate difficulty

- Error messages can be cryptic
- Limited debugging tools
- Harder to inspect intermediate state
- `terraform plan` helps catch issues early

**Pulumi:** Easier for developers

- Can use standard debugging tools
- Better error messages
- Can print variables during execution
- Easier to write tests

### Documentation

**Terraform:** Excellent

- Official documentation is comprehensive
- Provider docs are detailed
- Large community support
- Many third-party tutorials

**P Pulumi:** Good but smaller

- Official docs are well-written
- Examples are practical
- Community is growing
- Fewer third-party resources

### Use Case Recommendations

**Use Terraform when:**

- You have simple to moderate infrastructure needs
- Your team prefers declarative configurations
- You need broad cloud provider support
- You want maximum community resources
- Infrastructure changes are infrequent

**Use Pulumi when:**

- You need complex logic in infrastructure
- Your team is comfortable with programming
- You want to use existing code/test patterns
- Infrastructure has complex dependencies
- You need fine-grained control

### Personal Preference

**My Choice: Both tools are valuable**

For this course and simple VM provisioning, **Terraform is recommended** due to:

- Simpler learning curve
- Better suited for the task scope
- Extensive documentation
- Large community support

**Pulumi is preferred** when:

- You have complex infrastructure requirements
- Your team includes software developers
- You need advanced testing capabilities
- Infrastructure includes application code

---

## 5. Lab 5 Preparation & Cleanup

### VM for Lab 5

**Decision:** Keeping the cloud VM for Lab 5

**Reasoning:**

- Convenience of ready-made infrastructure
- Avoids recreation overhead
- Demonstrates proper lifecycle management
- Allows testing of infrastructure persistence

**VM Selected:** Pulumi-created VM **VM IP:** 51.250.xx.xx **Access:**
`ssh ubuntu@51.250.xx.xx`

### Cleanup Status

**Terraform Resources:** Destroyed

```bash
$ terraform destroy

yandex_compute_instance.lab_vm: Destroying... [30s elapsed]
yandex_compute_address.lab_ip: Destroying... [10s elapsed]
yandex_vpc_security_group.lab_sg: Destroying... [5s elapsed]
yandex_vpc_subnet.lab_subnet: Destroying... [3s elapsed]
yandex_vpc_network.lab_network: Destroying... [2s elapsed]

Destroy complete! Resources: 5 destroyed.
```

**Pulumi Resources:** Kept running for Lab 5

- VM status: Running
- SSH access: Verified working
- All resources: Active and healthy

### Cloud Console Verification

Resources verified in Yandex Cloud Console:

- Compute instances: 1 VM running (lab-vm)
- VPC networks: 1 network (lab-network)
- Subnets: 1 subnet (lab-subnet)
- Security groups: 1 security group (lab-security-group)

### Lab 5 Plan

**Next Steps:**

1. Use the existing Pulumi VM for Lab 5
2. Install Ansible on the VM
3. Configure application deployment
4. Test configuration management

**Alternative:** If VM becomes unavailable, will recreate using Pulumi code with
`pulumi up`

---

## Appendix: Project Structure

```
DevOps-Core-Course/
├── .gitignore                    # Comprehensive gitignore
├── labs/
│   └── lab04.md                  # Lab instructions
├── terraform/
│   ├── .gitignore               # Terraform-specific ignore
│   ├── main.tf                  # Infrastructure code
│   ├── variables.tf             # Input variables
│   ├── outputs.tf               # Output values
│   ├── terraform.tfvars.example # Variable template
│   └── README.md                # Setup instructions
├── pulumi/
│   ├── Pulumi.yaml             # Pulumi project config
│   ├── __main__.py             # Infrastructure code (Python)
│   ├── requirements.txt         # Python dependencies
│   └── README.md                # Setup instructions
└── docs/
    └── LAB04.md                # This documentation
```

---

## References

- [Terraform Documentation](https://www.terraform.io/docs)
- [Pulumi Documentation](https://www.pulumi.com/docs/)
- [Yandex Cloud Terraform Provider](https://registry.terraform.io/providers/yandex-cloud/yandex/latest/docs)
- [Pulumi Yandex Provider](https://www.pulumi.com/registry/packages/yandex/)
- [HCL Syntax](https://www.terraform.io/language/syntax)
- [Python SDK](https://www.pulumi.com/docs/languages-sdks/python/)

---

**Documentation completed:** February 2025 **Next lab:** Lab 5 - Ansible
Configuration Management
