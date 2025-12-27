# Man Pages

Generate Unix manual pages for your CLI.

## Using clap_mangen

```toml
[build-dependencies]
clap_mangen = "0.2"
clap = { version = "4", features = ["derive"] }
```

## Build Script

```rust
// build.rs
use clap::CommandFactory;
use clap_mangen::Man;
use std::fs;
use std::path::PathBuf;

include!("src/cli.rs");

fn main() {
    let out_dir = PathBuf::from("man");
    fs::create_dir_all(&out_dir).unwrap();

    let cmd = Cli::command();
    let man = Man::new(cmd.clone());

    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();

    fs::write(out_dir.join("dx.1"), buffer).unwrap();

    // Generate for subcommands
    for subcommand in cmd.get_subcommands() {
        let name = format!("dx-{}", subcommand.get_name());
        let man = Man::new(subcommand.clone());

        let mut buffer = Vec::new();
        man.render(&mut buffer).unwrap();

        fs::write(out_dir.join(format!("{}.1", name)), buffer).unwrap();
    }
}
```

## Generator Binary

Alternative approach with a separate binary:

```rust
// examples/generate-man.rs
use clap::CommandFactory;
use clap_mangen::Man;
use dx::Cli;

fn main() -> std::io::Result<()> {
    let cmd = Cli::command();
    let man = Man::new(cmd);

    man.render(&mut std::io::stdout())
}
```

```bash
cargo run --example generate-man > dx.1
```

## Man Page Sections

| Section | Content |
|---------|---------|
| 1 | User commands |
| 2 | System calls |
| 3 | Library functions |
| 5 | File formats |
| 7 | Miscellaneous |
| 8 | Admin commands |

## Installation

```bash
# Install to system
sudo install -m 644 man/dx.1 /usr/local/share/man/man1/

# Rebuild man database
sudo mandb

# View
man dx
```

## Customizing Man Output

```rust
use clap_mangen::Man;

let cmd = Cli::command()
    .version("1.0.0")
    .author("Your Name <you@example.com>")
    .about("Developer CLI tools")
    .after_help("EXAMPLES:\n    dx hash file.txt\n    dx encode -b base64 data");

let man = Man::new(cmd)
    .title("DX")
    .section("1")
    .manual("Developer Tools");
```

## Adding Examples Section

In your clap definition:

```rust
#[derive(Parser)]
#[command(
    after_help = r#"EXAMPLES:
    Hash a file:
        dx hash myfile.txt

    Encode to base64:
        dx encode --base64 "hello world"

    Generate UUID:
        dx uuid --version 7
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

## CI Integration

```yaml
# Generate man pages in CI
- name: Generate man pages
  run: cargo build --release

- name: Package man pages
  run: |
    mkdir -p dist/man
    cp man/*.1 dist/man/
    gzip dist/man/*.1
```

## Testing Man Pages

```bash
# Preview without installing
man ./man/dx.1

# Check for errors
groff -man -Tascii man/dx.1 > /dev/null

# Convert to HTML
groff -man -Thtml man/dx.1 > dx.html
```
