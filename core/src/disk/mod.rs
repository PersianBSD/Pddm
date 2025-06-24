pub mod provider;
pub mod local;
pub mod os;

#[cfg(target_os = "windows")]
pub mod smart;
