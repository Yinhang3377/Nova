# Nova Tests

This directory contains test files for the Nova project.

## Test Structure

Organize tests to mirror your source code structure:

```
tests/
├── unit/           # Unit tests
├── integration/    # Integration tests
├── e2e/           # End-to-end tests
└── fixtures/      # Test data and fixtures
```

## Testing Frameworks

Nova supports various testing frameworks depending on your chosen language:

- **Python**: pytest, unittest, nose2
- **JavaScript**: Jest, Mocha, Jasmine
- **Go**: Built-in testing package
- **Rust**: Built-in testing with cargo test
- **Java**: JUnit, TestNG

## Running Tests

Add scripts to run tests for your specific implementation. Common patterns:

```bash
# Python
python -m pytest tests/

# JavaScript
npm test

# Go
go test ./...

# Rust
cargo test
```

## Test Guidelines

1. **Write clear test names** that describe what is being tested
2. **Follow the AAA pattern**: Arrange, Act, Assert
3. **Keep tests independent** - each test should be able to run in isolation
4. **Test edge cases** and error conditions
5. **Maintain test data** in the fixtures directory

## Example Test

```python
# Example: tests/unit/test_example.py
def test_greet_returns_correct_message():
    from src.example import greet
    result = greet()
    assert "Hello from Nova" in result
```

---

*Good tests are the foundation of reliable software. Test early, test often!*