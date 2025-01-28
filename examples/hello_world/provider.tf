# Example of a provider configuration file.
#
# This file declares the required providers and their versions, and is meant to
# be a template for your own provider configuration file. You can copy this file
# to your own project and modify it as needed.
#
# This assumes an S3 backend for state storage. If you are using a different
# backend, you will need to modify the backend configuration block accordingly.

terraform {
  required_version = ">=1.0.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
  backend "s3" {}
}

# Configure the AWS Provider
provider "aws" {
  region  = var.aws_region
  profile = var.aws_profile
}
