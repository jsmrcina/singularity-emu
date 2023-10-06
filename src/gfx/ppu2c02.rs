use std::{cell::RefCell, rc::Rc};

use crate::{traits::{ReadWrite, Clockable}, cartridge::cart::Cart};

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

    pub fn frame_complete(&self) -> bool
    {
        return true;
    }

    pub fn set_frame_complete(&mut self, frame_complete: bool)
    {

    }
}

impl ReadWrite for Ppu2c02
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mirror_address = address & 0x7;

        match mirror_address
        {
            // Control
            0x0000 => return true,
            // Mask
            0x0001 => return true,
            // Status
            0x0002 => return true,
            // OAM Status
            0x0003 => return true,
            // OAM Data
            0x0004 => return true,
            // Scroll
            0x0005 => return true,
            // PPU Address
            0x0006 => return true,
            // PPU Data
            0x0007 => return true,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn cpu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mirror_address = address & 0x7;

        match mirror_address
        {
            // Control
            0x0000 => return true,
            // Mask
            0x0001 => return true,
            // Status
            0x0002 => return true,
            // OAM Status
            0x0003 => return true,
            // OAM Data
            0x0004 => return true,
            // Scroll
            0x0005 => return true,
            // PPU Address
            0x0006 => return true,
            // PPU Data
            0x0007 => return true,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow_mut().ppu_write(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        return handled;
    }

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow().ppu_read(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        return handled;
    }
}

impl Clockable for Ppu2c02
{
    fn clock_tick(&mut self)
    {
    }
}