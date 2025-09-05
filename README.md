# AI RAG Test

A simple Rust application for testing RAG (Retrieval-Augmented Generation) concepts with Meilisearch integration.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Docker and Docker Compose
- Meilisearch (included in Docker Compose)

### Quick Start with Docker Compose

```bash
# Start Meilisearch and the application
docker-compose up -d

# View logs
docker-compose logs -f app
```

### Local Development

```bash
# Start only Meilisearch
docker-compose up -d meilisearch

# Build and run the application locally
cargo build
cargo run
```

### Testing

```bash
cargo test
```

## Meilisearch Integration

This application demonstrates basic Meilisearch functionality:

- Connecting to a Meilisearch instance
- Creating and managing indexes
- Indexing sample documents
- Performing search queries

### Environment Variables

Copy `.env.example` to `.env` and adjust as needed:

```bash
cp .env.example .env
```

### Meilisearch Dashboard

When running with Docker Compose, the Meilisearch dashboard is available at:
http://localhost:7700

Default master key: `your-master-key-change-this-in-production`

## Docker

### Building the Docker image

```bash
docker build -t ai-rag-test .
```

### Running with Docker Compose

```bash
# Start all services
docker-compose up -d

# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

## Development

This project uses standard Rust tooling:

- `cargo fmt` - Format code
- `cargo clippy` - Lint code
- `cargo test` - Run tests

### Dependencies

- `meilisearch-sdk` - Official Meilisearch Rust client
- `tokio` - Async runtime
- `serde` - Serialization framework
- `serde_json` - JSON support

## CI/CD

The project includes GitHub Actions workflows for:

- Code formatting checks
- Clippy linting
- Running tests
- Building releases
- Docker image building

## License

This project is licensed under the MIT License.