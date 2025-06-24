// core/src/partition/provider.rs
#[derive(Debug, Clone)]
pub struct PartitionInfo {
    pub partition_name: String,
    pub mount_point: Option<String>,
    pub file_system: Option<String>,
    pub total_space: Option<u64>,
    pub used_space: Option<u64>,
    pub free_space: Option<u64>,
    pub volume_label: Option<String>,
    pub volume_id: Option<String>,

    // اطلاعات تکمیلی
    pub disk_number: Option<u32>,
    pub partition_number: Option<u32>,
    pub offset: Option<u64>,
    pub media_type: Option<String>,
    pub bus_type: Option<String>,
    pub is_boot: Option<bool>,
    pub is_system: Option<bool>,
    pub is_hidden: Option<bool>,
    pub guid_type: Option<String>,
    pub partition_type: Option<String>,
}
