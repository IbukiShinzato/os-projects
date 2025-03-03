// Memory management unit
pub struct Mmu {
    // base value
    base: usize,
}

impl Mmu {
    pub fn new(base: usize) -> Self {
        Self { base: base }
    }

    pub fn convert(&self, logical_address: usize) -> usize {
        self.base + logical_address
    }
}
