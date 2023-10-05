use crate::traits::ReadWrite;

pub struct Cart
{

}

impl Cart
{
    pub fn new() -> Self
    {
        let s = Cart {

        };

        return s;
    }
}

impl ReadWrite for Cart
{
    fn cpu_write(&mut self, address: u16, data: u8)
    {
        todo!()
    }

    fn cpu_read(&self, address: u16) -> u8
    {
        todo!()
    }

    fn ppu_write(&mut self, address: u16, data: u8) {
        todo!()
    }

    fn ppu_read(&self, address: u16) -> u8 {
        todo!()
    }
}