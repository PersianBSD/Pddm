use std::collections::HashMap;
use std::process::Command;

/// تابعی برای دریافت نگاشت drive letter به mount point در ویندوز
pub fn get_windows_mount_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-Volume | Select-Object DriveLetter, Path | ConvertTo-Json -Compress",
        ])
        .output();

    if let Ok(output) = output {
        if let Ok(json) = String::from_utf8(output.stdout) {
            if let Ok(entries) = serde_json::from_str::<serde_json::Value>(&json) {
                if let Some(array) = entries.as_array() {
                    for item in array {
                        let drive = item.get("DriveLetter").and_then(|v| v.as_str()).unwrap_or("");
                        let path = item.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                        if !drive.is_empty() && !path.is_empty() {
                            map.insert(drive.to_string(), path.to_string());
                        }
                    }
                } else if let Some(drive) = entries.get("DriveLetter") {
                    // اگر فقط یک درایو برگشت داده شده
                    let path = entries.get("Path").and_then(|v| v.as_str()).unwrap_or("");
                    if let Some(drive) = drive.as_str() {
                        map.insert(drive.to_string(), path.to_string());
                    }
                }
            }
        }
    }

    map
}