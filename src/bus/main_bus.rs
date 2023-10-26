use std::rc::Rc;
use std::cell::RefCell;

use crate::sound::apu2a03::Apu2a03;
use crate::traits::ReadWrite;

use crate::memory::ram::Ram;
use crate::cpu::cpu6502::Cpu6502;
use crate::gfx::ppu2c02::Ppu2c02;
use crate::cartridge::cart::Cart;

use crate::bus::bus_systems::BusSystems;

use crate::input::controller::NesController;

use super::dma_info::DmaInfo;

pub struct MainBus
{
    bus_systems: BusSystems,
    cpu_ram: Rc<RefCell<Ram>>,
    ppu: Rc<RefCell<Ppu2c02>>,
    cpu: Rc<RefCell<Cpu6502>>,
    apu: Rc<RefCell<Apu2a03>>,
    cartridge: Option<Rc<RefCell<Cart>>>,
    controllers: [Rc<RefCell<NesController>>; 2],
    system_clock_counter: u32,
    dma_info: Rc<RefCell<DmaInfo>>,
}

impl MainBus
{
    pub fn new() -> Self
    {
        let mut s = MainBus
        {
            bus_systems: BusSystems::new(),
            cpu_ram: Rc::new(RefCell::new(Ram::new(0x1FFF, 0x07FF))),
            ppu: Rc::new(RefCell::new(Ppu2c02::new())),
            cpu: Rc::new(RefCell::new(Cpu6502::new())),
            apu: Rc::new(RefCell::new(Apu2a03::new())),
            cartridge: None,
            controllers: [ Rc::new(RefCell::new(NesController::new())), Rc::new(RefCell::new(NesController::new())) ],
            system_clock_counter: 0,
            dma_info: Rc::new(RefCell::new(DmaInfo::new())),
        };

        let cpu_ram_trait_object = Rc::clone(&s.cpu_ram) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0, 0x1FFF), "CPU_RAM".to_string(), 1, cpu_ram_trait_object);

        let ppu_trait_object = Rc::clone(&s.ppu) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x2000, 0x3FFF), "PPU_RAM".to_string(), 1, ppu_trait_object);

        // APU handles several ranges of addresses
        {
            let mut apu_trait_object = Rc::clone(&s.apu) as Rc<RefCell<dyn ReadWrite>>;
            s.bus_systems.add_system((0x4000, 0x4013), "APU_CHANNELS".to_string(), 1, apu_trait_object);

            apu_trait_object = Rc::clone(&s.apu) as Rc<RefCell<dyn ReadWrite>>;
            s.bus_systems.add_system((0x4015, 0x4015), "APU_ENABLE".to_string(), 1, apu_trait_object);

            apu_trait_object = Rc::clone(&s.apu) as Rc<RefCell<dyn ReadWrite>>;
            s.bus_systems.add_system((0x4017, 0x4017), "APU_FRAME".to_string(), 1, apu_trait_object);
        }

        let dma_trait_object = Rc::clone(&s.dma_info) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4014, 0x4014), "DMA".to_string(), 1, dma_trait_object);

        let controller_1_trait_object = Rc::clone(&s.controllers[0]) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4016, 0x4016), "CONTROLLER_1".to_string(), 1, controller_1_trait_object);

        let controller_2_trait_object = Rc::clone(&s.controllers[1]) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4017, 0x4017), "CONTROLLER_2".to_string(), 1, controller_2_trait_object);
        s
    }

    pub fn insert_cartridge(&mut self, cartridge: Rc<RefCell<Cart>>)
    {
        self.cartridge = Some(Rc::clone(&cartridge));

        let cart_trait_object = Rc::clone(&cartridge) as Rc<RefCell<dyn ReadWrite>>;
        self.bus_systems.add_system((0x0, 0xFFFF), "CARTRIDGE".to_string(), 0, cart_trait_object);

        self.ppu.borrow_mut().connect_cartridge(Rc::clone(&cartridge));
    }
    
    pub fn get_cpu(&mut self) -> Rc<RefCell<Cpu6502>>
    {
        Rc::clone(&self.cpu)
    }

    pub fn get_ppu(&mut self) -> Rc<RefCell<Ppu2c02>>
    {
        Rc::clone(&self.ppu)
    }

    pub fn get_apu(&mut self) -> Rc<RefCell<Apu2a03>>
    {
        Rc::clone(&self.apu)
    }

    pub fn get_dma_info(&mut self) -> Rc<RefCell<DmaInfo>>
    {
        Rc::clone(&self.dma_info)
    }

    pub fn is_dma_transfer_in_progress(&self) -> bool
    {
        self.dma_info.borrow().is_transfer_in_progress()
    }

    pub fn get_clock_counter(&self) -> u32
    {
        self.system_clock_counter
    }

    pub fn get_controller(&mut self, index: usize) -> Rc<RefCell<NesController>>
    {
        Rc::clone(&self.controllers[index])
    }

    pub fn get_system_clock_counter(&self) -> u32
    {
        self.system_clock_counter
    }

    pub fn increment_clock_counter(&mut self)
    {
        self.system_clock_counter +=  1;
    }

    pub fn reset(&mut self)
    {
        self.system_clock_counter = 0;
    }

}

impl ReadWrite for MainBus
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        let matching_systems = self.bus_systems.get_matching_systems(address);

        let mut handled: bool = false;
        for system in matching_systems
        {
            handled = system.sys.borrow_mut().cpu_write(address, data);
            if handled
            {
                break;
            }
        }

        // Add back once we have systems covering the whole address range
        if !handled
        {
            panic!("Issued a write and no system handled it");
        }

        // If not handled, we already panicked
        true
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        let matching_systems = self.bus_systems.get_matching_systems(address);
    
        let mut handled: bool = false;
        for system in matching_systems
        {
            handled = system.sys.borrow_mut().cpu_read(address, data);
            if handled
            {
                break;
            }
        }

        handled
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

impl Default for MainBus {
    fn default() -> Self {
        MainBus::new()
    }
}