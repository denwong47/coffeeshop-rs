## Sample Terraform/OpenTofu files

This folder contains example Terraform configurations for use with either
OpenTofu or Terraform, in order to deploy

- an SQS queuea,
- a DynamoDB table, and
- a IAM role with the necessary policies for the above.

These are meant to be a documentation of the minimum resources required by the
`coffeeshop` framework, and should only serve as a starting point or your
custom configuration.

For simple deployments, it is possible to simply call modules directly from this
folder just like the `examples` folder do.
