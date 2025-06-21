use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use crate::partition::provider::PartitionInfo;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Win32LogicalDisk {
    pub device_id: String,
    pub volume_name: Option<String>,
    pub file_system: Option<String>,
    pub size: Option<String>,
    pub free_space: Option<String>,
    pub drive_type: u32,
}

pub fn list_partitions_wmi() -> Result<Vec<PartitionInfo>, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    let results: Vec<Win32LogicalDisk> = wmi_con.query()?;

    let partitions = results
        .into_iter()
        .map(|p| {
            let size_bytes = p
                .size
                .as_deref()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let free_bytes = p
                .free_space
                .as_deref()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0);
            let used_bytes = Some(size_bytes.saturating_sub(free_bytes));

            PartitionInfo {
                name: Some(p.device_id.clone()),
                volume_label: p.volume_name,
                file_system: p.file_system,
                total_space: size_bytes,
                free_space: free_bytes,
                used_space: used_bytes,
                is_removable: p.drive_type == 2, // 2 = Removable Drive
                mount_point: p.device_id,
            }
        })
        .collect();

    Ok(partitions)
}
