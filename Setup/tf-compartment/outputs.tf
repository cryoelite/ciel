# Outputs for compartment

output "compartment-name" {
  value = oci_identity_compartment.tf-compartment.name
}

output "compartment-OCID" {
  value = oci_identity_compartment.tf-compartment.id
}

/* output "yolo" {
  value= module.access_provider.all-availability-domains-in-your-tenancy

} */

#compartment-OCID = "ocid1.compartment.oc1..aaaaaaaautgh27hii42swqfzui2mixlkuj7hdqhbourcgrbpcaeysq5sra2q"
#compartment-name = "Gen1_Compartment"