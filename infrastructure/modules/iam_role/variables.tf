variable "coffeeshop_name" {
  type = string
}

variable "assume_role_principals" {
  type = list(object({
    type        = string,
    identifiers = list(string)
  }))
  description = "The principals to allow to assume the role."
  default = [{
    type        = "Service",
    identifiers = ["ec2.amazonaws.com"]
  }]
}

variable "dynamodb" {
  type = object({
    name          = string,
    arn           = string,
    partition_key = string,
    attributes = map(object({
      type = string
    }))
  })
  description = "The DynamoDB table to allow access to; this is exported by the `dynamodb` module."
}

variable "sqs" {
  type = object({
    name = string,
    arn  = string,
    url  = string
  })
  description = "The SQS queue to allow access to; this is exported by the `sqs` module."
}
