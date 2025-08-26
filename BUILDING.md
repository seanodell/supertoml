# Building and Distributing SuperTOML

This document explains how to build SuperTOML and distribute it through various package managers.

## Supported Platforms

- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)
- **Windows**: x86_64 (Intel/AMD)
- **Linux**: x86_64 (Intel/AMD), aarch64 (ARM64), i686 (32-bit Intel)

## Local Development

### Prerequisites
- Rust toolchain (latest stable)
- Docker (for testing in clean environment)

### Building and Testing

Using `mise` tasks (recommended):
```bash
# Build for your current platform
mise run build

# Run tests
mise run test

# Check code quality
mise run check

# Format code
mise run fmt

# Check code formatting (CI)
mise run fmt-check

# Clean build artifacts
mise run clean

# Test in Docker (clean environment)
mise run docker-test
```

Or using cargo directly:
```bash
# Build for your current platform
cargo build --release

# Run tests
cargo test

# Check code quality
cargo check && cargo clippy

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check
```

## Multi-Platform Builds

**All multi-platform builds are handled automatically by GitHub Actions.**

### Creating a Release
Releases are created automatically when you push a tag to GitHub:

```bash
# 1. Update version in Cargo.toml
# 2. Create and push a tag
git tag v0.1.0
git push origin v0.1.0
```

This will automatically:
1. Build for all platforms (macOS, Windows, Linux)
2. Run tests on all platforms
3. Create a GitHub release with all binaries
4. Generate release notes

### Checking Build Progress
After pushing a tag, check the build progress at:
https://github.com/seanodell/supertoml/actions

## Testing

### Local Testing
```bash
# Run all tests
mise run test
# or
cargo test

# Run tests with output
cargo test -- --nocapture
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

## Distribution

### GitHub Releases (Automated)
Releases are created automatically when you push a tag. Each release includes:
- Binaries for all supported platforms
- SHA256 checksums
- Release notes

### Package Managers

#### Homebrew
To add to Homebrew core:
1. Fork the Homebrew/homebrew-core repository
2. Add your formula to `Formula/supertoml.rb`
3. Submit a pull request

#### Mise
To add to mise registry:
1. Submit a pull request to the mise registry
2. Include installation URLs for all platforms

#### Cargo Install
Publish to crates.io:
```bash
# Publish to crates.io
cargo publish

# Users can then install with
cargo install supertoml
```

## CI/CD Pipeline

The GitHub Actions workflow (`.github/workflows/release.yml`) handles:

1. **Multi-platform builds** using matrix strategy
2. **Cross-compilation** for all target platforms
3. **Automated testing** of built binaries
4. **Release creation** with all artifacts
5. **SHA256 calculation** for package managers

## Release Process

1. **Update version** in `Cargo.toml`
2. **Create and push tag**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
3. **Wait for CI** to build and release (check GitHub Actions)
4. **Submit to package managers** (if desired)
5. **Test installations** from all sources

## Troubleshooting

### Local Build Issues
- Ensure you have the latest Rust toolchain: `rustup update`
- Clean and rebuild: `mise run clean && mise run build`

### GitHub Actions Issues
- Check the Actions tab for detailed error messages
- Ensure all tests pass locally before pushing a tag
- Verify the tag format: `v0.1.0` (not `0.1.0`)

### Package Manager Issues
- Verify SHA256 checksums match the released binaries
- Test the formula locally before submitting to package managers

## Security Considerations

- **Code signing**: Consider signing binaries for macOS
- **Reproducible builds**: Ensure consistent builds across environments
- **Vulnerability scanning**: Scan dependencies regularly
- **Checksums**: Always provide SHA256 checksums for downloads
