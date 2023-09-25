use bus::main_bus::MainBus;
use memory::ram::Ram;
use cpu::cpu6502::CPU6502;
use traits::ReadWrite;

pub mod traits;
pub mod bus;
pub mod memory;
pub mod cpu;

fn main()
{
    let mut mb = MainBus::new();

    let mut ram: Ram = Ram::new();
    let _cpu: CPU6502 = CPU6502::new(&mut ram);

    mb.add_system((0x0, 0xFFFF), String::from("MEMORY"), &mut ram);
    mb.write(0x0, 0xff);

    // TODO: Figure out lifetimes for RAM object, cannot take two mutable references
    // cpu.clock_tick();
}
