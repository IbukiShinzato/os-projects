use rand::Rng;

pub struct Cpu {
    id: usize,
}

impl Cpu {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn create_address(&self) -> usize {
        let mut rng = rand::thread_rng();
        let logical: usize = rng.gen_range(0..1000);
        println!("Cpu{} create logical address: {}", self.id, logical);
        logical
    }

    pub fn access(&self, physical_address: usize) {
        println!(
            "Cpu{} access to physical memory in {}",
            self.id, physical_address
        );
    }
}
