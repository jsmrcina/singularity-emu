use std::rc::Rc;
use std::cell::RefCell;

use std::collections::HashMap;

use crate::traits::ReadWrite;

pub struct System
{
    pub _name: String,
    // We assume priority of a given range is unique (i.e. you cannot have two priority 0 items over a specific range)
    pub priority: u8,
    pub sys: Rc<RefCell<dyn ReadWrite>>
}

pub struct BusSystems
{
    system_address_ranges: HashMap<(u16, u16), System>,
}

impl BusSystems
{
    pub fn new() -> Self
    {
        BusSystems
        { 
            system_address_ranges: HashMap::new()
        }
    }

    pub fn add_system(&mut self, address_range: (u16, u16), name: String, priority: u8, sys: Rc<RefCell<dyn ReadWrite>>)
    {
        let s: System = System {
            _name: name,
            priority,
            sys
        };

        if address_range.0 > address_range.1
        {
            panic!("Address range low end is greater than high end")
        }

        self.system_address_ranges.insert(address_range, s);
    }

    pub fn get_matching_systems(&mut self, address: u16) -> Vec<&mut System>
    {
        let mut matching_systems: Vec<_> = self.system_address_ranges.iter_mut()
            .filter(|x| x.0.0 <= address && x.0.1 >= address)
            .map(|(_, value)| value)
            .collect();

        if matching_systems.is_empty()
        {
            panic!("Failed to find a system which maps this address range");
        }

        // Sort by priority, let the lowest priority handle the operation
        matching_systems.sort_by(|a, b| a.priority.cmp(&b.priority));
        matching_systems
    }

}

impl Default for BusSystems {
    fn default() -> Self {
        BusSystems::new()
    }
}