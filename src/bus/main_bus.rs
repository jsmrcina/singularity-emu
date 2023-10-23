use std::rc::Rc;
use std::cell::RefCell;

use crate::traits::ReadWrite;

use crate::memory::ram::Ram;
use crate::cpu::cpu6502::Cpu6502;
use crate::gfx::ppu2c02::Ppu2c02;
use crate::cartridge::cart::Cart;

use crate::bus::bus_systems::BusSystems;

use crate::input::controller::NesController;


pub struct DmaInfo
{
    page: u8,
    addr: u8,
    data: u8,
    transfer: bool,
    sync: bool
}

impl DmaInfo
{
    pub fn new() -> Self
    {
        DmaInfo { page: 0, addr: 0, data: 0, transfer: false, sync: true }
    }

    pub fn is_transfer_in_progress(&self) -> bool
    {
        return self.transfer;
    }

    pub fn set_transfer_in_progress(&mut self, transfer: bool)
    {
        self.transfer = transfer;
    }

    pub fn is_sync_needed(&self) -> bool
    {
        return self.sync;
    }

    pub fn set_sync_needed(&mut self, sync: bool)
    {
        self.sync = sync;
    }

    pub fn get_data(&self) -> u8 { return self.data; }
    pub fn get_addr(&self) -> u8 { return self.addr; }
    pub fn get_page(&self) -> u8 { return self.page; }

    pub fn set_data(&mut self, data: u8) { self.data = data; }
    pub fn set_addr(&mut self, addr: u8) { self.addr = addr; }
    pub fn set_page(&mut self, page: u8) { self.page = page; }
}

impl ReadWrite for DmaInfo
{
    fn cpu_write(&mut self, _: u16, data: u8) -> bool
    {
        self.set_page(data);
        self.set_addr(0);
        self.set_transfer_in_progress(true);
        return true;
    }

    fn cpu_read(&mut self, _: u16, _: &mut u8) -> bool
    {
        return false;
    }

    fn ppu_write(&mut self, _: u16, _: u8) -> bool
    {
        panic!("Cannot PPU write to DmaInfo");
    }

    fn ppu_read(&self, _: u16, _: &mut u8) -> bool
    {
        panic!("Cannot PPU read from DmaInfo");
    }
}

pub struct MainBus
{
    bus_systems: BusSystems,
    cpu_ram: Rc<RefCell<Ram>>,
    ppu: Rc<RefCell<Ppu2c02>>,
    cpu: Rc<RefCell<Cpu6502>>,
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
            cpu: Rc::new(RefCell::new(Cpu6502::new())),
            ppu: Rc::new(RefCell::new(Ppu2c02::new())),
            cartridge: None,
            controllers: [ Rc::new(RefCell::new(NesController::new())), Rc::new(RefCell::new(NesController::new())) ],
            system_clock_counter: 0,
            dma_info: Rc::new(RefCell::new(DmaInfo::new())),
        };

        let cpu_ram_trait_object = Rc::clone(&s.cpu_ram) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0, 0x1FFF), "CPU_RAM".to_string(), 1, cpu_ram_trait_object);

        let ppu_trait_object = Rc::clone(&s.ppu) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x2000, 0x3FFF), "PPU_RAM".to_string(), 1, ppu_trait_object);

        let dma_trait_object = Rc::clone(&s.dma_info) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4014, 0x4014), "DMA".to_string(), 1, dma_trait_object);

        let controller_1_trait_object = Rc::clone(&s.controllers[0]) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4016, 0x4016), "CONTROLLER_1".to_string(), 1, controller_1_trait_object);

        let controller_2_trait_object = Rc::clone(&s.controllers[1]) as Rc<RefCell<dyn ReadWrite>>;
        s.bus_systems.add_system((0x4017, 0x4017), "CONTROLLER_2".to_string(), 1, controller_2_trait_object);

        return s;
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
        return Rc::clone(&self.cpu);
    }

    pub fn get_ppu(&mut self) -> Rc<RefCell<Ppu2c02>>
    {
        return Rc::clone(&self.ppu);
    }

    pub fn get_dma_info(&mut self) -> Rc<RefCell<DmaInfo>>
    {
        return Rc::clone(&self.dma_info);
    }

    pub fn get_clock_counter(&self) -> u32
    {
        return self.system_clock_counter;
    }

    pub fn get_controller(&mut self, index: usize) -> Rc<RefCell<NesController>>
    {
        return Rc::clone(&self.controllers[index]);
    }

    pub fn get_system_clock_counter(&self) -> u32
    {
        return self.system_clock_counter;
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

        let mut _handled: bool = false;
        for system in matching_systems
        {
            _handled = system.sys.borrow_mut().cpu_write(address, data);
            if _handled
            {
                break;
            }
        }

        // Add back once we have systems covering the whole address range
        // if !handled
        // {
        //     panic!("Issued a write and no system handled it");
        // }

        // If not handled, we already panicked
        return true;
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