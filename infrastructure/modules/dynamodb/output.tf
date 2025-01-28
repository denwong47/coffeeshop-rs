output "table" {
  value = {
    name          = aws_dynamodb_table.table.name
    arn           = aws_dynamodb_table.table.arn
    partition_key = aws_dynamodb_table.table.hash_key
    # Export the attributes of the table.
    attributes = {
      for attribute in aws_dynamodb_table.table.attribute :
      attribute.name => {
        type = attribute.type
      }
    }
  }
}
