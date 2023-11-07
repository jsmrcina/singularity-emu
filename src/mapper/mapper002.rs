use crate::traits::MapperTrait;

pub struct Mapper002
{
    prg_bank_selection_lo: u8,
    prg_bank_selection_hi: u8,
    prg_banks: u16,
    chr_banks: u16
}

impl Mapper002
{
    pub fn new(prg_banks: u16, chr_banks: u16) -> Self
    {
        Mapper002 {
            prg_bank_selection_lo: 0,
            prg_bank_selection_hi: 0,
            prg_banks,
            chr_banks
        }
    }
}

impl MapperTrait for Mapper002
{
    fn cpu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool
    {
        if (0x8000..=0xBFFF).contains(&address)
        {
            *mapped_addr = self.prg_bank_selection_lo as u32 * 0x4000 + (address as u32 & 0x3FFF);
            return true;
        }

        if (0xC000..=0xFFFF).contains(&address)
        {
            *mapped_addr = self.prg_bank_selection_hi as u32 * 0x4000 + (address as u32 & 0x3FFF);
            return true;
        }

        false
    }

    fn cpu_map_write(&mut self, address: u16, _: &mut u32, data: u8) -> bool
    {
        if (0x8000..=0xFFFF).contains(&address)
        {
            self.prg_bank_selection_lo = data & 0x0F;
            return true;
        } 

        false
    }

    fn ppu_map_read(&self, address: u16, mapped_addr: &mut u32) -> bool
    {
        if address < 0x2000
        {
            *mapped_addr = address as u32;
            true
        }
        else
        {
            false
        }
    }

    fn ppu_map_write(&mut self, address: u16, mapped_addr: &mut u32, _: u8) -> bool
    {
        if address < 0x2000 && self.chr_banks == 0
        {
            *mapped_addr = address as u32;
            return true;
        }

        false
    }

    fn reset(&mut self)
    {
        self.prg_bank_selection_lo = 0;
        self.prg_bank_selection_hi = (self.prg_banks - 1) as u8;
    }
}