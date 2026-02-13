variable "zone" {
  description = "Yandex Cloud zone"
  type        = string
  default     = "ru-central1-a"
}

variable "network_name" {
  description = "Name for VPC network"
  type        = string
  default     = "lab-network"
}

variable "subnet_name" {
  description = "Name for VPC subnet"
  type        = string
  default     = "lab-subnet"
}

variable "subnet_cidr" {
  description = "CIDR block for subnet"
  type        = list(string)
  default     = ["192.168.10.0/24"]
}

variable "security_group_name" {
  description = "Name for security group"
  type        = string
  default     = "lab-security-group"
}

variable "vm_name" {
  description = "Name for the VM instance"
  type        = string
  default     = "lab-vm"
}

variable "vm_user" {
  description = "Username for VM access"
  type        = string
  default     = "ubuntu"
}

variable "image_id" {
  description = "ID of the boot image (Ubuntu 22.04 LTS)"
  type        = string
  default     = "fd87uqpm4aqk1m2f0q8q"
}

variable "public_key_path" {
  description = "Path to public SSH key"
  type        = string
  default     = "~/.ssh/id_rsa.pub"
}

variable "my_ip" {
  description = "Your IP address for SSH access (use 0.0.0.0/0 for testing)"
  type        = string
  default     = "0.0.0.0/0"
}
