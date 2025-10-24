# S3FS - S3-Compatible Filesystem Server

A lightweight S3-compatible server that uses the local filesystem as storage backend.

## Features

- S3-compatible API endpoints
- Local filesystem storage
- Configurable authentication
- Multiple configuration methods (CLI, environment variables, config file)
- Debug logging support

## Installation

```bash
cargo build --release
```

## Configuration

S3FS supports multiple ways to configure credentials and server settings:

### 1. Command Line Arguments

```bash
./target/release/s3fs \
  --host 0.0.0.0 \
  --port 9000 \
  --root ./my-s3-data \
  --access-key my-access-key \
  --secret-key my-secret-key
```

### 2. Environment Variables

```bash
export S3FS_ACCESS_KEY=my-access-key
export S3FS_SECRET_KEY=my-secret-key
./target/release/s3fs --host 0.0.0.0 --port 9000
```

### 3. Configuration File

Create a JSON configuration file:

```json
{
  "host": "0.0.0.0",
  "port": 9000,
  "root": "./s3-data",
  "access_key": "my-access-key",
  "secret_key": "my-secret-key"
}
```

Then run with:

```bash
./target/release/s3fs --config config.json
```

## Command Line Options

- `-c, --config <FILE>`: Configuration file path
- `-H, --host <HOST>`: Host to bind to (default: 127.0.0.1)
- `-p, --port <PORT>`: Port to bind to (default: 9001)
- `-r, --root <DIR>`: Root directory for file storage (default: ./s3-data)
- `-a, --access-key <KEY>`: S3 Access Key ID (default: admin)
- `-s, --secret-key <KEY>`: S3 Secret Access Key (default: admin)

## Environment Variables

- `S3FS_ACCESS_KEY`: S3 Access Key ID
- `S3FS_SECRET_KEY`: S3 Secret Access Key

## Usage Examples

### Basic Usage

```bash
# Start server with default settings
cargo run

# Start server on all interfaces
cargo run -- --host 0.0.0.0 --port 9000
```

### With Custom Credentials

```bash
# Using command line
cargo run -- --access-key mykey --secret-key mysecret

# Using environment variables
export S3FS_ACCESS_KEY=mykey
export S3FS_SECRET_KEY=mysecret
cargo run
```

### S3 Client Configuration

Configure your S3 client with:

- **Endpoint**: `http://127.0.0.1:9001` (or your configured host:port)
- **Access Key**: Your configured access key
- **Secret Key**: Your configured secret key
- **Region**: `us-east-1` (or any region)
- **Path Style**: `true` (force path style for compatibility)

### Example with AWS CLI

```bash
aws configure set aws_access_key_id mykey
aws configure set aws_secret_access_key mysecret
aws configure set region us-east-1

# List buckets
aws s3 ls --endpoint-url http://127.0.0.1:9001

# Create bucket
aws s3 mb s3://test-bucket --endpoint-url http://127.0.0.1:9001

# Upload file
aws s3 cp file.txt s3://test-bucket/ --endpoint-url http://127.0.0.1:9001
```

## Development

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Build for Production

```bash
cargo build --release
```

## Security Notes

- This server is intended for development and testing purposes
- The authentication is basic and credentials are logged in plain text
- For production use, consider additional security measures
- Store sensitive credentials in environment variables or secure configuration files
- Ensure proper file system permissions on the storage directory

## License

[Add your license information here]
