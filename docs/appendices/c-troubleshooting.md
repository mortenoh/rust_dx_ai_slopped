# Troubleshooting

Solutions to common CLI development issues.

## Compilation Errors

### "trait X is not implemented for Y"

```rust
// Problem: Display trait not implemented
println!("{}", my_struct);

// Solution: Derive or implement Display
#[derive(Debug)]
struct MyStruct { ... }
println!("{:?}", my_struct);

// Or implement Display
impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MyStruct: {}", self.field)
    }
}
```

### "borrowed value does not live long enough"

```rust
// Problem
fn get_name() -> &str {
    let name = String::from("hello");
    &name  // Error: name doesn't live long enough
}

// Solution: Return owned value
fn get_name() -> String {
    String::from("hello")
}
```

### "cannot move out of borrowed content"

```rust
// Problem
fn process(items: &Vec<String>) {
    for item in items {  // Tries to move
        consume(item);
    }
}

// Solution: Clone or borrow
fn process(items: &Vec<String>) {
    for item in items {
        consume(item.clone());  // Clone
        // or
        use_ref(&item);  // Borrow
    }
}
```

## Runtime Errors

### File Not Found

```rust
// Problem
let content = std::fs::read_to_string("config.toml")?;

// Solution: Better error message
let content = std::fs::read_to_string("config.toml")
    .with_context(|| "Could not read config.toml")?;

// Or check existence first
let path = Path::new("config.toml");
if !path.exists() {
    eprintln!("Config file not found. Run 'dx init' first.");
    std::process::exit(1);
}
```

### Permission Denied

```rust
// Problem on Unix: can't execute
// Solution: Check/set permissions
use std::os::unix::fs::PermissionsExt;
let mut perms = fs::metadata(&path)?.permissions();
perms.set_mode(0o755);
fs::set_permissions(&path, perms)?;
```

## clap Issues

### Arguments Not Parsing

```rust
// Problem: positional after optional
#[derive(Parser)]
struct Cli {
    file: String,           // Positional
    #[arg(short)]
    verbose: bool,          // Optional
}
// dx file.txt -v  ✓
// dx -v file.txt  ✗ (file consumed as "-v")

// Solution: Put optionals first or use --
```

### Conflicting Short Options

```rust
// Problem: -h conflicts with --help
#[arg(short = 'H')]  // Use uppercase
hash: String,
```

## Testing Issues

### Test Output Not Visible

```bash
# Show output
cargo test -- --nocapture

# Show output for specific test
cargo test test_name -- --nocapture
```

### Tests Running in Parallel Conflict

```rust
// Problem: Tests share resource
#[test]
fn test_a() { write_to_file("test.txt"); }
#[test]
fn test_b() { write_to_file("test.txt"); }

// Solution: Use unique names or serialize
#[test]
fn test_a() { write_to_file("test_a.txt"); }

// Or use serial_test crate
#[serial]
#[test]
fn test_a() { ... }
```

### assert_cmd Binary Not Found

```rust
// Problem: binary not built
Command::cargo_bin("dx")?;

// Solution: Build first
cargo build
cargo test

// Or in test
Command::cargo_bin("dx")
    .expect("Binary not found. Run 'cargo build' first")
```

## Performance Issues

### Slow Startup

```bash
# Profile startup
hyperfine 'dx --version'

# Check what's slow
cargo flamegraph --bin dx -- --version
```

Causes:
- Loading large configs
- Initializing unused features
- Heavy static initialization

### High Memory Usage

```bash
# Monitor memory
/usr/bin/time -v dx process large_file.txt

# Stream instead of loading
let file = File::open(path)?;
let reader = BufReader::new(file);
for line in reader.lines() { ... }
```

## Cross-Platform Issues

### Path Separators

```rust
// Problem: Hardcoded separators
let path = "config/app.toml";  // Fails on Windows

// Solution: Use Path
let path = Path::new("config").join("app.toml");
```

### Line Endings

```rust
// Problem: Different line endings
assert_eq!(output, "line1\nline2");  // Fails on Windows

// Solution: Normalize
let normalized = output.replace("\r\n", "\n");
```
