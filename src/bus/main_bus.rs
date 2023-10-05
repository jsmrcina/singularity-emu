use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;
use crate::traits::ReadWrite;

use crate::memory::ram::Ram;
use crate::cpu::cpu6502::Cpu6502;
use crate::gfx::ppu2c02::Ppu2c02;
use crate::cartridge::cart::{Cart, self};

pub struct System
{
    _name: String,
    sys: Option<Rc<RefCell<dyn ReadWrite>>>
}

pub struct MainBus
{
    system_address_ranges: HashMap<(u16, u16), System>,
    cpu_ram: Rc<RefCell<Ram>>,
    ppu_ram: Rc<RefCell<Ram>>,
    ppu: Rc<RefCell<Ppu2c02>>,
    cpu: Rc<RefCell<Cpu6502>>,
    cartridge: Option<Rc<RefCell<Cart>>>,
    system_clock_counter: u32
}

impl MainBus
{
    pub fn new() -> Self
    {
        let mut s = MainBus
        {
            system_address_ranges: HashMap::new(),
            cpu_ram: Rc::new(RefCell::new(Ram::new(0x1FFF, 0x07FF))),
            ppu_ram: Rc::new(RefCell::new(Ram::new(0x1FFF, 0x8))),
            cpu: Rc::new(RefCell::new(Cpu6502::new())),
            ppu: Rc::new(RefCell::new(Ppu2c02::new())),
            cartridge: None,
            system_clock_counter: 0
        };

        let cpu_ram_trait_object = Rc::clone(&s.cpu_ram) as Rc<RefCell<dyn ReadWrite>>;
        s.add_system((0, 0x1FFF), "CPU_RAM".to_string(), Some(cpu_ram_trait_object));

        let ppu_ram_trait_object = Rc::clone(&s.ppu_ram) as Rc<RefCell<dyn ReadWrite>>;
        s.add_system((0x2000, 0x3FFF), "PPU_RAM".to_string(), Some(ppu_ram_trait_object));

        return s;
    }

    pub fn add_system(&mut self, address_range: (u16, u16), name: String, sys: Option<Rc<RefCell<dyn ReadWrite>>>)
    {
        let s: System = System {
            _name: name,
            sys: sys
        };

        if address_range.0 > address_range.1
        {
            panic!("Address range low end is greater than high end")
        }

        // TODO: Validate that new address range does not conflict with existing ones

        self.system_address_ranges.insert(address_range, s);
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cart>>)
    {
        self.cartridge = Some(Rc::clone(&cartridge));
        self.ppu.borrow_mut().connect_cartridge(Rc::clone(&cartridge));
    }

    pub fn reset(&self)
    {

    }

    pub fn clock_tick(&self)
    {

    }

    pub fn get_cpu(&mut self) -> Rc<RefCell<Cpu6502>>
    {
        return Rc::clone(&self.cpu);
    }
}

impl ReadWrite for MainBus
{
    fn cpu_write(&mut self, address: u16, data: u8)
    {
        let mut iter = self.system_address_ranges.iter_mut();
        let result = iter.find(|x| x.0.0 <= address && x.0.1 >= address);

        match result
        {
            Some(x) =>
                match &x.1.sys
                {
                    Some(sys) => sys.borrow_mut().cpu_write(address, data),
                    None => panic!("System not initialized")
                }
            None => panic!("Failed to find a system which maps this address range"),
        }
    }

    fn cpu_read(&self, address: u16) -> u8
    {
        let mut iter = self.system_address_ranges.iter();
        let result = iter.find(|x| x.0.0 <= address && x.0.1 >= address);

        match result
        {
            Some(x) =>
                match &x.1.sys
                    {
                        Some(sys) => sys.borrow_mut().cpu_read(address),
                        None => panic!("System not initialized")
                    }
            None => panic!("Failed to find a system which maps this address range"),
        }
    }

    fn ppu_write(&mut self, address: u16, data: u8)
    {
        panic!("Main bus cannot be written to by PPU");
    }

    fn ppu_read(&self, address: u16) -> u8
    {
        panic!("Main bus cannot be read by PPU");   
    }
}