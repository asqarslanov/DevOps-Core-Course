# Terraform Infrastructure for Lab 4

This directory contains Terraform configuration to provision infrastructure in
Yandex Cloud.

## Prerequisites

1. **Terraform CLI** (version 1.0.0 or higher)
2. **Yandex Cloud account**
3. **Service account with necessary permissions**
4. **SSH key pair** (for VM access)

## Yandex Cloud Setup

### 1. Create Service Account

1. Go to Yandex Cloud Console → IAM → Service Accounts
2. Create a new service account with the following roles:
   - `compute.admin` (for creating/managing VMs)
   - `vpc.admin` (for creating/managing networks)
3. Generate an authorized key for the service account (JSON format)
4. Save the JSON key file securely

### 2. Configure Authentication

Set up authentication using one of these methods:

**Method 1: Environment Variable (Recommended)**

```bash
export YC_TOKEN=$(yc iam create-token)
export YC_CLOUD_ID="your_cloud_id"
export YC_FOLDER_ID="your_folder_id"
```

**Method 2: Service Account Key File**

```bash
export YC_KEY_FILE="/path/to/service-account-key.json"
```

### 3. Generate SSH Key Pair

```bash
ssh-keygen -t rsa -b 2048 -f ~/.ssh/id_rsa -N ""
```

## Usage

### 1. Initialize Terraform

```bash
cd terraform
terraform init
```

### 2. Configure Variables

Copy the example variables file and update it:

```bash
cp terraform.tfvars.example terraform.tfvars
# Edit terraform.tfvars with your values
```

### 3. Plan Infrastructure

```bash
terraform plan
```

### 4. Apply Infrastructure

```bash
terraform apply
```

### 5. Connect to VM

After apply, use the output command to connect:

```bash
terraform output ssh_connection
ssh ubuntu@<vm-ip>
```

### 6. Clean Up

```bash
terraform destroy
```

## Files

- `main.tf` - Main Terraform configuration (providers and resources)
- `variables.tf` - Input variable definitions
- `outputs.tf` - Output value definitions
- `terraform.tfvars` - Variable values (gitignored)
- `terraform.tfvars.example` - Template for terraform.tfvars

## Resources Created

- VPC Network
- VPC Subnet
- Security Group (firewall rules)
- Compute Instance (VM)
- Static Public IP Address

## Cost

This configuration uses Yandex Cloud's free tier:

- VM: 20% vCPU, 1 GB RAM (free tier)
- Storage: 10 GB SSD
- Static IP: 1 address (free tier)
