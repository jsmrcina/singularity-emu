use crate::traits::Clockable;

pub struct SoundLengthCounter
{
    counter: u8,
    enable: bool,
    halt: bool,
    result: u8
}

pub const LENGTH_TABLE: [u8; 32] = [  10, 254, 20,  2, 40,  4, 80,  6,
    160,   8, 60, 10, 14, 12, 26, 14,
     12,  16, 24, 18, 48, 20, 96, 22,
    192,  24, 72, 26, 16, 28, 32, 30 ];

impl SoundLengthCounter
{
    pub fn new() -> Self
    {
        SoundLengthCounter { counter: 0, enable: false, halt: false, result: 0 }
    }

    pub fn set_enable(&mut self, enable: bool)
    {
        self.enable = enable;
    }

    pub fn set_halt(&mut self, halt: bool)
    {
        self.halt = halt;
    }

    pub fn get_result(&self) -> u8
    {
        self.result
    }

    pub fn set_counter(&mut self, counter: u8)
    {
        self.counter = counter;
    }

    pub fn get_counter(&self) -> u8
    {
        self.counter
    }
}

impl Default for SoundLengthCounter
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Clockable for SoundLengthCounter
{
    fn clock_tick(&mut self) -> bool
    {
        if !self.enable
        {
            self.counter = 0;
        }
        else if self.counter > 0 && !self.halt
        {
            self.counter -= 1;
        }

        self.result = self.counter;
        true
    }
}