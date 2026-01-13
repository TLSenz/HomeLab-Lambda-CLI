terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region = var.aws_region
}

# DynamoDB table for storing server configurations
resource "aws_dynamodb_table" "homelab_servers" {
  name         = "homelab-servers"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "server_id"

  attribute {
    name = "server_id"
    type = "S"
  }

  point_in_time_recovery {
    enabled = true
  }

  tags = {
    Name        = "Homelab Servers Table"
    Project     = "homelab-manager"
    Environment = var.environment
  }
}

# IAM role for Lambda function
resource "aws_iam_role" "lambda_role" {
  name = "${var.project_name}-lambda-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })

  tags = {
    Name        = "${var.project_name}-lambda-role"
    Project     = var.project_name
    Environment = var.environment
  }
}

# IAM policy for Lambda to access DynamoDB and CloudWatch
resource "aws_iam_policy" "lambda_policy" {
  name        = "${var.project_name}-lambda-policy"
  description = "Policy for Homelab Manager Lambda function"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Resource = "arn:aws:logs:*:*:*"
      },
      {
        Effect = "Allow"
        Action = [
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:UpdateItem",
          "dynamodb:DeleteItem",
          "dynamodb:Scan",
          "dynamodb:Query"
        ]
        Resource = [
          aws_dynamodb_table.homelab_servers.arn,
          "${aws_dynamodb_table.homelab_servers.arn}/*"
        ]
      }
    ]
  })

  tags = {
    Name        = "${var.project_name}-lambda-policy"
    Project     = var.project_name
    Environment = var.environment
  }
}

# Attach policy to role
resource "aws_iam_role_policy_attachment" "lambda_policy_attachment" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_policy.arn
}

# CloudWatch Log Group for Lambda
resource "aws_cloudwatch_log_group" "lambda_log_group" {
  name              = "/aws/lambda/${var.project_name}-function"
  retention_in_days = 14

  tags = {
    Name        = "${var.project_name}-lambda-logs"
    Project     = var.project_name
    Environment = var.environment
  }
}

# API Gateway REST API
resource "aws_api_gateway_rest_api" "homelab_api" {
  name        = "${var.project_name}-api"
  description = "API Gateway for Homelab Manager"

  endpoint_configuration {
    types = ["REGIONAL"]
  }

  tags = {
    Name        = "${var.project_name}-api"
    Project     = var.project_name
    Environment = var.environment
  }
}

# API Gateway resource for /servers
resource "aws_api_gateway_resource" "servers" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  parent_id   = aws_api_gateway_rest_api.homelab_api.root_resource_id
  path_part   = "servers"
}

# API Gateway resource for /servers/{id}
resource "aws_api_gateway_resource" "servers_id" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  parent_id   = aws_api_gateway_resource.servers.id
  path_part   = "{id}"
}

# API Gateway method for POST /servers (create)
resource "aws_api_gateway_method" "servers_post" {
  rest_api_id   = aws_api_gateway_rest_api.homelab_api.id
  resource_id   = aws_api_gateway_resource.servers.id
  http_method   = "POST"
  authorization = "NONE"
}

# API Gateway method for GET /servers (list)
resource "aws_api_gateway_method" "servers_get" {
  rest_api_id   = aws_api_gateway_rest_api.homelab_api.id
  resource_id   = aws_api_gateway_resource.servers.id
  http_method   = "GET"
  authorization = "NONE"
}

# API Gateway method for PUT /servers/{id} (update)
resource "aws_api_gateway_method" "servers_id_put" {
  rest_api_id   = aws_api_gateway_rest_api.homelab_api.id
  resource_id   = aws_api_gateway_resource.servers_id.id
  http_method   = "PUT"
  authorization = "NONE"
}

# API Gateway method for DELETE /servers/{id} (delete)
resource "aws_api_gateway_method" "servers_id_delete" {
  rest_api_id   = aws_api_gateway_rest_api.homelab_api.id
  resource_id   = aws_api_gateway_resource.servers_id.id
  http_method   = "DELETE"
  authorization = "NONE"
}

