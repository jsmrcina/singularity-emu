use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use ggez::Context;

use crate::traits::{ReadWrite, Clockable};

use crate::memory::ram::Ram;
use crate::cpu::cpu6502::Cpu6502;
use crate::gfx::ppu2c02::Ppu2c02;
use crate::cartridge::cart::Cart;

pub struct System
{
    _name: String,
    // We assume priority of a given range is unique (i.e. you cannot have two priority 0 items over a specific range)
    priority: u8,
    sys: Rc<RefCell<dyn ReadWrite>>
}

pub struct MainBus
{
    system_address_ranges: HashMap<(u16, u16), System>,
    cpu_ram: Rc<RefCell<Ram>>,
    ppu: Rc<RefCell<Ppu2c02>>,
    cpu: Rc<RefCell<Cpu6502>>,
    cartridge: Option<Rc<RefCell<Cart>>>,
    system_clock_counter: u32
}

impl MainBus
{
    pub fn new(ctx: &Context) -> Self
    {
        let mut s = MainBus
        {
            system_address_ranges: HashMap::new(),
            cpu_ram: Rc::new(RefCell::new(Ram::new(0x1FFF, 0x07FF))),
            cpu: Rc::new(RefCell::new(Cpu6502::new())),
            ppu: Rc::new(RefCell::new(Ppu2c02::new(ctx))),
            cartridge: None,
            system_clock_counter: 0
        };

        let cpu_ram_trait_object = Rc::clone(&s.cpu_ram) as Rc<RefCell<dyn ReadWrite>>;
        s.add_system((0, 0x1FFF), "CPU_RAM".to_string(), 1, cpu_ram_trait_object);

        let ppu_trait_object = Rc::clone(&s.ppu) as Rc<RefCell<dyn ReadWrite>>;
        s.add_system((0x2000, 0x3FFF), "PPU_RAM".to_string(), 1, ppu_trait_object);

        return s;
    }

    pub fn add_system(&mut self, address_range: (u16, u16), name: String, priority: u8, sys: Rc<RefCell<dyn ReadWrite>>)
    {
        let s: System = System {
            _name: name,
            priority: priority,
            sys: sys
        };

        if address_range.0 > address_range.1
        {
            panic!("Address range low end is greater than high end")
        }

        self.system_address_ranges.insert(address_range, s);
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cart>>)
    {
        self.cartridge = Some(Rc::clone(&cartridge));

        let cart_trait_object = Rc::clone(&cartridge) as Rc<RefCell<dyn ReadWrite>>;
        self.add_system((0x0, 0xFFFF), "CARTRIDGE".to_string(), 0, cart_trait_object);

        self.ppu.borrow_mut().connect_cartridge(Rc::clone(&cartridge));
    }

    pub fn reset(&self)
    {

    }

    pub fn get_cpu(&mut self) -> Rc<RefCell<Cpu6502>>
    {
        return Rc::clone(&self.cpu);
    }

    pub fn get_ppu(&mut self) -> Rc<RefCell<Ppu2c02>>
    {
        return Rc::clone(&self.ppu);
    }
}

impl ReadWrite for MainBus
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut matching_systems: Vec<_> = self.system_address_ranges.iter_mut()
            .filter(|x| x.0.0 <= address && x.0.1 >= address)
            .collect();

        if matching_systems.is_empty()
        {
            panic!("Failed to find a system which maps this address range");
        }

        // Sort by priority, let the lowest priority handle the operation
        matching_systems.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));

        let mut handled: bool = false;
        for system in matching_systems
        {
            handled = system.1.sys.borrow_mut().cpu_write(address, data);
            if handled
            {
                break;
            }
        }

        if !handled
        {
            panic!("Issued a write and no system handled it");
        }

        // If not handled, we already panicked
        return true;
    }

    fn cpu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mut matching_systems: Vec<_> = self.system_address_ranges.iter()
            .filter(|x| x.0.0 <= address && x.0.1 >= address)
            .collect();

        if matching_systems.is_empty()
        {
            panic!("Failed to find a system which maps this address range");
        }

        // Sort by priority, let the lowest priority handle the operation
        matching_systems.sort_by(|a, b| a.1.priority.cmp(&b.1.priority));
    
        let mut handled: bool = false;
        for system in matching_systems
        {
            handled = system.1.sys.borrow_mut().cpu_read(address, data);
            if handled
            {
                break;
            }
        }

        return handled;
    }

    fn ppu_write(&mut self, _: u16, _: u8) -> bool
    {
        panic!("Main bus cannot be written to by PPU");
    }

    fn ppu_read(&self, _: u16, _: &mut u8) -> bool
    {
        panic!("Main bus cannot be read by PPU");   
    }
}

impl Clockable for MainBus
{
    fn clock_tick(&mut self)
    {
        self.ppu.borrow_mut().clock_tick();
        if self.system_clock_counter % 3 == 0
        {
            self.cpu.borrow_mut().clock_tick();
        }

        self.system_clock_counter += 1;
    }
}