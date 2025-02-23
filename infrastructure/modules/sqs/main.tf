locals {
  queue_name = "task-queue-${var.coffeeshop_name}"
}

resource "aws_sqs_queue" "queue" {
  name                       = local.queue_name
  delay_seconds              = 0
  max_message_size           = var.max_message_size
  message_retention_seconds  = var.message_retention_seconds
  receive_wait_time_seconds  = var.receive_wait_time_seconds
  visibility_timeout_seconds = var.visibility_timeout_seconds
}
