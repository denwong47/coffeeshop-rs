locals {
  combined_attributes = merge(
    # Automatically defines the partition key for you.
    tomap({
      (var.partition_key) = {
        type = "S"
      }
    }),
    var.attributes
  )
}

resource "aws_dynamodb_table" "table" {
  name         = "task-queue-${var.coffeeshop_name}"
  billing_mode = var.billing_mode
  hash_key     = var.partition_key

  # Declare all the attributes in the table.
  dynamic "attribute" {
    for_each = local.combined_attributes
    content {
      name = attribute.key
      type = attribute.value.type
    }
  }

  # This attribute name is hard coded in coffeeshop; do not change it.
  ttl {
    attribute_name = "ttl"
    enabled        = true
  }
}
