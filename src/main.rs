use bus::main_bus::MainBus;
use memory::ram::Ram;

pub mod traits;
pub mod bus;
pub mod memory;

fn main()
{
    let mut mb = MainBus::new();

    let mut ram: Ram = Ram::new();

    mb.add_system((0x0, 0xFFFF), String::from("MEMORY"), &mut ram);
    mb.write(0x0, 0xff);
}
