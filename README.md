# Homelab Manager

A serverless AWS Lambda-based system for managing NixOS configuration files across your homelab infrastructure. This project provides a REST API and CLI tool to track which server uses which NixOS configuration file.

## Architecture

- **Lambda Function**: Rust-based API with DynamoDB backend
- **CLI Tool**: Rust CLI for easy management via command line
- **Infrastructure**: Terraform for automated deployment
- **Database**: DynamoDB for storing server configurations
- **API**: API Gateway for HTTP endpoints

## Project Structure

```
aws-lambda/
├── lambda/                 # Rust Lambda function
│   ├── src/
│   │   ├── handlers/       # API endpoint handlers
│   │   ├── models/         # Data models
│   │   └── main.rs        # Lambda entry point
│   └── Cargo.toml
├── cli/                   # Rust CLI tool
│   ├── src/
│   │   ├── commands/       # CLI commands
│   │   ├── config/         # Configuration handling
│   │   └── main.rs        # CLI entry point
│   └── Cargo.toml
├── terraform/             # Infrastructure as Code
│   ├── main.tf           # Main resources
│   ├── variables.tf      # Input variables
│   └── outputs.tf       # Output values
└── README.md
```

## Quick Start

### Prerequisites

- AWS CLI configured with appropriate permissions
- Rust and Cargo installed
- Terraform installed
- Docker (for Lambda builds)

### 1. Clone and Setup

```bash
git clone <repository-url>
cd aws-lambda
```

### 2. Deploy Infrastructure

```bash
cd terraform
terraform init
terraform plan
terraform apply
```

After deployment, note the API Gateway URL from the outputs.

### 3. Build and Deploy Lambda

```bash
cd ../lambda
cargo install cargo-lambda
cargo lambda build --release --target lambda

# The Terraform deployment will automatically build and deploy the Lambda
cd ../terraform
terraform apply  # This will rebuild and update the Lambda
```

### 4. Install CLI Tool

```bash
cd ../cli
cargo install --path .
```

## Usage

### Configuration

Create a configuration file at `~/.config/homelab/config.yaml` or `./homelab.yaml`:

```yaml
api_url: "https://your-api-gateway-url.execute-api.us-east-1.amazonaws.com/dev"
timeout_seconds: 30
region: "us-east-1"
```

Or use the example configuration:

```bash
cp homelab.yaml.example homelab.yaml
# Edit with your API URL
```

### CLI Commands

#### Add a Server

```bash
homelab add --server "web-server" --config-path "/etc/nixos/web-server.nix" --description "Main web server"
```

#### List All Servers

```bash
homelab list
```

#### Update a Server

```bash
homelab update --id "server-id-123" --config-path "/etc/nixos/new-config.nix" --description "Updated description"
```

#### Delete a Server

```bash
homelab delete --id "server-id-123"
```

#### Using Custom API URL

```bash
homelab --api-url "https://custom-url.com" list
```

### API Endpoints

Once deployed, the API provides these endpoints:

- `POST /servers` - Add a new server configuration
- `GET /servers` - List all server configurations  
- `PUT /servers/{id}` - Update a server configuration
- `DELETE /servers/{id}` - Delete a server configuration

#### Example API Usage

```bash
# Add a server
curl -X POST https://your-api-url/servers \
  -H "Content-Type: application/json" \
  -d '{
    "server_name": "web-server",
    "config_file_path": "/etc/nixos/web-server.nix",
    "description": "Main web server"
  }'

# List servers
curl https://your-api-url/servers

# Update a server
curl -X PUT https://your-api-url/servers/server-id-123 \
  -H "Content-Type: application/json" \
  -d '{
    "config_file_path": "/etc/nixos/updated-config.nix"
  }'

# Delete a server
curl -X DELETE https://your-api-url/servers/server-id-123
```

## Data Model

### Server Configuration

Each server configuration stored in DynamoDB contains:

- `server_id` (String): Unique identifier
- `server_name` (String): Human-readable name
- `config_file_path` (String): Path to NixOS configuration file
- `description` (String, optional): Server description
- `created_at` (String): ISO 8601 timestamp
- `updated_at` (String): ISO 8601 timestamp

## Development

### Lambda Development

```bash
cd lambda
cargo check
cargo test
cargo lambda build --release --target lambda
```

### CLI Development

```bash
cd cli
cargo check
cargo test
cargo run -- --help
```

### Infrastructure

```bash
cd terraform
terraform validate
terraform plan
```

## AWS Resources Created

- DynamoDB table: `homelab-servers`
- Lambda function: `homelab-manager-function`
- API Gateway REST API with CORS enabled
- IAM Role and Policies for Lambda execution
- CloudWatch Log Group for logging

## Security Considerations

- Lambda uses least-privilege IAM policies
- API Gateway currently allows public access (can be restricted)
- All requests are logged to CloudWatch
- Input validation on all endpoints
- CORS is configured for web interface integration

## Monitoring and Logs

View Lambda logs:

```bash
aws logs tail /aws/lambda/homelab-manager-function --follow
```

View API Gateway logs through CloudWatch metrics.

## Cost Optimization

- DynamoDB uses pay-per-request billing
- Lambda has 30-second timeout
- CloudWatch logs with 14-day retention
- API Gateway with regional endpoints

## Troubleshooting

### Common Issues

1. **Lambda build fails**: Install cargo-lambda: `cargo install cargo-lambda`
2. **API returns 404**: Check API Gateway deployment and resource paths
3. **CLI can't connect**: Verify API URL in configuration
4. **DynamoDB permissions**: Ensure Lambda IAM role includes DynamoDB access

### Logs

- Lambda logs: `/aws/lambda/homelab-manager-function`
- API Gateway logs: Available through CloudWatch
- Terraform logs: Check terraform console output

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.

## Support

For issues and questions, please open an issue on the repository.