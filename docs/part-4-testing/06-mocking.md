# Mocking and Test Doubles

Isolate code from dependencies during testing.

## Trait-Based Mocking

```rust
trait FileSystem {
    fn read(&self, path: &str) -> Result<String, Error>;
}

struct RealFs;
impl FileSystem for RealFs {
    fn read(&self, path: &str) -> Result<String, Error> {
        std::fs::read_to_string(path).map_err(Into::into)
    }
}

#[cfg(test)]
struct MockFs {
    files: HashMap<String, String>,
}

#[cfg(test)]
impl FileSystem for MockFs {
    fn read(&self, path: &str) -> Result<String, Error> {
        self.files.get(path).cloned().ok_or(Error::NotFound)
    }
}
```

## Using mockall

```bash
cargo add --dev mockall
```

```rust
use mockall::*;

#[automock]
trait HttpClient {
    fn get(&self, url: &str) -> Result<String, Error>;
}

#[test]
fn test_with_mock() {
    let mut mock = MockHttpClient::new();
    mock.expect_get()
        .returning(|_| Ok("response".to_string()));

    let result = fetch_data(&mock);
    assert!(result.is_ok());
}
```

## Dependency Injection

```rust
struct App<F: FileSystem> {
    fs: F,
}

impl<F: FileSystem> App<F> {
    fn process(&self, path: &str) -> Result<()> {
        let content = self.fs.read(path)?;
        // Process...
        Ok(())
    }
}
```
