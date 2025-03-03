use cpu::Cpu;
use memory::Memory;
use mmu::Mmu;
use os::Os;

mod cpu;
mod memory;
mod mmu;
mod os;

fn main() {
    let os = Os::new();
    let cpu = Cpu::new(1);
    let memory = Memory::new();

    let base = os.get_base(&memory);
    let mmu = Mmu::new(base);

    let logical_address = cpu.create_address();
    let physical_address = mmu.convert(logical_address);

    cpu.access(physical_address);
}
