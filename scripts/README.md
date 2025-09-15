# Nova Scripts

This directory contains build, deployment, and utility scripts for the Nova project.

## Available Scripts

- `setup.sh` - Project setup and initialization
- `test.sh` - Run all tests
- `build.sh` - Build the project
- `clean.sh` - Clean build artifacts

## Usage

Make scripts executable and run them from the project root:

```bash
chmod +x scripts/*.sh
./scripts/setup.sh
```

## Script Guidelines

1. **Make scripts executable**: `chmod +x script_name.sh`
2. **Include error handling**: Check exit codes and handle failures
3. **Add help text**: Include usage information
4. **Use proper shebang**: Start with `#!/bin/bash` or appropriate interpreter
5. **Document parameters**: Explain any command-line arguments

## Examples

### Running Setup
```bash
./scripts/setup.sh
```

### Running Tests
```bash
./scripts/test.sh
```

### Building Project
```bash
./scripts/build.sh
```

---

*Scripts automate repetitive tasks and ensure consistent project operations.*