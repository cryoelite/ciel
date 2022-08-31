provider "ocih"{
  tenancy_ocid = var.tenancy_OCID
  user_ocid = var.user_OCID
  private_key = var.private_key
  fingerprint = var.fingerprint
  region = var.region
}

provider "ocic"{
  tenancy_ocid = var.tenancy_OCID
  user_ocid = var.user_OCID
  private_key = var.private_key
  fingerprint = var.fingerprint
  region = var.region
}