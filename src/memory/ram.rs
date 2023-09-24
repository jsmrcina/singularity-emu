use crate::traits::ReadWrite;

pub struct Ram
{
    buffer: [u8; 4096]
}

impl Ram
{
    pub fn new() -> Self
    {
        Ram
        {
            buffer: [0; 4096]
        }
    }
}

impl ReadWrite for Ram
{
    fn write(&mut self, address: u16, data: u8)
    {
        self.buffer[address as usize] = data;
    }

    fn read(&self, address: u16) -> u8
    {
        return self.buffer[address as usize];
    }
}