"""Pulumi infrastructure for Lab 4 - VM Creation with Yandex Cloud"""

import os

import pulumi_yandex as yandex

import pulumi

# Configuration
config = pulumi.Config()
zone = config.get("zone") or "ru-central1-a"
vm_name = config.get("vm_name") or "lab-vm"
vm_user = config.get("vm_user") or "ubuntu"
image_id = config.get("image_id") or "fd87uqpm4aqk1m2f0q8q"

# Get SSH public key
public_key_path = config.get("public_key_path") or os.path.expanduser(
    "~/.ssh/id_rsa.pub",
)
with open(public_key_path, "r") as f:
    public_key = f.read().strip()

# Get my IP for SSH (default to 0.0.0.0/0 for testing)
my_ip = config.get("my_ip") or "0.0.0.0/0"

# Create VPC Network
network = yandex.VpcNetwork("lab-network", name="lab-network")

# Create VPC Subnet
subnet = yandex.VpcSubnet(
    "lab-subnet",
    name="lab-subnet",
    zone=zone,
    network_id=network.id,
    v4_cidr_blocks=["192.168.10.0/24"],
)

# Create Security Group
security_group = yandex.VpcSecurityGroup(
    "lab-sg",
    name="lab-security-group",
    network_id=network.id,
    ingress=[
        # SSH from my IP
        {
            "protocol": "TCP",
            "from_port": 22,
            "to_port": 22,
            "cidr_blocks": [my_ip],
            "description": "Allow SSH from my IP",
        },
        # HTTP from anywhere
        {
            "protocol": "TCP",
            "from_port": 80,
            "to_port": 80,
            "cidr_blocks": ["0.0.0.0/0"],
            "description": "Allow HTTP",
        },
        # Custom port 5000 for app deployment
        {
            "protocol": "TCP",
            "from_port": 5000,
            "to_port": 5000,
            "cidr_blocks": ["0.0.0.0/0"],
            "description": "Allow custom port 5000",
        },
    ],
    egress=[
        # HTTPS outbound
        {
            "protocol": "TCP",
            "from_port": 443,
            "to_port": 443,
            "cidr_blocks": ["0.0.0.0/0"],
            "description": "Allow HTTPS outbound",
        },
        # HTTP outbound
        {
            "protocol": "TCP",
            "from_port": 80,
            "to_port": 80,
            "cidr_blocks": ["0.0.0.0/0"],
            "description": "Allow HTTP outbound",
        },
    ],
)

# Create Static IP Address
static_ip = yandex.ComputeAddress("lab-vm-ip", name="lab-vm-ip", zone=zone)

# Create Compute Instance
vm = yandex.ComputeInstance(
    "lab-vm",
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
            type="network-hdd",
        ),
    ),
    network_interface=[
        yandex.ComputeInstanceNetworkInterfaceArgs(
            subnet_id=subnet.id,
            security_group_ids=[security_group.id],
            nat_ip_address=static_ip.address,
        ),
    ],
    metadata={"ssh-keys": f"{vm_user}:{public_key}"},
)

# Exports
pulumi.export("vm_public_ip", vm.network_interface[0].nat_ip_address)
pulumi.export("vm_private_ip", vm.network_interface[0].ip_address)
pulumi.export("vm_name", vm.name)
pulumi.export("vm_zone", vm.zone)
pulumi.export(
    "ssh_connection",
    f"ssh {vm_user}@{vm.network_interface[0].nat_ip_address}",
)
