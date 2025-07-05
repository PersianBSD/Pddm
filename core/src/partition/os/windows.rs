use crate::utils::types::info::PartitionInfo;
use serde::Deserialize;
use std::process::Command;



#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartitionJson {
    disk_number: Option<u32>,
    partition_number: Option<u32>,
    drive_letter: Option<String>,
    offset: Option<u64>,
    size: Option<u64>,
    r#type: Option<String>,  // برای کلمه رزرو شده مثل `type`
    gpt_type: Option<String>,
    file_system_label: Option<String>,
    file_system_type: Option<String>,
    media_type: Option<String>,
    bus_type: Option<String>,
    is_boot: Option<bool>,
    is_system: Option<bool>,
    is_hidden: Option<bool>,
}

pub fn smart_partition_list() -> Result<Vec<PartitionInfo>, Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            r#"
            Get-Partition | ForEach-Object {
                $disk = Get-Disk -Number $_.DiskNumber
                $vol = $_ | Get-Volume -ErrorAction SilentlyContinue
                [PSCustomObject]@{
                    DiskNumber = $_.DiskNumber
                    PartitionNumber = $_.PartitionNumber
                    DriveLetter = $_.DriveLetter
                    Offset = $_.Offset
                    Size = $_.Size
                    Type = $_.Type
                    GptType = $_.GptType
                    FileSystemLabel = $vol.FileSystemLabel
                    FileSystemType = $vol.FileSystem
                    MediaType = $disk.MediaType
                    BusType = $disk.BusType
                    IsBoot = $_.IsBoot
                    IsSystem = $_.IsSystem
                    IsHidden = $_.IsHidden
                }
            } | ConvertTo-Json -Compress
            "#,
        ])
        .output()?;

    let json = String::from_utf8_lossy(&output.stdout);
    let partitions: Vec<PartitionJson> = serde_json::from_str(&json)?;

    let result = partitions
        .into_iter()
        .map(|p| PartitionInfo {
            partition_name: p.drive_letter.clone().unwrap_or_else(|| "<no name>".into()),
            mount_point: p.drive_letter.clone().map(|s| format!("{}:\\", s)),
            file_system: p.file_system_type,
            total_space: p.size,
            used_space: None, // قابل محاسبه نیست اینجا
            free_space: None, // قابل محاسبه نیست اینجا
            volume_label: p.file_system_label,
            volume_id: p.gpt_type.clone(),

            disk_number: p.disk_number,
            partition_number: p.partition_number,
            offset: p.offset,
            media_type: p.media_type,
            bus_type: p.bus_type,
            is_boot: p.is_boot,
            is_system: p.is_system,
            is_hidden: p.is_hidden,
            guid_type: p.gpt_type.clone(),
            partition_type: p.r#type,
        })
        .collect();

    Ok(result)
}

/// Filters partitions based on disk name like \"Disk0\", \"Disk1\" etc.
pub fn get_partitions_for_disk(disk_name: &str) -> Result<Vec<PartitionInfo>, Box<dyn std::error::Error>> {
    let all_parts = smart_partition_list()?;

    Ok(all_parts.into_iter()
        .filter(|p| {
            if let Some(num) = p.disk_number {
                let matches = disk_name.ends_with(&format!("PhysicalDrive{}", num));
                //println!("DISK_MATCH? disk_name='{}' vs num={} => {}", disk_name, num, matches);
                matches
            } else {
                false
            }
        })
        .collect())
}
