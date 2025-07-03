use crate::util::types::{GenPart, PartitionFlags};

#[derive(Debug, Clone, PartialEq)]
pub enum PendingType {
    Create,
    Delete,
    ModifyFlags,
    Wipe,
    DeleteAll,
}

#[derive(Debug, Clone)]
pub struct PendingAction {
    pub action_type: PendingType,
    pub partition: GenPart,
    pub nested_partitions: Vec<GenPart>,
    pub flags: Option<PartitionFlags>,
    pub wipe: bool,
}

pub struct PendingActionManager {
    actions: Vec<PendingAction>,
}

impl PendingActionManager {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn add(&mut self, action: PendingAction) {
        self.actions.push(action);
    }

    pub fn create_partition(&mut self, part: GenPart) {
        self.add(PendingAction {
            action_type: PendingType::Create,
            partition: part,
            nested_partitions: vec![],
            flags: None,
            wipe: false,
        });
    }

    pub fn delete_partition(&mut self, part: GenPart, nested: Vec<GenPart>) {
        self.add(PendingAction {
            action_type: PendingType::Delete,
            partition: part,
            nested_partitions: nested,
            flags: None,
            wipe: false,
        });
    }

    pub fn wipe_partition(&mut self, part: GenPart) {
        self.add(PendingAction {
            action_type: PendingType::Wipe,
            partition: part,
            nested_partitions: vec![],
            flags: None,
            wipe: true,
        });
    }

    pub fn modify_flags(&mut self, part: GenPart, flags: PartitionFlags) {
        self.add(PendingAction {
            action_type: PendingType::ModifyFlags,
            partition: part,
            nested_partitions: vec![],
            flags: Some(flags),
            wipe: false,
        });
    }

    pub fn delete_all(&mut self) {
        self.actions.clear();
    }

    pub fn undo(&mut self) {
        self.actions.pop();
    }

    pub fn count(&self) -> usize {
        self.actions.len()
    }

    pub fn get(&self, index: usize) -> Option<&PendingAction> {
        self.actions.get(index)
    }

    pub fn all(&self) -> &[PendingAction] {
        &self.actions
    }
}
