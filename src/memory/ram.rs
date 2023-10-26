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
            // Our vector will never grow past mirror size + 1
            buffer: vec![0; (mirror_size + 1) as usize],
            size,
            mirror_size
        }
    }
}

impl ReadWrite for Ram
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        if address > self.size
        {
            panic!("Wrote to RAM outside of addressable range")
        }

        // If the RAM is mirrored, mask the address down to the mirrored size
        let mirror_address = address & self.mirror_size;

        self.buffer[mirror_address as usize] = data;
        true
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        if address > self.size
        {
            panic!("Read from RAM outside of addressable range {:?} {:?}", address, self.size)
        }

        // If the RAM is mirrored, mask the address down to the mirrored size
        let mirror_address = address & self.mirror_size;

        *data = self.buffer[mirror_address as usize];
        true
    }

    fn ppu_write(&mut self, _: u16, _: u8) -> bool
    {
        panic!("CPU RAM canot be written to by PPU");
    }

    fn ppu_read(&self, _: u16, _: &mut u8) -> bool
    {
        panic!("CPU RAM canot be read from PPU");
    }
}