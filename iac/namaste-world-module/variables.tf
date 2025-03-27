variable "instance_type" {
  type        = string
  description = "The type of instance to launch."
  default     = "t3.micro"
}

variable "subnet_id" {
  type        = string
  description = "The ID of the subnet to launch the instance into."
  validation {
    condition = length(regexall("^sub-net-[\\d|\\w]+$", var.subnet_id))
    error_message = "The subnet_id must match the pattern ^subnet-[\\d|\\w]+$"
  }
}