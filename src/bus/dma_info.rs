use crate::traits::ReadWrite;


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
        self.transfer
    }

    pub fn set_transfer_in_progress(&mut self, transfer: bool)
    {
        self.transfer = transfer;
    }

    pub fn is_sync_needed(&self) -> bool
    {
        self.sync
    }

    pub fn set_sync_needed(&mut self, sync: bool)
    {
        self.sync = sync;
    }

    pub fn get_data(&self) -> u8 { self.data }
    pub fn get_addr(&self) -> u8 { self.addr }
    pub fn get_page(&self) -> u8 { self.page }

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
        true
    }

    fn cpu_read(&mut self, _: u16, _: &mut u8) -> bool
    {
        false
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

impl Default for DmaInfo {
    fn default() -> Self {
        DmaInfo::new()
    }
}
