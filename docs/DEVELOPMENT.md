# Pristine Development Guide

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (1.75 or later) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Cargo** (comes with Rust)
- **Node.js** (18 or later) - [Install Node.js](https://nodejs.org/)
- **npm** or **yarn** (comes with Node.js)
- **Git** - [Install Git](https://git-scm.com/)

### Optional Tools

- **Docker** - For containerized development
- **just** - Command runner (`cargo install just`)
- **cargo-watch** - For auto-rebuilding (`cargo install cargo-watch`)

## Repository Setup

### Clone the Repository

```bash
git clone https://github.com/aymenbelarbi/pristine.git
cd pristine
```

### Install Dependencies

```bash
# Install Rust dependencies
cargo fetch

# Install frontend dependencies
cd web
npm install
cd ..
```

## Project Structure

```
pristine/
├── Cargo.toml              # Workspace configuration
├── Cargo.lock              # Dependency lock file
├── rust-toolchain.toml     # Rust toolchain version
├── rustfmt.toml            # Rust formatting configuration
├── clippy.toml             # Clippy linting configuration
├── Dockerfile              # Docker build configuration
├── docker-compose.yml      # Docker Compose configuration
├── LICENSE                 # License file
├── README.md               # Project documentation
├── docs/                   # Additional documentation
│   ├── ARCHITECTURE.md
│   ├── API.md
│   ├── CLI.md
│   ├── CONFIGURATION.md
│   ├── DEVELOPMENT.md
│   └── DEPLOYMENT.md
├── crates/                 # Rust crates
│   ├── pristine-core/      # Core types and traits
│   ├── pristine-domain/    # Domain logic
│   ├── pristine-acquire/   # Repository acquisition
│   ├── pristine-inventory/ # File system traversal
│   ├── pristine-select/    # File selection engine
│   ├── pristine-compress/  # Content compression
│   ├── pristine-extract/   # Content extraction
│   ├── pristine-cache/     # Caching layer
│   ├── pristine-safety/    # Secret scanning
│   ├── pristine-mcp/       # MCP server
│   ├── pristine-observability/ # Metrics/logging
│   ├── pristine-cli/       # CLI
│   └── pristine-api/       # HTTP API
├── web/                    # Frontend application
│   ├── src/
│   │   ├── api/            # API client
│   │   ├── components/     # React components
│   │   ├── hooks/          # Custom hooks
│   │   ├── lib/            # Utility functions
│   │   ├── stores/         # State management
│   │   ├── styles/         # CSS styles
│   │   └── types/          # TypeScript types
│   ├── public/             # Static assets
│   └── package.json        # Node.js dependencies
└── .github/                # GitHub Actions
    └── workflows/          # CI/CD workflows
```

## Building from Source

### Build All Crates

```bash
# Debug build
cargo build --workspace

# Release build (optimized)
cargo build --workspace --release
```

### Build Specific Crate

```bash
# Build only the CLI
cargo build --bin pristine

# Build only the API server
cargo build --bin pristine-api
```

### Build Frontend

```bash
cd web

# Development build
npm run build

# Production build
npm run build -- --mode production
```

## Running the Application

### Run CLI in Development

```bash
# Run directly with cargo
cargo run --bin pristine -- overview ./my-project

# Run with verbose output
cargo run --bin pristine -- --verbose overview ./my-project
```

### Run API Server

```bash
# Start the API server
cargo run --bin pristine-api

# Start with custom port
cargo run --bin pristine-api -- --port 3000

# Start with config file
cargo run --bin pristine-api -- --config pristine.yaml
```

### Run Frontend Development Server

```bash
cd web

# Start development server
npm run dev

# Start with custom port
npm run dev -- --port 3001
```

### Run with Docker Compose

```bash
# Start all services
docker compose up

# Start in background
docker compose up -d

# View logs
docker compose logs -f

# Stop services
docker compose down
```

## Testing

### Run All Tests

```bash
# Run all workspace tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture
```

### Run Tests for Specific Crate

```bash
# Test core crate
cargo test -p pristine-core

# Test domain crate
cargo test -p pristine-domain
```

### Run Integration Tests

```bash
# Run integration tests
cargo test --test '*'

# Run specific integration test
cargo test --test integration_test_name
```

### Run Frontend Tests

```bash
cd web

# Run unit tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch
```

### Run Benchmarks

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench benchmark_name
```

## Code Style Guidelines

### Rust Code Style

Prustine follows the official Rust style guidelines with some additional rules.

#### Formatting

```bash
# Format all code
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

#### Linting

```bash
# Run clippy
cargo clippy --workspace

# Run clippy with all targets
cargo clippy --workspace --all-targets

# Run clippy with deny warnings
cargo clippy --workspace -- -D warnings
```

#### Naming Conventions

- **Types and traits:** `PascalCase` (e.g., `ArtifactRequest`, `FileRanker`)
- **Functions and methods:** `snake_case` (e.g., `process_request`, `get_file`)
- **Constants:** `SCREAMING_SNAKE_CASE` (e.g., `MAX_TOKEN_COUNT`)
- **Modules:** `snake_case` (e.g., `file_scanner`)
- **Variables:** `snake_case` (e.g., `file_path`, `token_count`)

#### Documentation

- All public items must have documentation comments (`///`)
- Include examples in documentation where appropriate
- Use `//!` for module-level documentation

```rust
/// Processes an artifact request and returns a context artifact.
///
/// # Arguments
///
/// * `request` - The artifact request containing source, profile, and options
///
/// # Returns
///
/// A `Result` containing the context artifact or an error
///
/// # Examples
///
/// ```
/// use pristine_core::*;
///
/// let request = ArtifactRequest { /* ... */ };
/// let artifact = process_request(request).await?;
/// ```
pub async fn process_request(request: ArtifactRequest) -> Result<ContextArtifact> {
    // ...
}
```

#### Error Handling

- Use `thiserror` for library error types
- Use `anyhow` for application-level errors
- Provide meaningful error messages
- Include context in errors

```rust
#[derive(Error, Debug)]
pub enum PristineError {
    #[error("Source not found: {0}")]
    SourceNotFound(String),
    
    #[error("Git error: {0}")]
    GitError(String),
}
```

### TypeScript/React Code Style

#### Formatting

Prettier is used for formatting. Configuration is in `web/.prettierrc`.

```bash
cd web

# Format all files
npm run format

# Check formatting
npm run format:check
```

#### Linting

ESLint is used for linting. Configuration is in `web/.eslintrc`.

```bash
cd web

# Run linter
npm run lint

# Fix lint issues
npm run lint:fix
```

#### Component Guidelines

- Use functional components with hooks
- Use TypeScript for all components
- Define prop types using interfaces
- Use named exports

```typescript
interface ArtifactCardProps {
  artifact: Artifact;
  onSelect?: (id: string) => void;
}

export function ArtifactCard({ artifact, onSelect }: ArtifactCardProps) {
  // ...
}
```

#### File Naming

- Components: `PascalCase.tsx` (e.g., `ArtifactCard.tsx`)
- Hooks: `useCamelCase.ts` (e.g., `useIngest.ts`)
- Utilities: `camelCase.ts` (e.g., `formatters.ts`)
- Types: `camelCase.ts` (e.g., `index.ts`)

## Debugging

### Enable Debug Logging

```bash
# Set log level via environment
RUST_LOG=debug cargo run --bin pristine -- overview ./my-project

# Set log level for specific module
RUST_LOG=pristine::engine=debug cargo run --bin pristine -- overview ./my-project
```

### Using Debugger

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-gdb target/debug/pristine

# Or use VS Code debugger
# Set breakpoints and press F5
```

### Frontend Debugging

```bash
cd web

# Start development server with React DevTools
npm run dev

# Open browser DevTools (F12)
# Use React DevTools extension
```

## Contributing

### Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test --workspace`
5. Run lints: `cargo clippy --workspace -- -D warnings`
6. Format code: `cargo fmt`
7. Commit your changes
8. Push to your fork
9. Open a pull request

### Commit Message Format

Follow conventional commits:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(cli): add --explain flag to show selection reasons

fix(cache): handle cache invalidation on snapshot update

docs(api): add examples for all endpoints
```

### Pull Request Guidelines

1. **Description:** Clearly describe what the PR does and why
2. **Tests:** Include tests for new functionality
3. **Documentation:** Update documentation for any changes
4. **Changelog:** Add a changelog entry if applicable
5. **Review:** Address review comments promptly

### Code Review Process

1. All changes require at least one review
2. CI must pass before merging
3. Squash commits before merging
4. Delete branch after merging

## Release Process

### Versioning

Pristine follows [Semantic Versioning](https://semver.org/):

- **MAJOR:** Incompatible API changes
- **MINOR:** New functionality (backwards-compatible)
- **PATCH:** Bug fixes (backwards-compatible)

### Creating a Release

1. Update version in `Cargo.toml` and all crate `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Create a git tag: `git tag v0.1.0`
4. Push the tag: `git push origin v0.1.0`
5. GitHub Actions will build and publish the release

## Troubleshooting

### Common Issues

#### Build Failures

```bash
# Clean and rebuild
cargo clean
cargo build --workspace

# Update dependencies
cargo update
```

#### Test Failures

```bash
# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run specific test
cargo test test_name -- --nocapture
```

#### Frontend Build Issues

```bash
cd web

# Clear node modules
rm -rf node_modules
npm install

# Clear build cache
rm -rf dist
npm run build
```

#### Port Already in Use

```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>

# Or use different port
cargo run --bin pristine-api -- --port 8081
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/tokio/tutorial)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [React Documentation](https://react.dev/)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)
