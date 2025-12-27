//! # Shell Completions
//!
//! This example shows how to generate shell completions.
//!
//! Run with: `cargo run --example 1001_shell_completions`

#![allow(dead_code)]

fn main() {
    println!("=== Shell Completions ===\n");

    // =========================================================================
    // BUILD-TIME GENERATION
    // =========================================================================

    println!("--- Build-Time Generation ---");
    println!(
        r#"
Generate completions during build:

# build.rs
use clap::CommandFactory;
use clap_complete::{{generate_to, Shell}};
use std::{{env, fs}};

include!("src/cli/args.rs");

fn main() {{
    let outdir = env::var_os("OUT_DIR").unwrap();
    let mut cmd = Cli::command();

    // Create completions directory
    let completions_dir = std::path::Path::new(&outdir).join("completions");
    fs::create_dir_all(&completions_dir).unwrap();

    // Generate for each shell
    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {{
        generate_to(shell, &mut cmd, "dx", &completions_dir).unwrap();
    }}

    // Tell cargo to re-run if args change
    println!("cargo:rerun-if-changed=src/cli/args.rs");
}}

Completions are generated to:
  target/<profile>/build/dx-<hash>/out/completions/
"#
    );

    println!();

    // =========================================================================
    // RUNTIME GENERATION
    // =========================================================================

    println!("--- Runtime Generation ---");
    println!(
        r#"
Generate completions at runtime via subcommand:

// src/cli/args.rs
#[derive(Subcommand)]
pub enum Commands {{
    // ... other commands ...

    /// Generate shell completions
    Completions {{
        /// Shell to generate for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    }},
}}

// src/commands/completions.rs
use clap::CommandFactory;
use clap_complete::generate;
use std::io;

pub fn run(shell: clap_complete::Shell) {{
    let mut cmd = crate::cli::Cli::command();
    generate(shell, &mut cmd, "dx", &mut io::stdout());
}}

Usage:
  dx completions bash > ~/.local/share/bash-completion/completions/dx
  dx completions zsh > ~/.zfunc/_dx
  dx completions fish > ~/.config/fish/completions/dx.fish
  dx completions powershell > dx.ps1
"#
    );

    println!();

    // =========================================================================
    // SHELL-SPECIFIC SETUP
    // =========================================================================

    println!("--- Shell-Specific Setup ---");
    println!(
        r#"
Setup instructions per shell:

BASH:
  # Add to ~/.bashrc
  source <(dx completions bash)

  # Or install globally
  dx completions bash | sudo tee /etc/bash_completion.d/dx

ZSH:
  # Add to ~/.zshrc (before compinit)
  mkdir -p ~/.zfunc
  dx completions zsh > ~/.zfunc/_dx
  fpath+=~/.zfunc

  # Or with oh-my-zsh
  dx completions zsh > ~/.oh-my-zsh/completions/_dx

FISH:
  dx completions fish > ~/.config/fish/completions/dx.fish

POWERSHELL:
  # Add to $PROFILE
  dx completions powershell | Out-String | Invoke-Expression

  # Or save to file and source
  dx completions powershell > dx.ps1
  . ./dx.ps1
"#
    );

    println!();

    // =========================================================================
    // COMPLETION FEATURES
    // =========================================================================

    println!("--- Completion Features ---");
    println!(
        r#"
What gets completed:

1. SUBCOMMANDS
   $ dx h<TAB>
   hash

2. OPTIONS
   $ dx hash --<TAB>
   --algorithm  --help  --output  --quiet

3. OPTION VALUES (value_enum)
   $ dx hash --algorithm <TAB>
   md5  sha256  sha512

4. FILE PATHS
   $ dx hash <TAB>
   file.txt  src/  docs/

For custom completions, use value_hint:

#[arg(value_hint = clap::ValueHint::FilePath)]
file: PathBuf,

#[arg(value_hint = clap::ValueHint::DirPath)]
directory: PathBuf,

#[arg(value_hint = clap::ValueHint::Hostname)]
host: String,

#[arg(value_hint = clap::ValueHint::Url)]
url: String,
"#
    );

    println!();

    // =========================================================================
    // DYNAMIC COMPLETIONS
    // =========================================================================

    println!("--- Dynamic Completions ---");
    println!(
        r#"
For dynamic completions (e.g., from config), use clap_complete_command:

# Cargo.toml
[dependencies]
clap_complete_command = "0.5"

use clap_complete_command::Commander;

#[derive(Parser)]
struct Cli {{
    #[command(subcommand)]
    command: Commands,
}}

#[derive(Subcommand)]
enum Commands {{
    /// Shell completions
    Completions(Commander<Self>),
    // ...
}}

fn main() {{
    let cli = Cli::parse();
    match cli.command {{
        Commands::Completions(cmd) => cmd.run(),
        // ...
    }}
}}

For truly dynamic completions (e.g., listing profiles):

# completions/dx.bash
_dx_profiles() {{
    dx config profile list --quiet 2>/dev/null
}}

complete -F _dx_profiles dx config profile use
"#
    );

    println!();

    // =========================================================================
    // DISTRIBUTION
    // =========================================================================

    println!("--- Distributing Completions ---");
    println!(
        r#"
Include completions in packages:

# Package layout
dx-v1.0.0-linux-x64/
├── dx
├── README.md
├── LICENSE
└── completions/
    ├── dx.bash
    ├── _dx (zsh)
    ├── dx.fish
    └── _dx.ps1

# Debian package (Cargo.toml)
[package.metadata.deb]
assets = [
    ["completions/dx.bash", "usr/share/bash-completion/completions/dx", "644"],
    ["completions/_dx", "usr/share/zsh/site-functions/_dx", "644"],
    ["completions/dx.fish", "usr/share/fish/completions/dx.fish", "644"],
]

# Homebrew
def install
  bin.install "dx"
  bash_completion.install "completions/dx.bash" => "dx"
  zsh_completion.install "completions/_dx"
  fish_completion.install "completions/dx.fish"
end
"#
    );

    // =========================================================================
    // SUMMARY
    // =========================================================================

    println!("=== Summary ===");
    println!("Shell completions:");
    println!("  1. Generate in build.rs or via subcommand");
    println!("  2. Support Bash, Zsh, Fish, PowerShell");
    println!("  3. Use value_hint for file/dir/url completion");
    println!("  4. Include completions in packages");
    println!("  5. Provide install instructions");
}
