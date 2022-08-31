module "access_compartment"{
    source= "github.com/cryoelite/Valheim_TF/tf-compartment"
    private_key = var.private_key
    tenancy_OCID = var.tenancy_OCID
    user_OCID = var.user_OCID
    fingerprint = var.fingerprint
    region = var.region
} 