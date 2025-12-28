use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct EguiArgs {
    #[command(subcommand)]
    pub command: EguiCommand,
}

#[derive(Subcommand, Debug)]
pub enum EguiCommand {
    // === Existing ===
    /// Basic hello world window
    Demo,
    /// Counter with increment/decrement buttons
    Counter,
    /// Live updating clock
    Clock,
    /// Work simulator with progress bar
    Work,

    // === Generators ===
    /// UUID generator with format selection and bulk generation
    Uuid,
    /// Password generator with strength indicator
    Password,
    /// QR code generator with live preview
    Qrcode,
    /// Lorem ipsum text generator
    Lorem,
    /// Color picker with HEX/RGB/HSL conversion
    Color,

    // === Encoders/Decoders ===
    /// Hash calculator (MD5, SHA256, SHA512)
    Hash,
    /// Base64 encoder/decoder
    Base64,
    /// Hex encoder/decoder with ASCII view
    Hex,
    /// URL encoder/decoder with breakdown
    Url,

    // === Converters ===
    /// Timestamp converter with multiple formats
    Timestamp,
    /// Unit converter (bytes, time)
    Units,
    /// Number base converter (bin/oct/dec/hex)
    Base,
    /// JSON formatter and validator
    Json,

    // === Utilities ===
    /// Regex pattern tester with highlighting
    Regex,
    /// Text diff viewer
    Diff,
    /// Stopwatch with lap times
    Stopwatch,
    /// Expression calculator
    Calculator,

    // === Text Tools ===
    /// Case converter (upper, lower, camel, snake, etc.)
    Case,
    /// Text statistics (chars, words, lines)
    TextStats,
    /// Markdown preview editor
    Markdown,
    /// Pomodoro timer
    Timer,

    // === Widget Showcase ===
    /// Data table with sorting and resizing
    Table,
    /// Modal/popup window dialogs
    Modal,
    /// Charts and graphs with egui_plot
    Plot,
    /// Image viewer with loading
    Image,
    /// Menu bar demonstration
    Menu,
    /// Context menu (right-click) demo
    Context,
    /// Tabbed interface demo
    Tabs,
    /// Tree view hierarchical data
    Tree,
    /// Code editor with syntax highlighting
    Code,
}
