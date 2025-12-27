# Help Customization

Customize how your CLI presents help information to users.

## Basic Customization

```rust
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "myapp",
    version = "1.0",
    about = "Short description",
    long_about = "Detailed description\n\nWith multiple paragraphs.",
    after_help = "Examples:\n  myapp input.txt\n  myapp -v input.txt output.txt",
    author = "Your Name <you@example.com>"
)]
struct Args {
    /// Input file to process
    input: String,
}
```

## Custom Help Templates

```rust
#[derive(Parser)]
#[command(help_template = "\
{name} {version}
{author}

{about}

{usage-heading} {usage}

{all-args}

{after-help}
")]
struct Args {}
```

Template variables: `{name}`, `{version}`, `{author}`, `{about}`, `{usage}`, `{all-args}`, `{options}`, `{positionals}`, `{subcommands}`.

## Styled Help

```rust
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};

#[derive(Parser)]
#[command(styles = Styles::styled()
    .header(AnsiColor::Green.on_default() | Effects::BOLD)
    .usage(AnsiColor::Cyan.on_default())
    .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
)]
struct Args {}
```

## Hiding Elements

```rust
#[derive(Parser)]
struct Args {
    #[arg(long, hide = true)]
    internal: Option<String>,

    #[arg(long, default_value = "secret", hide_default_value = true)]
    token: String,

    #[arg(long, value_enum, hide_possible_values = true)]
    mode: Mode,
}
```

## Next/Previous Links

```rust
#[derive(Parser)]
#[command(
    after_help = "See 'myapp help <command>' for more information.",
    after_long_help = "DOCUMENTATION\n  Full docs: https://example.com/docs"
)]
struct Args {}
```
