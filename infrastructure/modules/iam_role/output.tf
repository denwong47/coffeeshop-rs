output "role" {
  value = {
    role = {
      name = aws_iam_role.role.name
      arn  = aws_iam_role.role.arn
    },
    policy = {
      name = aws_iam_policy.policy.name
      arn  = aws_iam_policy.policy.arn
    },
    attachment = {
      role       = aws_iam_role_policy_attachment.attachment.role
      policy_arn = aws_iam_role_policy_attachment.attachment.policy_arn
    }
  }
}
