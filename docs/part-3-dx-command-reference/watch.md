# watch - File Watcher

The `watch` command monitors files for changes and runs commands automatically.

## Usage

```bash
dx watch <PATH> [OPTIONS] -- <COMMAND>
```

## Options

| Option | Description |
|--------|-------------|
| `-e`, `--extension` | Filter by file extension (can repeat) |
| `-d`, `--debounce` | Debounce delay in milliseconds (default: 300) |
| `-c`, `--clear` | Clear screen before each run |
| `--no-initial` | Don't run command on startup |

## Examples

### Basic Watch
```bash
# Watch directory and run command
dx watch src/ -- cargo build

# Watch current directory
dx watch . -- make test
```

### Filter by Extension
```bash
# Only watch .rs files
dx watch src/ -e rs -- cargo test

# Watch multiple extensions
dx watch . -e rs -e toml -- cargo build

# Watch JavaScript files
dx watch src/ -e js -e ts -- npm test
```

### With Clear Screen
```bash
# Clear before each run
dx watch src/ -c -- cargo test

# Clear and run tests
dx watch tests/ -c -e rs -- cargo test
```

### Custom Debounce
```bash
# Longer debounce for slow commands
dx watch . -d 1000 -- cargo build --release

# Shorter debounce for fast commands
dx watch src/ -d 100 -- cargo check
```

### Skip Initial Run
```bash
# Don't run on startup, only on changes
dx watch src/ --no-initial -- cargo test
```

## Common Use Cases

### Development Workflow
```bash
# Auto-rebuild on changes
dx watch src/ -e rs -- cargo build

# Auto-test on changes
dx watch . -e rs -c -- cargo test

# Auto-format and build
dx watch src/ -e rs -- "cargo fmt && cargo build"
```

### Web Development
```bash
# Rebuild frontend
dx watch src/ -e js -e css -- npm run build

# Restart server
dx watch . -e py -- python app.py
```

### Documentation
```bash
# Rebuild mdbook
dx watch docs/ -e md -- mdbook build
```

## How It Works

1. Uses `notify` crate for cross-platform file watching
2. Debounces rapid changes to avoid multiple runs
3. Runs the command in a shell (`sh -c` on Unix, `cmd /C` on Windows)
4. Shows which file triggered the change
5. Reports command exit status

## Output

```
[watch] Watching src/ for changes...
[watch] Running: cargo test
   Compiling dx v0.1.0
    Finished dev [unoptimized + debuginfo]
     Running tests
[watch] Command completed (exit code: 0)
[watch] Waiting for changes...
[watch] File changed: src/main.rs
[watch] Running: cargo test
...
```

Press `Ctrl+C` to stop watching.
