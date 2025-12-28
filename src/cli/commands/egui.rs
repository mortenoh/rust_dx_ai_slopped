use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct EguiArgs {
    #[command(subcommand)]
    pub command: EguiCommand,
}

#[derive(Subcommand, Debug)]
pub enum EguiCommand {
    /// Basic hello world window
    Demo,
    /// Counter with increment/decrement buttons
    Counter,
    /// Live updating clock
    Clock,
}
