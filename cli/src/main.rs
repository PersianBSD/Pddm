use pddm_core::disk::os::windows::list_disks_smart;

fn main() {
    println!("=== Testing list_disks_smart ===");
    match list_disks_smart() {
        Ok(disks) => {
            for disk in disks {
                println!("Name        : {}", disk.name);
                println!("Size (GB)   : {}", disk.size_gb);
                println!("Removable   : {}", disk.is_removable);
                println!("Model       : {}", disk.model.unwrap_or_else(|| "<unknown>".into()));
                println!("Serial      : {}", disk.serial.unwrap_or_else(|| "<unknown>".into()));
                println!("---------------------------");
            }
        }
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
