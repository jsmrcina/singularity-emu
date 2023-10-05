use crate::traits::ReadWrite;

pub struct Ram
{
    pub buffer: Vec<u8>,
    pub size: u16,
    pub mirror_size: u16
}

impl Ram
{
    pub fn new(size: u16, mirror_size: u16) -> Self
    {
        Ram
        {
            // Our vector will never grow past mirror size
            buffer: Vec::with_capacity(mirror_size as usize),
            size: size,
            mirror_size: mirror_size
        }
    }
}

impl ReadWrite for Ram
{
    fn cpu_write(&mut self, address: u16, data: u8)
    {
        if address > self.size
        {
            panic!("Wrote to RAM outside of addressable range")
        }

        // If the RAM is mirrored, mask the address down to the mirrored size
        let mirror_address = address & self.mirror_size;

        self.buffer[mirror_address as usize] = data;
    }

    fn cpu_read(&self, address: u16) -> u8
    {
        if address > self.size
        {
            panic!("Wrote to RAM outside of addressable range")
        }

        // If the RAM is mirrored, mask the address down to the mirrored size
        let mirror_address = address & self.mirror_size;

        return self.buffer[mirror_address as usize];
    }

    fn ppu_write(&mut self, address: u16, data: u8)
    {
        panic!("CPU RAM canot be written to by PPU");
    }

    fn ppu_read(&self, address: u16) -> u8
    {
        panic!("CPU RAM canot be read from PPU");
    }
}