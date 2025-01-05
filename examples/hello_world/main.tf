# Calling the modules from `infrastructure` directory to create the resources.

module "dynamodb" {
  source          = "../../infrastructure/modules/dynamodb"
  coffeeshop_name = var.coffeeshop_name
  billing_mode    = "PAY_PER_REQUEST"
  partition_key   = "identifier"
}

module "sqs" {
  source          = "../../infrastructure/modules/sqs"
  coffeeshop_name = var.coffeeshop_name
}

module "iam_role" {
  source          = "../../infrastructure/modules/iam_role"
  coffeeshop_name = var.coffeeshop_name
  assume_role_principals = [
    {
      type        = "Service"
      identifiers = ["ec2.amazonaws.com"]
    },
    {
      type        = "AWS"
      identifiers = []
    }
  ]
  dynamodb = module.dynamodb.table
  sqs      = module.sqs.queue
}

output "resources" {
  value = {
    dynamodb = module.dynamodb.table,
    iam_role = module.iam_role.role,
    sqs      = module.sqs.queue
  }
}
