use crate::traits::Clockable;

pub struct Sequencer
{
    sequence: u32,
    timer: u16,
    reload: u16,
    output: u8,
    enable: bool,
    callback: Option<fn(s: &mut u32)>,

    frequency: f64,
    duty_cycle: f64
}

impl Sequencer
{
    pub fn new() -> Self
    {
        Sequencer
        {
            sequence: 0,
            timer: 0,
            reload: 0,
            output: 0,
            enable: false,
            callback: None,

            frequency: 0.0,
            duty_cycle: 0.0
        }
    }

    pub fn set_callback(&mut self, callback: fn(s: &mut u32))
    {
        self.callback = Some(callback);
    }

    pub fn get_output(&self) -> u8
    {
        self.output
    }

    pub fn set_sequence(&mut self, sequence: u32)
    {
        self.sequence = sequence;
    }

    pub fn get_reload(&self) -> u16
    {
        self.reload
    }

    pub fn set_reload(&mut self, reload: u16)
    {
        self.reload = reload;
    }

    pub fn set_timer(&mut self, timer: u16)
    {
        self.timer = timer;
    }

    pub fn set_enable(&mut self, enable: bool)
    {
        self.enable = enable;
    }

    pub fn set_frequency(&mut self, frequency: f64)
    {
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> f64
    {
        self.frequency
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f64)
    {
        self.duty_cycle = duty_cycle;
    }

    pub fn get_duty_cycle(&self) -> f64
    {
        self.duty_cycle
    }
}

impl Default for Sequencer {
    fn default() -> Self {
        Self::new()
    }
}

impl Clockable for Sequencer
{
    fn clock_tick(&mut self) -> bool
    {
        if self.enable
        {
            self.timer = self.timer.wrapping_sub(1);
            if self.timer == 0xFFFF
            {
                self.timer = self.reload + 1;
                (self.callback.unwrap())(&mut self.sequence);
                self.output = (self.sequence & 0x00000001) as u8;
            }
        }

        self.output > 0
    }
}