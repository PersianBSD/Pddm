use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guid {
    pub one: u32,
    pub two: u16,
    pub three: u16,
    pub four: u64, // ترکیب 8 بایت آخر
}

#[derive(Debug, Clone)]
pub struct GuidEntry {
    pub guid: Guid,
    pub description: String,
    pub uid: u32,
}

pub struct GuidManager {
    guid_data: Vec<GuidEntry>,
}

impl GuidManager {
    pub fn new() -> Self {
        let mut manager = Self { guid_data: vec![] };
        manager.add_default_guids();
        manager
    }

    pub fn add(&mut self, guid: Guid, uid: u32, description: String) {
        self.guid_data.push(GuidEntry { guid, uid, description });
    }

    pub fn get_description(&self, guid: &Guid) -> Option<String> {
        self.guid_data
            .iter()
            .find(|entry| &entry.guid == guid)
            .map(|entry| entry.description.clone())
    }

    pub fn to_string(guid: &Guid) -> String {
        format!(
            "{:08X}-{:04X}-{:04X}-{:04X}-{:012X}",
            guid.one,
            guid.two,
            guid.three,
            (guid.four >> 48) as u16,
            guid.four & 0xFFFFFFFFFFFF
        )
    }

    pub fn from_string(s: &str) -> Option<Guid> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 5 {
            return None;
        }

        let one = u32::from_str_radix(parts[0], 16).ok()?;
        let two = u16::from_str_radix(parts[1], 16).ok()?;
        let three = u16::from_str_radix(parts[2], 16).ok()?;
        let four_1 = u16::from_str_radix(parts[3], 16).ok()?;
        let four_2 = u64::from_str_radix(parts[4], 16).ok()?;

        let four = ((four_1 as u64) << 48) | four_2;

        Some(Guid { one, two, three, four })
    }

    fn add_default_guids(&mut self) {
        let list = vec![
            ("EBD0A0A2-B9E5-4433-87C0-68B6B72699C7", 5, "Basic data partition"),
            ("C12A7328-F81F-11D2-BA4B-00A0C93EC93B", 1, "EFI System Partition"),
            ("0FC63DAF-8483-4772-8E79-3D69D8477DE4", 2, "Linux filesystem"),
            ("0657FD6D-A4AB-43C4-84E5-0933C84B4F4F", 3, "Linux swap"),
            ("E3C9E316-0B5C-4DB8-817D-F92DF00215AE", 4, "Microsoft Reserved Partition"),
            ("DE94BBA4-06D1-4D40-A16A-BFD50179D6AC", 6, "Windows Recovery Environment")
        ];

        for (guid_str, uid, desc) in list {
            if let Some(guid) = Self::from_string(guid_str) {
                self.add(guid, uid, desc.into());
            }
        }
    }
}
