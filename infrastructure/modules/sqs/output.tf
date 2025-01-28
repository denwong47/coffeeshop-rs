output "queue" {
  value = {
    name = aws_sqs_queue.queue.name
    arn  = aws_sqs_queue.queue.arn
    url  = aws_sqs_queue.queue.url
  }
}
