use crate::traits::{ReadWrite, MapperTrait};

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use byteorder::ReadBytesExt;

use crate::mapper::mapper000::Mapper000;

struct InesHeader
{
    _name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper_1: u8,
    mapper_2: u8,
    _prg_ram_size: u8,
    _tv_system_1: u8,
    _tv_system_2: u8,
    _unused: [u8; 5]
}

#[derive(Debug, Clone, Copy)]
pub enum MirrorMode
{
    Horizontal,
    Vertical,
    OneScreenLo,
    OneScreenHi
}

impl InesHeader
{
    pub fn new<R: Read>(mut reader: R) -> io::Result<Self>
    {
        let mut _name = [0u8; 4];
        reader.read_exact(&mut _name)?;

        let prg_rom_chunks = reader.read_u8()?;
        let chr_rom_chunks = reader.read_u8()?;
        let mapper_1 = reader.read_u8()?;
        let mapper_2 = reader.read_u8()?;
        let _prg_ram_size = reader.read_u8()?;
        let _tv_system_1 = reader.read_u8()?;
        let _tv_system_2 = reader.read_u8()?;

        let mut _unused = [0u8; 5];
        reader.read_exact(&mut _unused)?;

        Ok(InesHeader {
            _name,
            prg_rom_chunks,
            chr_rom_chunks,
            mapper_1,
            mapper_2,
            _prg_ram_size,
            _tv_system_1,
            _tv_system_2,
            _unused,
        })
    }
}

pub struct Cart
{
    prg_memory: Vec<u8>,
    chr_memory: Vec<u8>,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    mirror_mode: MirrorMode,
    mapper: Option<Arc<Mutex<dyn MapperTrait>>>
}

impl Cart
{
    pub fn new(filename: String) -> io::Result<Self>
    {
        let mut file = File::open(filename)?;
        let header = InesHeader::new(&mut file)?;

        // Skip training information
        if header.mapper_1 & 0x04 == 0x04
        {
            file.seek(SeekFrom::Current(512))?;
        }

        let mut s = Cart
        {
            prg_memory: Vec::new(),
            chr_memory: Vec::new(),
            mapper_id: ((header.mapper_1 >> 4) << 4) | (header.mapper_2 >> 4),
            prg_banks: 0,
            chr_banks: 0,
            mapper: None,
            mirror_mode: MirrorMode::Horizontal
        };

        if header.mapper_1 & 0x01 == 0x01
        {
            s.mirror_mode = MirrorMode::Vertical;
        }

        let file_type: u8 = 1;

        if file_type == 1
        {
            s.prg_banks = header.prg_rom_chunks;
            s.prg_memory.resize(s.prg_banks as usize * 16384, 0);
            file.read_exact(&mut s.prg_memory)?;

            s.chr_banks = header.chr_rom_chunks;
            s.chr_memory.resize(s.chr_banks as usize * 8192, 0);
            file.read_exact(&mut s.chr_memory)?;
        }

        // TODO: file type 0 and 2

        s.mapper = match s.mapper_id
        {
            0 => Some(Arc::new(Mutex::new(Mapper000::new(s.prg_banks, s.chr_banks)))),
            _ => panic!("Invalid mapper type, not supported")
        };

        Ok(s)
    }

    pub fn get_mirror_mode(&self) -> MirrorMode
    {
        self.mirror_mode
    }

}

impl ReadWrite for Cart
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut mapped_addr: u32 = 0;
        let handled = match &self.mapper
        {
            Some(x) => x.lock().unwrap().cpu_map_write(address, &mut mapped_addr),
            None => panic!("No mapper set for cartridge")
        };

        if handled
        {
            self.prg_memory[mapped_addr as usize] = data;
        }

        handled
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        let mut mapped_addr: u32 = 0;
        let handled = match &self.mapper
        {
            Some(x) => x.lock().unwrap().cpu_map_read(address, &mut mapped_addr),
            None => panic!("No mapper set for cartridge")
        };

        if handled
        {
            *data = self.prg_memory[mapped_addr as usize];
        }

        handled
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut mapped_addr: u32 = 0;
        let handled = match &self.mapper
        {
            Some(x) => x.lock().unwrap().ppu_map_write(address, &mut mapped_addr),
            None => panic!("No mapper set for cartridge")
        };

        if handled
        {
            self.chr_memory[mapped_addr as usize] = data;
        }

        handled
    }

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mut mapped_addr: u32 = 0;
        let handled =match &self.mapper
        {
            Some(x) => x.lock().unwrap().ppu_map_read(address, &mut mapped_addr),
            None => panic!("No mapper set for cartridge")
        };

        if handled
        {
            *data = self.chr_memory[mapped_addr as usize];
        }
        
        handled
    }
}

unsafe impl Send for Cart {}