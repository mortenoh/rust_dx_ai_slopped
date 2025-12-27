# Common Crates

Essential crates for CLI development.

## CLI Framework

| Crate | Purpose |
|-------|---------|
| `clap` | Argument parsing (derive or builder) |
| `clap_complete` | Shell completions |
| `clap_mangen` | Man page generation |

```toml
clap = { version = "4", features = ["derive"] }
```

## Error Handling

| Crate | Purpose |
|-------|---------|
| `anyhow` | Easy error handling |
| `thiserror` | Custom error types |
| `color-eyre` | Colorful error reports |

```toml
anyhow = "1"
thiserror = "1"
```

## Serialization

| Crate | Purpose |
|-------|---------|
| `serde` | Serialization framework |
| `serde_json` | JSON support |
| `toml` | TOML support |
| `serde_yaml` | YAML support |

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"
```

## Terminal Output

| Crate | Purpose |
|-------|---------|
| `colored` | Colored text |
| `indicatif` | Progress bars |
| `console` | Terminal utilities |
| `dialoguer` | User prompts |
| `tabled` | Table formatting |

```toml
colored = "2"
indicatif = "0.17"
```

## File System

| Crate | Purpose |
|-------|---------|
| `walkdir` | Directory traversal |
| `glob` | Pattern matching |
| `tempfile` | Temporary files |
| `dirs` | Standard directories |

```toml
walkdir = "2"
dirs = "5"
```

## Async Runtime

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `async-std` | Alternative async |
| `futures` | Async utilities |

```toml
tokio = { version = "1", features = ["full"] }
```

## HTTP Client

| Crate | Purpose |
|-------|---------|
| `reqwest` | Full-featured HTTP |
| `ureq` | Simple blocking HTTP |

```toml
reqwest = { version = "0.12", features = ["json"] }
```

## Cryptography

| Crate | Purpose |
|-------|---------|
| `sha2` | SHA-256/512 |
| `md-5` | MD5 (legacy) |
| `hex` | Hex encoding |
| `base64` | Base64 encoding |

```toml
sha2 = "0.10"
hex = "0.4"
```

## Date/Time

| Crate | Purpose |
|-------|---------|
| `chrono` | Full-featured dates |
| `time` | Lighter alternative |

```toml
chrono = "0.4"
```

## Logging

| Crate | Purpose |
|-------|---------|
| `tracing` | Structured logging |
| `tracing-subscriber` | Log formatting |
| `log` | Simple logging |
| `env_logger` | Env-configured logs |

```toml
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Testing

| Crate | Purpose |
|-------|---------|
| `assert_cmd` | CLI testing |
| `predicates` | Assertion helpers |
| `insta` | Snapshot testing |
| `proptest` | Property testing |

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
insta = "1"
```

## Configuration

| Crate | Purpose |
|-------|---------|
| `config` | Configuration management |
| `figment` | Advanced config |
| `dotenvy` | .env file loading |

```toml
config = "0.14"
dotenvy = "0.15"
```

## Parallel Processing

| Crate | Purpose |
|-------|---------|
| `rayon` | Data parallelism |
| `crossbeam` | Concurrency primitives |

```toml
rayon = "1"
```
