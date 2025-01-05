# Required for the `str_pascal` function;
# see https://registry.terraform.io/providers/northwood-labs/corefunc/latest/docs/functions/str_pascal
# for more information.

terraform {
  required_version = ">=1.0.0"
  required_providers {
    corefunc = {
      source  = "northwood-labs/corefunc"
      version = "~> 1.0"
    }
  }
}
