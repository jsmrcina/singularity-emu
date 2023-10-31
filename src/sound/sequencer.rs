use crate::traits::Clockable;

pub struct Sequencer
{
    sequence: u32,
    timer: u16,
    reload: u16,
    output: u8,
    enable: bool,
    mode: bool,
    callback: Option<fn(s: &mut u32, mode: bool)>,
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
            mode: false,
            callback: None,
        }
    }

    pub fn set_callback(&mut self, callback: fn(s: &mut u32, mode: bool))
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

    pub fn get_timer(&self) -> u16
    {
        self.timer
    }

    pub fn set_timer(&mut self, timer: u16)
    {
        self.timer = timer;
    }

    pub fn get_enable(&self) -> bool
    {
        self.enable
    }

    pub fn set_enable(&mut self, enable: bool)
    {
        self.enable = enable;
    }

    pub fn get_mode(&self) -> bool
    {
        self.mode
    }

    pub fn set_mode(&mut self, mode: bool)
    {
        self.mode = mode;
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
                (self.callback.unwrap())(&mut self.sequence, self.mode);
                self.output = (self.sequence & 0x00000001) as u8;
            }
        }

        self.output > 0
    }
}