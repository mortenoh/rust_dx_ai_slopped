//! Top-level CLI argument definitions.

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

use super::commands::{
    CalcArgs, ChatArgs, ConfigArgs, EncodeArgs, EnvArgs, ExprArgs, FunArgs, GrepArgs, HashArgs,
    HttpArgs, JsonArgs, NetArgs, RandArgs, SystemArgs, TextArgs, TimeArgs, UuidArgs, WatchArgs,
};

/// dx - Developer Experience CLI
///
/// A production-ready developer toolkit with utilities for hashing,
/// encoding, UUID generation, timestamps, and more.
#[derive(Parser, Debug)]
#[command(
    name = "dx",
    author,
    version,
    about = "Developer Experience CLI - A toolkit for common developer tasks",
    long_about = None,
    propagate_version = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Disable colored output
    #[arg(long, global = true, env = "NO_COLOR")]
    pub no_color: bool,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Output format (text, json, or quiet)
    #[arg(short, long, global = true, default_value = "text")]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

/// Output format for command results
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text output
    #[default]
    Text,
    /// JSON output for scripting
    Json,
    /// Minimal output (values only)
    Quiet,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compute file or string hashes (MD5, SHA256, SHA512)
    #[command(visible_alias = "h")]
    Hash(HashArgs),

    /// Encode or decode data (base64, hex)
    #[command(visible_alias = "e")]
    Encode(EncodeArgs),

    /// Generate UUIDs (v4, v7)
    #[command(visible_alias = "u")]
    Uuid(UuidArgs),

    /// Convert and format timestamps
    #[command(visible_alias = "t")]
    Time(TimeArgs),

    /// Format and validate JSON
    #[command(visible_alias = "j")]
    Json(JsonArgs),

    /// Manage environment variables
    Env(EnvArgs),

    /// Manage application configuration
    #[command(visible_alias = "cfg")]
    Config(ConfigArgs),

    /// Generate random data (numbers, strings, passwords)
    #[command(visible_alias = "r")]
    Rand(RandArgs),

    /// Transform text (case conversion, slugify, etc.)
    Text(TextArgs),

    /// Unit conversions (bytes, time, base, percent)
    #[command(visible_alias = "c")]
    Calc(CalcArgs),

    /// Expression evaluator (math, functions, constants)
    #[command(visible_alias = "x")]
    Expr(ExprArgs),

    /// Network utilities (IP, DNS, ports)
    Net(NetArgs),

    /// Real-time chat using gRPC
    Chat(ChatArgs),

    /// Fun terminal effects (progress bars, spinners, hacker mode)
    Fun(FunArgs),

    /// Search for patterns in files (like grep)
    #[command(visible_alias = "g")]
    Grep(GrepArgs),

    /// Make HTTP requests
    Http(HttpArgs),

    /// Watch files for changes and run commands
    #[command(visible_alias = "w")]
    Watch(WatchArgs),

    /// System information and utilities
    #[command(visible_alias = "sys")]
    System(SystemArgs),

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

impl Cli {
    /// Generate shell completions and write to stdout
    pub fn print_completions(shell: Shell) {
        let mut cmd = Self::command();
        clap_complete::generate(shell, &mut cmd, "dx", &mut std::io::stdout());
    }
}
