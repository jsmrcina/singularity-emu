use crate::traits::MapperTrait;

pub struct Mapper000
{
    prg_banks: u8,
    _chr_banks: u8
}

impl Mapper000
{
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self
    {
        Mapper000 {
            prg_banks,
            _chr_banks: chr_banks
        }
    }
}

impl MapperTrait for Mapper000
{
    fn cpu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool
    {
        if address >= 0x8000
        {
            let mask: u16 = if self.prg_banks == 1
                {
                    0x3FFF
                }
                else
                {
                    0x7FFF
                };

            *mapped_addr = address as u32 & mask as u32;
            return true;
        }

        false
    }

    fn cpu_map_write(&self, address: u16, mapped_addr: &mut u32) -> bool
    {
        if address >= 0x8000
        {
            let mask: u16 = if self.prg_banks == 1
            {
                0x3FFF
            }
            else
            {
                0x7FFF
            };

            *mapped_addr = address as u32 & mask as u32;
            return true;
        }

        false
    }

    fn ppu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool
    {
        if address <= 0x1FFF
        {
            *mapped_addr = address as u32;
            return true;
        }

        false
    }

    fn ppu_map_write(&self, _: u16, _: &mut u32) -> bool
    {
        false
    }
}