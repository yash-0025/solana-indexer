use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slot(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct SlotInfo {
    pub slot : Slot,
    pub parent_slot : Slot,
    pub block_height: Option<u64>,
}


impl SlotInfo {
    pub fn new(slot: u64, parent_slot: u64, block_height: Option<u64>) -> Self {
        Self {
            slot: Slot(slot),
            parent_slot: Slot(parent_slot),
            block_height,
        }
    }


    pub fn is_parent_consecutive(&self) -> bool {
         self.slot.0 == self.parent_slot.0 + 1
    }
}


impl fmt::Display for SlotInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let height_str = match self.block_height {
            Some(h) => h.to_string(),
            None => "none".to_string(),
        };

        write!(f, "Slot {} (parent: {}, height: {})", self.slot.0, self.parent_slot.0 , height_str)
    } 
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slot_info_creation_and_consecutive_check() {
    let consecutive_info = SlotInfo::new(105, 104, Some(90));
    assert_eq!(consecutive_info.slot, Slot(105));
    assert_eq!(consecutive_info.parent_slot, Slot(104));
    assert!(consecutive_info.is_parent_consecutive());
    
    
    let skipped_info = SlotInfo::new(110, 108, Some(94));
    assert!(!skipped_info.is_parent_consecutive());
    
    let display = format!("{}", consecutive_info);
    assert!(display.contains("Slot 105"));
    assert!(display.contains("parent: 104"));
    assert!(display.contains("height: 90"));

    }
}