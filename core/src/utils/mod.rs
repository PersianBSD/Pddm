// core/lib/mod.rs

//pub use chs;


pub mod convert;

//pub mod crc;

//pub mod free_space;

pub mod errors;

pub mod format;

//pub mod free_space_manager;

pub mod guid;

pub mod legacy_ebr;

#[cfg(target_os = "windows")]
pub mod win_mount;

#[cfg(target_os = "windows")]
//pub mod windows;

pub mod types;

pub mod manager;

pub mod checker;

#[cfg(target_os = "linux")]
mod linux;

