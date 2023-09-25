use std::collections::HashMap;
use crate::traits::ReadWrite;

pub struct System<'a>
{
    _name: String,
    sys: &'a mut dyn ReadWrite
}

pub struct MainBus<'a>
{
    system_address_ranges: HashMap<(u16, u16), System<'a>>
}

impl<'a> MainBus<'a>
{
    pub fn new() -> Self
    {
        MainBus {
            system_address_ranges: HashMap::new()
        }
    }

    pub fn add_system(&mut self, address_range: (u16, u16), name: String, sys: &'a mut dyn ReadWrite)
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
}

impl<'a> ReadWrite for MainBus<'a>
{
    fn write(&mut self, address: u16, data: u8)
    {
        let mut iter = self.system_address_ranges.iter_mut();
        let result = iter.find(|x| x.0.0 <= address && x.0.1 >= address);

        match result
        {
            Some(x) => x.1.sys.write(address, data),
            None => panic!("Failed to find a system which maps this address range"),
        }
    }

    fn read(&self, address: u16) -> u8
    {
        let mut iter = self.system_address_ranges.iter();
        let result = iter.find(|x| x.0.0 <= address && x.0.1 >= address);

        match result
        {
            Some(x) => x.1.sys.read(address),
            None => panic!("Failed to find a system which maps this address range"),
        }
    }
}