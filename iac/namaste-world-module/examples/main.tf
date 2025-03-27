data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default" {
  filter {
    name = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

module "test_instance" {
  source = "../"
  subnet_id = data.aws_subnets.default.ids[0]
}

output "aws_instance_arn" {
  value = module.test_instance.aws_instance_arn
}