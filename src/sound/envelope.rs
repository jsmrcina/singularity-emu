use crate::traits::Clockable;

pub struct Envelope
{
    start: bool,
    disable: bool,
    divider_count: u16,
    volume: u16,
    output: u16,
    decay_count: u16,
    is_looped: bool
}

impl Envelope
{
    pub fn new() -> Self
    {
        Envelope
        {
            start: false,
            disable: false,
            divider_count: 0,
            volume: 0,
            output: 0,
            decay_count: 0,
            is_looped: false
        }
    }

    pub fn get_start(&self) -> bool {
        self.start
    }

    pub fn set_start(&mut self, start: bool) {
        self.start = start;
    }

    pub fn get_disable(&self) -> bool {
        self.disable
    }

    pub fn divider_count(&self) -> u16 {
        self.divider_count
    }

    pub fn get_volume(&self) -> u16 {
        self.volume
    }

    pub fn get_output(&self) -> u16 {
        self.output
    }

    pub fn get_decay_count(&self) -> u16 {
        self.decay_count
    }

    pub fn set_disable(&mut self, disable: bool) {
        self.disable = disable;
    }

    pub fn set_divider_count(&mut self, divider_count: u16) {
        self.divider_count = divider_count;
    }

    pub fn set_volume(&mut self, volume: u16) {
        self.volume = volume;
    }

    pub fn set_output(&mut self, output: u16) {
        self.output = output;
    }

    pub fn set_decay_count(&mut self, decay_count: u16) {
        self.decay_count = decay_count;
    }

    pub fn is_looped(&self) -> bool
    {
        self.is_looped
    }

    pub fn set_looped(&mut self, is_looped: bool)
    {
        self.is_looped = is_looped;
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self::new()
    }
}

impl Clockable for Envelope
{
    fn clock_tick(&mut self) -> bool
    {
        if !self.start
        {
            if self.divider_count == 0
            {
                self.divider_count = self.volume;
                if self.decay_count == 0
                {
                    if self.is_looped
                    {
                        self.decay_count = 15;
                    }
                }
                else
                {
                    self.decay_count -= 1;
                }
            }
            else
            {
                self.divider_count -= 1;
            }
        }
        else
        {
            self.start = false;
            self.decay_count = 15;
            self.divider_count = self.volume; 
        }

        if self.disable
        {
            self.output = self.volume;
        }
        else
        {
            self.output = self.decay_count;
        }

        false
    }
}