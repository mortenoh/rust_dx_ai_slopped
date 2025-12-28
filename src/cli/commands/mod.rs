//! Subcommand argument definitions.

pub mod calc;
pub mod chat;
pub mod config;
pub mod encode;
pub mod env;
pub mod expr;
pub mod fun;
pub mod grep;
pub mod hash;
pub mod http;
pub mod json;
pub mod net;
pub mod rand;
pub mod system;
pub mod text;
pub mod time;
#[cfg(feature = "ui")]
pub mod ui;
pub mod uuid;
pub mod watch;

pub use calc::{CalcArgs, CalcCommand};
pub use chat::{ChatArgs, ChatCommand};
pub use config::{ConfigArgs, ConfigCommand, ConfigFormat};
pub use encode::{EncodeArgs, EncodingFormat};
pub use env::{EnvArgs, EnvCommand, ExportFormat};
pub use expr::{ExprArgs, ExprCommand};
pub use fun::{FunArgs, FunCommand};
pub use grep::GrepArgs;
pub use hash::{Algorithm, HashArgs};
pub use http::{HttpArgs, HttpCommand};
pub use json::{JsonArgs, JsonCommand};
pub use net::{NetArgs, NetCommand};
pub use rand::{RandArgs, RandCommand};
pub use system::{SystemArgs, SystemCommand};
pub use text::{TextArgs, TextCommand};
pub use time::{TimeArgs, TimeCommand, TimeFormat};
#[cfg(feature = "ui")]
pub use ui::UiArgs;
pub use uuid::{UuidArgs, UuidFormat, UuidVersion};
pub use watch::WatchArgs;
