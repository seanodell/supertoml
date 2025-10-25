# Building SuperTOML

This document explains how to build SuperTOML from source.

## Prerequisites

- **Rust toolchain** (latest stable) - managed via `mise`
- **Docker** (optional, for clean environment testing)

## Development Setup

### Using mise (Recommended)

The project uses `mise` for tool management. Install mise first:

```bash
# Install mise (if not already installed)
curl https://mise.jdx.dev/install.sh | sh

# Install project tools
mise install
```

This will install:
- Rust 1.89.0
- pre-commit 3.7.0
- act (latest) for local GitHub Actions testing

### Manual Setup

If you prefer not to use mise:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install pre-commit
pip install pre-commit

# Install act (optional, for local GitHub Actions testing)
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

## Building

### Available Tasks

The project includes several `mise` tasks for building:

```bash
# Build for your current platform
mise run build

# Clean build artifacts
mise run clean

# Test in Docker (clean environment)
mise run docker-test
```

### Manual Commands

You can also use cargo directly:

```bash
# Build for your current platform
cargo build --release

# Clean build artifacts
cargo clean
```

## Testing Builds

### Local Testing

```bash
# Run all tests
mise run test
# or
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Docker Testing

```bash
# Test in Docker environment
mise run docker-test
```

Use Docker testing when you want to:
- Test in a clean environment
- Verify the build works in a different Linux distribution
- Ensure no local dependencies are missing
- Test the final binary in isolation

### Testing Built Binaries

```bash
# Test the locally built binary
./target/release/supertoml --help
./target/release/supertoml test.toml test
```

## Cross-Platform Building

### Supported Targets

- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-unknown-linux-gnu` (Linux x86_64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `i686-unknown-linux-gnu` (Linux 32-bit)

### Cross-Compilation

```bash
# Add target toolchain
rustup target add x86_64-unknown-linux-gnu

# Build for specific target
cargo build --release --target x86_64-unknown-linux-gnu

# Build for macOS from Linux (requires macOS SDK)
cargo build --release --target x86_64-apple-darwin
```

### Docker Cross-Compilation

For clean cross-compilation builds:

```bash
# Build in Docker container
docker run --rm -v $(pwd):/app -w /app rust:latest cargo build --release
```

## Build Optimization

### Release Builds

```bash
# Optimized release build
cargo build --release

# Strip debug symbols (reduces binary size)
strip target/release/supertoml
```

### Build Configuration

The project uses default Rust optimization settings:
- **Debug builds**: Fast compilation, debug symbols
- **Release builds**: Optimized for performance, no debug symbols

## Troubleshooting

### Build Issues

```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
mise run clean && mise run build

# Check for dependency issues
cargo tree
```

### Cross-Compilation Issues

- Ensure target toolchains are installed: `rustup target add <target>`
- Check that cross-compilation dependencies are available
- Use Docker testing for clean environment validation

### Test Failures

```bash
# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test_name

# Run tests in Docker
mise run docker-test
```

## Build Artifacts

After building, you'll find:

- **Debug binary**: `target/debug/supertoml`
- **Release binary**: `target/release/supertoml`
- **Cross-compiled binaries**: `target/<target>/release/supertoml`

## Performance Tips

- **Use `cargo check`** for fast feedback during development
- **Use `cargo build`** for debug builds during development
- **Use `cargo build --release`** for production builds
- **Use Docker** for clean, reproducible builds
