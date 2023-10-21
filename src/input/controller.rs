use crate::traits::ReadWrite;

#[repr(u8)]
pub enum NesKey
{
    A = 0x80,
    B = 0x40,
    START = 0x20,
    SELECT = 0x10,
    UP = 0x08,
    DOWN = 0x04,
    LEFT = 0x02,
    RIGHT = 0x01
}

pub struct NesController
{
    snapshot_state: u8,
    live_state: u8
}

impl NesController
{
    pub fn new() -> Self
    {
        NesController
        {
            snapshot_state: 0,
            live_state: 0
        }
    }

    pub fn snapshot(&mut self)
    {
        println!("{:#b}", self.live_state);
        self.snapshot_state = self.live_state;
    }

    pub fn set_live_state_bit(&mut self, key: NesKey)
    {
        self.live_state |= key as u8;
    }

    pub fn clear_live_state(&mut self)
    {
        self.live_state = 0;
    }

    pub fn read_snapshot_bit(&mut self) -> bool
    {
        let bit: bool = (self.snapshot_state & 0x80) > 0;
        self.snapshot_state <<= 1;
        return bit;
    }
}

impl ReadWrite for NesController
{
    fn cpu_write(&mut self, address: u16, _: u8) -> bool
    {
        if address != 0x4016 && address != 0x4017
        {
            panic!("Invalid write to controller mapped to incorrect address");
        }

        self.snapshot();
        return true;
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        if address != 0x4016 && address != 0x4017
        {
            panic!("Invalid write to controller mapped to incorrect address");
        }

        *data = self.read_snapshot_bit() as u8;
        return true;
    }

    fn ppu_write(&mut self, _: u16, _: u8) -> bool
    {
        panic!("PPU cannot write to NES controller")
    }

    fn ppu_read(&self, _: u16, _: &mut u8) -> bool
    {
        panic!("PPU cannot read from NES controller")
    }
}