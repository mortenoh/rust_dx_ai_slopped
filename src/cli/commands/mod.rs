//! Subcommand argument definitions.

pub mod calc;
pub mod chat;
pub mod compress;
pub mod config;
pub mod csv;
pub mod dhis2;
pub mod diff;
#[cfg(feature = "egui")]
pub mod egui;
pub mod encode;
pub mod encrypt;
pub mod env;
pub mod expr;
pub mod fun;
pub mod grep;
pub mod hash;
pub mod http;
pub mod json;
pub mod jwt;
pub mod markdown;
pub mod net;
pub mod rand;
pub mod system;
pub mod template;
pub mod text;
pub mod time;
#[cfg(feature = "ui")]
pub mod ui;
pub mod uuid;
pub mod watch;
pub mod xml;
pub mod yaml;

pub use calc::{CalcArgs, CalcCommand};
pub use chat::{ChatArgs, ChatCommand};
pub use compress::{CompressArgs, CompressCommand};
pub use config::{ConfigArgs, ConfigCommand, ConfigFormat};
pub use csv::{CsvArgs, CsvCommand};
pub use dhis2::{Dhis2Args, Dhis2Command};
pub use diff::DiffArgs;
#[cfg(feature = "egui")]
pub use egui::{EguiArgs, EguiCommand};
pub use encode::{EncodeArgs, EncodingFormat};
pub use encrypt::{EncryptArgs, EncryptCommand};
pub use env::{EnvArgs, EnvCommand, ExportFormat};
pub use expr::{ExprArgs, ExprCommand};
pub use fun::{FunArgs, FunCommand};
pub use grep::GrepArgs;
pub use hash::{Algorithm, HashArgs};
pub use http::{HttpArgs, HttpCommand};
pub use json::{JsonArgs, JsonCommand};
pub use jwt::{JwtArgs, JwtCommand};
pub use markdown::{MarkdownArgs, MarkdownCommand};
pub use net::{NetArgs, NetCommand};
pub use rand::{RandArgs, RandCommand};
pub use system::{SystemArgs, SystemCommand};
pub use template::{TemplateArgs, TemplateCommand};
pub use text::{TextArgs, TextCommand};
pub use time::{TimeArgs, TimeCommand, TimeFormat};
#[cfg(feature = "ui")]
pub use ui::UiArgs;
pub use uuid::{UuidArgs, UuidFormat, UuidVersion};
pub use watch::WatchArgs;
pub use xml::{XmlArgs, XmlCommand};
pub use yaml::{YamlArgs, YamlCommand};
