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
pub mod json;
pub mod net;
pub mod rand;
pub mod text;
pub mod time;
pub mod uuid;

pub use calc::{CalcArgs, CalcCommand};
pub use chat::{ChatArgs, ChatCommand};
pub use config::{ConfigArgs, ConfigCommand, ConfigFormat};
pub use encode::{EncodeArgs, EncodingFormat};
pub use env::{EnvArgs, EnvCommand, ExportFormat};
pub use expr::{ExprArgs, ExprCommand};
pub use fun::{FunArgs, FunCommand};
pub use grep::GrepArgs;
pub use hash::{Algorithm, HashArgs};
pub use json::{JsonArgs, JsonCommand};
pub use net::{NetArgs, NetCommand};
pub use rand::{RandArgs, RandCommand};
pub use text::{TextArgs, TextCommand};
pub use time::{TimeArgs, TimeCommand, TimeFormat};
pub use uuid::{UuidArgs, UuidFormat, UuidVersion};
