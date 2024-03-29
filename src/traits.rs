pub trait ReadWrite
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool;
    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool;
    fn ppu_write(&mut self, address: u16, data: u8) -> bool;
    fn ppu_read(&self, address: u16, data: &mut u8) -> bool;
}

pub trait MapperTrait
{
    fn cpu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool;
    fn cpu_map_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool;
    fn ppu_map_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool;
    fn reset(&mut self);
}

pub trait Clockable
{
    fn clock_tick(&mut self) -> bool;
}

pub trait Resettable
{
    fn reset(&mut self);
}