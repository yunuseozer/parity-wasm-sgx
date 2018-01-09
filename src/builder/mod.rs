//! Various builders to generate/alter wasm components

mod invoke;
mod module;
mod code;
mod misc;
mod import;
mod memory;
mod table;
mod export;
mod global;
mod data;

pub use self::invoke::Identity;
pub use self::module::{module, from_module, ModuleBuilder};
pub use self::code::{signatures, signature, function, SignatureBuilder, FunctionBuilder};
pub use self::memory::MemoryBuilder;
pub use self::import::{import, ImportBuilder};
pub use self::export::{export, ExportBuilder};
pub use self::global::{global, GlobalBuilder};
