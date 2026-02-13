terraform {
  required_providers {
    yandex = {
      source  = "yandex-cloud/yandex"
      version = "~> 0.100.0"
    }
  }
  required_version = ">= 1.0.0"
}

provider "yandex" {
  zone = var.zone
}

resource "yandex_vpc_network" "lab_network" {
  name = var.network_name
}

resource "yandex_vpc_subnet" "lab_subnet" {
  name           = var.subnet_name
  zone           = var.zone
  network_id     = yandex_vpc_network.lab_network.id
  v4_cidr_blocks = var.subnet_cidr
}

resource "yandex_vpc_security_group" "lab_sg" {
  name       = var.security_group_name
  network_id = yandex_vpc_network.lab_network.id

  ingress {
    protocol    = "TCP"
    from_port   = 22
    to_port     = 22
    cidr_blocks = [var.my_ip]
    description = "Allow SSH from my IP"
  }

  ingress {
    protocol    = "TCP"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow HTTP"
  }

  ingress {
    protocol    = "TCP"
    from_port   = 5000
    to_port     = 5000
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow custom port 5000 for app deployment"
  }

  egress {
    protocol    = "TCP"
    from_port   = 443
    to_port     = 443
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow HTTPS outbound"
  }

  egress {
    protocol    = "TCP"
    from_port   = 80
    to_port     = 80
    cidr_blocks = ["0.0.0.0/0"]
    description = "Allow HTTP outbound"
  }
}

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
      type     = "network-hdd"
    }
  }

  network_interface {
    subnet_id          = yandex_vpc_subnet.lab_subnet.id
    security_group_ids = [yandex_vpc_security_group.lab_sg.id]
    nat                = true
  }

  metadata = {
    ssh-keys = "${var.vm_user}:${file(var.public_key_path)}"
  }
}

resource "yandex_compute_address" "lab_ip" {
  name = "lab-vm-ip"
  zone = var.zone
}
