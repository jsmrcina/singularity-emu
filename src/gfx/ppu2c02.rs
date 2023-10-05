use std::{cell::RefCell, rc::Rc};

use crate::{traits::ReadWrite, cartridge::cart::Cart};

pub struct Ppu2c02
{
    cartridge: Option<Rc<RefCell<Cart>>>,
    nametables: [[u8; 1024]; 2],
    palettes: [u8; 32]
}

impl Ppu2c02
{
    pub fn new() -> Self
    {
        let s = Ppu2c02
        {
            cartridge: None,
            nametables: [[0; 1024]; 2],
            palettes: [0; 32]
        };

        return s;
    }

    pub fn connect_cartridge(&mut self, cartridge: Rc<RefCell<Cart>>)
    {
        self.cartridge = Some(cartridge);
    }
}

impl ReadWrite for Ppu2c02
{
    fn cpu_write(&mut self, address: u16, data: u8)
    {
        match address
        {
            // Control
            0x0000 => return,
            // Mask
            0x0001 => return,
            // Status
            0x0002 => return,
            // OAM Status
            0x0003 => return,
            // OAM Data
            0x0004 => return,
            // Scroll
            0x0005 => return,
            // PPU Address
            0x0006 => return,
            // PPU Data
            0x0007 => return,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn cpu_read(&self, address: u16) -> u8
    {
        let data: u8 = 0x00;

        match address
        {
            // Control
            0x0000 => return 0,
            // Mask
            0x0001 => return 0,
            // Status
            0x0002 => return 0,
            // OAM Status
            0x0003 => return 0,
            // OAM Data
            0x0004 => return 0,
            // Scroll
            0x0005 => return 0,
            // PPU Address
            0x0006 => return 0,
            // PPU Data
            0x0007 => return 0,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn ppu_write(&mut self, address: u16, data: u8)
    {
        let mut_addr = address & 0x3FFF;
    }

    fn ppu_read(&self, address: u16) -> u8
    {
        let data: u8 = 0x00;
        let mut_addr = address & 0x3FFF;
        return data;
    }
}