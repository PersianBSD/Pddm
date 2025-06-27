use crate::partition::provider::PartitionInfo;

/// نتیجه تحلیل فضای آزاد
pub struct FreeSpaceSummary {
    pub total_space: u64,
    pub used_space: u64,
    pub free_space: u64,
    pub usage_percent: f64,
}

/// دریافت خلاصه وضعیت فضای دیسک از روی لیست پارتیشن‌ها
pub fn summarize_partitions(partitions: &[PartitionInfo]) -> FreeSpaceSummary {
    let mut total = 0u64;
    let mut used = 0u64;
    let mut free = 0u64;

    for p in partitions {
        if let Some(t) = p.total_space {
            total += t;
        }
        if let Some(u) = p.used_space {
            used += u;
        }
        if let Some(f) = p.free_space {
            free += f;
        }
    }

    let usage_percent = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    FreeSpaceSummary {
        total_space: total,
        used_space: used,
        free_space: free,
        usage_percent,
    }
}
