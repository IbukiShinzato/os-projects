use crate::memory::Memory;

pub struct Os {}

impl Os {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_base(&self, memory: &Memory) -> usize {
        let table = memory.table;
        let mut base = 0;
        for (address, data) in table.iter().enumerate() {
            if *data != 0 {
                base = address;
                break;
            }
        }
        base
    }
}
