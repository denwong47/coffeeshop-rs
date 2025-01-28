# A demo IAM role to create a IAM role that is capable of performing the
# necessary actions on DynamoDB and SQS.
#
# Since `coffeeshop` applications are often permanent and long-lived, roles
# are not the best choice for this use case; this solely for the purpose of
# documenting the policies required. In a real-world scenario, you should
# consider using a service user instead.

data "aws_iam_policy_document" "assume_role" {
  statement {
    effect = "Allow"

    dynamic "principals" {
      for_each = var.assume_role_principals
      content {
        type        = principals.value.type
        identifiers = principals.value.identifiers
      }
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "role" {
  name               = provider::corefunc::str_pascal("coffeeshop-${var.coffeeshop_name}-role", true)
  assume_role_policy = data.aws_iam_policy_document.assume_role.json
}

data "aws_iam_policy_document" "policy" {
  statement {
    sid    = "AllowDynamoDB"
    effect = "Allow"
    actions = [
      "dynamodb:BatchGetItem",
      "dynamodb:BatchWriteItem",
      "dynamodb:ConditionCheckItem",
      "dynamodb:TagResource",
      "dynamodb:UntagResource",
      "dynamodb:PutItem",
      "dynamodb:DescribeTable",
      "dynamodb:DeleteItem",
      "dynamodb:GetItem",
      "dynamodb:UpdateItem",
      "dynamodb:DescribeTimeToLive"
    ]
    resources = [
      var.dynamodb.arn
    ]
  }

  statement {
    sid    = "AllowSQS"
    effect = "Allow"
    actions = [
      "sqs:DeleteMessage",
      "sqs:TagQueue",
      "sqs:UntagQueue",
      "sqs:PurgeQueue",
      "sqs:ReceiveMessage",
      "sqs:SendMessage",
      "sqs:ListQueueTags",
      "sqs:ChangeMessageVisibility",
      "sqs:GetQueueAttributes"
    ]
    resources = [
      var.sqs.arn
    ]
  }
}

resource "aws_iam_policy" "policy" {
  name        = provider::corefunc::str_pascal("coffeeshop-${var.coffeeshop_name}-policy", true)
  description = "Policy for the ${var.coffeeshop_name} application."
  policy      = data.aws_iam_policy_document.policy.json
}

resource "aws_iam_role_policy_attachment" "attachment" {
  role       = aws_iam_role.role.name
  policy_arn = aws_iam_policy.policy.arn
}
