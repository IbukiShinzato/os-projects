use rand::Rng;

#[derive(Debug)]
pub struct Memory {
    pub table: [usize; 10000],
}

impl Memory {
    pub fn new() -> Self {
        let mut table = [0; 10000];
        let mut rng = rand::thread_rng();
        let count: usize = rng.gen_range(0..100);

        for _ in 0..count {
            let address = rng.gen_range(1..10000);
            let data = rng.gen_range(1..1000);
            table[address] = data;
        }
        // 0 => None
        Self { table: table }
    }
}
