variable "coffeeshop_name" {
  type = string
}

variable "billing_mode" {
  type        = string
  description = "The billing mode for the table. Possible values are PROVISIONED and PAY_PER_REQUEST."
  default     = "PAY_PER_REQUEST"

  validation {
    condition     = contains(["PROVISIONED", "PAY_PER_REQUEST"], var.billing_mode)
    error_message = "Valid values for `billing_mode` are 'PROVISIONED' and 'PAY_PER_REQUEST'."
  }
}

variable "partition_key" {
  type        = string
  description = "The attribute to use as the partition key."
  # This defaults to the same as the CLI `Config` struct in coffeeshop.
  default = "identifier"
}

variable "attributes" {
  type = map(object({
    type = string
  }))
  description = "Any additional attributes to add to the table. Since only the partition key is declared for you, this should always be empty; otherwise you will get an error about unindexed attributes."
  default     = {}
}
