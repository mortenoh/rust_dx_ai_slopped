# Configuration Management

Support configuration files for persistent settings.

## Configuration Hierarchy

1. Default values (in code)
2. Configuration file
3. Environment variables
4. Command-line arguments

## Using the config Crate

```toml
[dependencies]
config = "0.14"
serde = { version = "1", features = ["derive"] }
```

```rust
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub default_algorithm: String,
    pub output_format: String,
    pub color: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            // Defaults
            .set_default("default_algorithm", "sha256")?
            .set_default("output_format", "hex")?
            .set_default("color", true)?
            // Config file
            .add_source(
                File::with_name(&config_path())
                    .required(false)
            )
            // Environment variables (DX_*)
            .add_source(
                Environment::with_prefix("DX")
                    .separator("_")
            )
            .build()?;

        config.try_deserialize()
    }
}

fn config_path() -> String {
    dirs::config_dir()
        .map(|p| p.join("dx").join("config.toml"))
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
```

## Configuration File Format

```toml
# ~/.config/dx/config.toml

# Default hash algorithm
default_algorithm = "sha256"

# Output format: hex, base64
output_format = "hex"

# Enable colored output
color = true

[aliases]
h = "hash"
e = "encode"
```

## Nested Configuration

```rust
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub hash: HashConfig,
    pub encode: EncodeConfig,
}

#[derive(Debug, Deserialize)]
pub struct HashConfig {
    pub algorithm: String,
    pub uppercase: bool,
}

#[derive(Debug, Deserialize)]
pub struct EncodeConfig {
    pub default_encoding: String,
}
```

```toml
[hash]
algorithm = "sha256"
uppercase = false

[encode]
default_encoding = "base64"
```

## Init Command

```rust
fn init_config() -> Result<()> {
    let config_dir = dirs::config_dir()
        .ok_or("No config directory")?
        .join("dx");

    std::fs::create_dir_all(&config_dir)?;

    let config_file = config_dir.join("config.toml");
    if config_file.exists() {
        println!("Config already exists: {}", config_file.display());
        return Ok(());
    }

    let default_config = r#"# dx configuration

# Default hash algorithm
default_algorithm = "sha256"

# Output format: hex, base64
output_format = "hex"

# Enable colored output
color = true
"#;

    std::fs::write(&config_file, default_config)?;
    println!("Created config: {}", config_file.display());
    Ok(())
}
```

## Show Current Config

```rust
fn show_config(config: &AppConfig) {
    println!("Configuration:");
    println!("  Config file: {}", config_path());
    println!("  Algorithm: {}", config.default_algorithm);
    println!("  Format: {}", config.output_format);
    println!("  Color: {}", config.color);
}
```

## CLI Override

```rust
#[derive(Parser)]
struct Cli {
    /// Override default algorithm
    #[arg(long, env = "DX_ALGORITHM")]
    algorithm: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    let mut config = AppConfig::load().unwrap_or_default();

    // CLI overrides config
    if let Some(alg) = cli.algorithm {
        config.default_algorithm = alg;
    }
}
```
