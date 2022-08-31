terraform {
      required_providers {
     ocih= {
         source= "registry.terraform.io/hashicorp/oci"
         version = "4.79.0"
     }
    ocic= {
         source= "registry.terraform.io/oracle/oci"
         version = "4.79.0"
     }
    }
  cloud {
      organization= "cryoelite"
      workspaces {
          name= "Valheim_TF"
      }
  }
}