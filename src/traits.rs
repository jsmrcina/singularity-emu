pub trait ReadWrite
{
    fn write(&mut self, address: u16, data: u8);
    fn read(&self, address: u16) -> u8;
}