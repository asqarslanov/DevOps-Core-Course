output "vm_public_ip" {
  description = "Public IP address of the VM"
  value       = yandex_compute_instance.lab_vm.network_interface[0].nat_ip_address
}

output "vm_private_ip" {
  description = "Private IP address of the VM"
  value       = yandex_compute_instance.lab_vm.network_interface[0].ip_address
}

output "vm_name" {
  description = "Name of the VM instance"
  value       = yandex_compute_instance.lab_vm.name
}

output "vm_zone" {
  description = "Zone where VM is deployed"
  value       = yandex_compute_instance.lab_vm.zone
}

output "ssh_connection" {
  description = "SSH connection command"
  value       = "ssh ${var.vm_user}@${yandex_compute_instance.lab_vm.network_interface[0].nat_ip_address}"
}

output "network_id" {
  description = "ID of the VPC network"
  value       = yandex_vpc_network.lab_network.id
}

output "subnet_id" {
  description = "ID of the VPC subnet"
  value       = yandex_vpc_subnet.lab_subnet.id
}
