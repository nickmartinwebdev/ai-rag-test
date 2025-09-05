# AI RAG Test

A simple Rust application for testing RAG (Retrieval-Augmented Generation) concepts.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Docker (optional)

### Building

```bash
cargo build
```

### Running

```bash
cargo run
```

### Testing

```bash
cargo test
```

## Docker

### Building the Docker image

```bash
docker build -t ai-rag-test .
```

### Running the Docker container

```bash
docker run -p 8080:8080 ai-rag-test
```

## Development

This project uses standard Rust tooling:

- `cargo fmt` - Format code
- `cargo clippy` - Lint code
- `cargo test` - Run tests

## CI/CD

The project includes GitHub Actions workflows for:

- Code formatting checks
- Clippy linting
- Running tests
- Building releases
- Docker image building

## License

This project is licensed under the MIT License.