# Pulumi Infrastructure for Lab 4

This directory contains Pulumi infrastructure code to provision resources in
Yandex Cloud using Python.

## Prerequisites

1. **Pulumi CLI** (version 3.0.0 or higher)
2. **Python 3.8+**
3. **Yandex Cloud account**
4. **Service account with necessary permissions**
5. **SSH key pair** (for VM access)

## Yandex Cloud Setup

### 1. Create Service Account

1. Go to Yandex Cloud Console → IAM → Service Accounts
2. Create a new service account with the following roles:
   - `compute.admin` (for creating/managing VMs)
   - `vpc.admin` (for creating/managing networks)
3. Generate an authorized key for the service account (JSON format)
4. Save the JSON key file securely

### 2. Configure Authentication

Set up authentication using environment variables:

```bash
export YC_TOKEN=$(yc iam create-token)
export YC_CLOUD_ID="your_cloud_id"
export YC_FOLDER_ID="your_folder_id"
```

Or use a service account key file:

```bash
export YC_KEY_FILE="/path/to/service-account-key.json"
```

### 3. Generate SSH Key Pair

```bash
ssh-keygen -t rsa -b 2048 -f ~/.ssh/id_rsa -N ""
```

## Usage

### 1. Create Python Virtual Environment

```bash
cd pulumi
python3 -m venv venv
source venv/bin/activate  # Linux/Mac
# or
.\venv\Scripts\activate  # Windows
```

### 2. Install Dependencies

```bash
pip install -r requirements.txt
```

### 3. Initialize Pulumi

```bash
pulumi login  # Login to Pulumi (use Pulumi Cloud or self-hosted)
pulumi new python  # Create new Pulumi project (if starting fresh)
```

### 4. Configure Stack

Set configuration values:

```bash
pulumi config set zone ru-central1-a
pulumi config set vm_name lab-vm
pulumi config set vm_user ubuntu
pulumi config set my_ip 0.0.0.0/0
pulumi config set public_key_path ~/.ssh/id_rsa.pub
```

Or edit `Pulumi.<stack>.yaml` directly:

```yaml
config:
  pulumi:region: ru-central1
  zone: ru-central1-a
  vm_name: lab-vm
  vm_user: ubuntu
  my_ip: 0.0.0.0/0
```

### 5. Preview Infrastructure

```bash
pulumi preview
```

### 6. Apply Infrastructure

```bash
pulumi up
```

### 7. Connect to VM

After apply, use the output command:

```bash
pulumi stack output ssh_connection
ssh ubuntu@<vm-ip>
```

### 8. Clean Up

```bash
pulumi destroy
```

## Files

- `__main__.py` - Main Pulumi infrastructure code (Python)
- `requirements.txt` - Python dependencies
- `Pulumi.yaml` - Pulumi project metadata
- `Pulumi.<stack>.yaml` - Stack configuration (gitignored)
- `venv/` - Python virtual environment (gitignored)

## Resources Created

- VPC Network
- VPC Subnet
- Security Group (firewall rules)
- Compute Instance (VM)
- Static Public IP Address

## Comparison with Terraform

### Similarities

- Both use declarative infrastructure definition
- Both support multiple cloud providers
- Both have similar workflow (plan/apply/destroy)
- Both manage infrastructure state

### Differences

- **Language**: Terraform uses HCL, Pulumi uses general-purpose languages
  (Python, TypeScript, Go, etc.)
- **State Management**: Terraform stores state in local files or remote
  backends, Pulumi uses Pulumi Cloud by default
- **Logic**: Terraform has limited HCL-based logic, Pulumi allows full
  programming language features
- **Testing**: Terraform uses external testing tools, Pulumi supports native
  unit testing

## Cost

This configuration uses Yandex Cloud's free tier:

- VM: 20% vCPU, 1 GB RAM (free tier)
- Storage: 10 GB SSD
- Static IP: 1 address (free tier)