# API Gateway integration for Lambda
resource "aws_api_gateway_integration" "lambda_integration" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  resource_id = aws_api_gateway_resource.servers.id
  http_method = aws_api_gateway_method.servers_post.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.homelab_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "lambda_integration_get" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  resource_id = aws_api_gateway_resource.servers.id
  http_method = aws_api_gateway_method.servers_get.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.homelab_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "lambda_integration_put" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  resource_id = aws_api_gateway_resource.servers_id.id
  http_method = aws_api_gateway_method.servers_id_put.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.homelab_lambda.invoke_arn
}

resource "aws_api_gateway_integration" "lambda_integration_delete" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id
  resource_id = aws_api_gateway_resource.servers_id.id
  http_method = aws_api_gateway_method.servers_id_delete.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.homelab_lambda.invoke_arn
}

# API Gateway deployment
resource "aws_api_gateway_deployment" "api_deployment" {
  rest_api_id = aws_api_gateway_rest_api.homelab_api.id

  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.servers.id,
      aws_api_gateway_resource.servers_id.id,
      aws_api_gateway_method.servers_post.id,
      aws_api_gateway_method.servers_get.id,
      aws_api_gateway_method.servers_id_put.id,
      aws_api_gateway_method.servers_id_delete.id,
      aws_api_gateway_integration.lambda_integration.id,
      aws_api_gateway_integration.lambda_integration_get.id,
      aws_api_gateway_integration.lambda_integration_put.id,
      aws_api_gateway_integration.lambda_integration_delete.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }

  depends_on = [
    aws_api_gateway_method.servers_post,
    aws_api_gateway_method.servers_get,
    aws_api_gateway_method.servers_id_put,
    aws_api_gateway_method.servers_id_delete,
    aws_api_gateway_integration.lambda_integration,
    aws_api_gateway_integration.lambda_integration_get,
    aws_api_gateway_integration.lambda_integration_put,
    aws_api_gateway_integration.lambda_integration_delete,
  ]
}

# API Gateway stage
resource "aws_api_gateway_stage" "api_stage" {
  deployment_id = aws_api_gateway_deployment.api_deployment.id
  rest_api_id   = aws_api_gateway_rest_api.homelab_api.id
  stage_name    = var.environment

  tags = {
    Name        = "${var.project_name}-api-${var.environment}"
    Project     = var.project_name
    Environment = var.environment
  }
}

# Lambda permission for API Gateway
resource "aws_lambda_permission" "api_gateway_permission" {
  statement_id  = "AllowExecutionFromAPIGateway"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.homelab_lambda.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.homelab_api.execution_arn}/*/*/*"
}

# Build Lambda function zip file
data "archive_file" "lambda_zip" {
  type        = "zip"
  source_dir  = "${path.root}/../lambda/target/lambda/bootstrap/release/"
  output_path = "${path.root}/lambda_function.zip"

  depends_on = [null_resource.build_lambda]
}

# Build the Lambda function
resource "null_resource" "build_lambda" {
  triggers = {
    source_hash = sha256(join("", [for f in fileset("${path.root}/../lambda", "**/*.rs") : filesha256("${path.root}/../lambda/${f}")]))
  }

  provisioner "local-exec" {
    command = "cd ../lambda && cargo build --release --target lambda"
  }
}

# Lambda function
resource "aws_lambda_function" "homelab_lambda" {
  filename      = data.archive_file.lambda_zip.output_path
  function_name = "${var.project_name}-function"
  role          = aws_iam_role.lambda_role.arn
  handler       = "bootstrap"
  runtime       = "provided.al2023"
  timeout       = 30

  environment {
    variables = {
      TABLE_NAME = aws_dynamodb_table.homelab_servers.name
      RUST_LOG   = "info"
    }
  }

  tags = {
    Name        = "${var.project_name}-lambda"
    Project     = var.project_name
    Environment = var.environment
  }

  depends_on = [
    aws_cloudwatch_log_group.lambda_log_group,
    null_resource.build_lambda
  ]
}