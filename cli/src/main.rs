// use pddm_core::disk::local::get_disks;
// use std::collections::HashMap;
// use pddm_core::partition::local::get_partitions;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let disks = get_disks();
//     let partitions = get_partitions();

//     // گروه‌بندی پارتیشن‌ها بر اساس disk_number
//     let mut parts_by_disk: HashMap<u32, Vec<_>> = HashMap::new();
//     for part in partitions {
//         if let Some(disk_num) = part.disk_number {
//             parts_by_disk.entry(disk_num).or_default().push(part);
//         }
//     }

//     // چاپ اطلاعات
//     for (i, disk) in disks.iter().enumerate() {
//         let size_gb = disk.size_gb / 1_000_000_000;
//         println!(
//             "💽 Disk {} | Model: {} | Size: {}GB | Bus: {} | Type: {} | Serial: {}",
//             i,
//             disk.model.clone().unwrap_or("-".into()),
//             size_gb,
//             disk.bus_type.clone().unwrap_or("-".into()),
//             disk.media_type.clone().unwrap_or("-".into()),
//             disk.serial.clone().unwrap_or("-".into())
//         );

//         if let Some(parts) = parts_by_disk.get(&(i as u32)) {
//             for (j, part) in parts.iter().enumerate() {
//                 let total = format_bytes(part.total_space.unwrap_or(0));
//                 let free = format_bytes(part.free_space.unwrap_or(0));
//                 println!(
//                     "    ├─ 🧩 Partition {} | Drive: {} | Label: {} | Type: {} | FS: {} | Volume: {} | Free: {}",
//                     j + 1,
//                     part.mount_point.clone().unwrap_or("-".into()),
//                     part.volume_label.clone().unwrap_or("-".into()),
//                     part.partition_type.clone().unwrap_or("-".into()),
//                     part.file_system.clone().unwrap_or("-".into()),
//                     total,
//                     free
//                 );
//             }
//         } else {
//             println!("    ⚠️  بدون پارتیشن یافت‌شده");
//         }
//         println!(); // خط جداکننده بین دیسک‌ها
//     }

//     Ok(())
// }

// fn format_bytes(bytes: u64) -> String {
//     if bytes > 1_000_000_000 {
//         format!("{:.1}GB", bytes as f64 / 1_000_000_000.0)
//     } else if bytes > 1_000_000 {
//         format!("{:.1}MB", bytes as f64 / 1_000_000.0)
//     } else {
//         format!("{}B", bytes)
//     }
// }

use pddm_core::disk::smart::smart_disks_list;
use pddm_core::partition::smart::smart_partition_list;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let disks = smart_disks_list()?;
    let partitions = smart_partition_list()?;

    // گروه‌بندی پارتیشن‌ها بر اساس نام دیسک
    let mut disk_part_map: HashMap<String, Vec<_>> = HashMap::new();
    for p in partitions {
        if let Some(disk_name) = &p.name {
            disk_part_map.entry(disk_name.clone()).or_default().push(p);
        }
    }

    for (i, disk) in disks.iter().enumerate() {
        println!("{}", "—".repeat(80));
        println!(
            "Disk Num | Model         | GPT/MBR | Size (GB)"
        );
        println!("{}", "—".repeat(80));
        println!(
            "Disk {:<4} | {:<13} | {:<7} | {:>8}",
            i,
            disk.model.clone().unwrap_or("-".into()),
            disk.partition_style.clone().unwrap_or("-".into()).to_lowercase(),
            disk.size_gb
        );

        println!("{}", "—".repeat(80));
        println!("Partition | FS    | Size (GB) | GUID");
        println!("{}", "—".repeat(80));

        if let Some(parts) = disk_part_map.get(&disk.name) {
            for part in parts {
                let size = part.total_space.unwrap_or(0) / 1024 / 1024 / 1024;
                println!(
                    "{:<9} | {:<5} | {:>9} | {}",
                    part.name,
                    part.file_system.clone().unwrap_or("-".into()),
                    size,
                    part.guid_type.clone().unwrap_or("-".into())
                );
            }
        } else {
            println!("(No partitions found)");
        }
    }

    Ok(())
}

