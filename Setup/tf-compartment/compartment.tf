resource "oci_identity_compartment" "tf-compartment" {
    # Required
    compartment_id = var.tenancy_OCID
    description = "Compartment for ValheimTF resources."
    name = "Gen2_Compartment"
}