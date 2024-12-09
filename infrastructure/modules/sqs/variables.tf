variable "coffeeshop_name" {
  type = string
}

variable "max_message_size" {
  type        = number
  description = "The size limit of each message in bytes. Maximum 256KB."
  default     = 2048
}

variable "message_retention_seconds" {
  type        = number
  description = "The number of seconds that Amazon SQS retains a message."
  default     = 24 * 60 * 60 # 1 day
}

variable "receive_wait_time_seconds" {
  type        = number
  description = "The time for which a ReceiveMessage call will wait for a message to arrive. Maximum 20 seconds."
  default     = 20
}

variable "visibility_timeout_seconds" {
  type        = number
  description = "The duration (in seconds) that the received messages are hidden from subsequent retrieve requests after being retrieved by a ReceiveMessage request. This is related to the expected processing time of each ticket; if the processing time is longer than this value, the message will be processed multiple times."
  default     = 120
}
