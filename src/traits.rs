pub trait ReadWrite
{
    fn cpu_write(&mut self, address: u16, data: u8);
    fn cpu_read(&self, address: u16) -> u8;
    fn ppu_write(&mut self, address: u16, data: u8);
    fn ppu_read(&self, address: u16) -> u8;
}