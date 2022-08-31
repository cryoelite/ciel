terraform {
      required_providers {
     oci= {
         source= "oracle/oci"
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