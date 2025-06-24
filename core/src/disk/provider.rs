#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub size_gb: u64,
    pub is_removable: bool,
    pub model: Option<String>,
    pub serial: Option<String>,
    pub partition_style : Option<String>,
    pub bus_type: Option<String>,       
    pub media_type: Option<String>, 
}

