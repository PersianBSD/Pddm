// core/lib/mod.rs

pub mod chs;

pub mod convert;

pub mod errors;

pub mod format;

pub mod guid;

pub mod legacy_ebr;

#[cfg(target_os = "windows")]
pub mod win_mount;

pub mod manager;

pub mod checker;

pub mod types;