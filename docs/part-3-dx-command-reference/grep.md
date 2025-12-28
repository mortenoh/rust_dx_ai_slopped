# grep - Regex Search

The `grep` command searches for patterns in files using regular expressions.

## Usage

```bash
dx grep <PATTERN> [PATH]
```

## Options

| Option | Description |
|--------|-------------|
| `-i`, `--ignore-case` | Case insensitive search |
| `-n`, `--line-number` | Show line numbers (default: true) |
| `-c`, `--count` | Only show match count |
| `-l`, `--files-with-matches` | Only show file names |
| `-B <N>` | Show N lines before match |
| `-A <N>` | Show N lines after match |
| `-C <N>` | Show N lines before and after |
| `-r`, `--recursive` | Search recursively (default for directories) |
| `-e`, `--extension` | Filter by file extension |
| `--hidden` | Include hidden files |
| `--no-color` | Disable colored output |

## Examples

### Basic Search
```bash
# Search in current directory
dx grep "fn main"

# Search in specific path
dx grep "TODO" src/

# Search single file
dx grep "error" logs/app.log
```

### Case Insensitive
```bash
dx grep -i "error" src/
dx grep -i "warning" logs/
```

### Context Lines
```bash
# 2 lines before and after
dx grep -C 2 "panic" src/

# 3 lines before
dx grep -B 3 "fn main" src/

# 5 lines after
dx grep -A 5 "struct" src/
```

### Filter by Extension
```bash
dx grep "use tokio" src/ -e rs
dx grep "import" . -e py
dx grep "function" . -e js
```

### Count Matches
```bash
dx grep -c "TODO" src/
dx grep -c "unwrap" src/
```

### List Files Only
```bash
dx grep -l "async fn" src/
dx grep -l "test" tests/
```

### Include Hidden Files
```bash
dx grep --hidden "secret" .
```

## Regex Syntax

The command uses Rust's regex crate. Common patterns:

| Pattern | Matches |
|---------|---------|
| `.` | Any character |
| `*` | Zero or more of previous |
| `+` | One or more of previous |
| `?` | Zero or one of previous |
| `^` | Start of line |
| `$` | End of line |
| `\d` | Digit |
| `\w` | Word character |
| `\s` | Whitespace |
| `[abc]` | Character class |
| `(a\|b)` | Alternation |

## Examples with Regex

```bash
# Find function definitions
dx grep "fn \w+\(" src/

# Find TODO/FIXME comments
dx grep "TODO\|FIXME" src/

# Find lines starting with "use"
dx grep "^use" src/

# Find struct definitions
dx grep "^pub struct \w+" src/

# Find test functions
dx grep "#\[test\]" tests/
```
