use crate::traits::Clockable;

pub struct Sweeper
{
    enabled: bool,
    down: bool,
    reload: bool,
    shift: u8,
    timer: u8,
    period: u8,
    change: u16,
    mute: bool,

    channel: bool,
    target: u16,
    changed_by_tick: bool
}

impl Sweeper
{
    pub fn new() -> Self
    {
        Sweeper
        {
            enabled: false,
            down: false,
            reload: false,
            shift: 0,
            timer: 0,
            period: 0,
            change: 0,
            mute: false,
            channel: false,
            target: 0,
            changed_by_tick: false
        }
    }


    pub fn get_enabled(&self) -> bool
    {
        self.enabled
    }

    pub fn get_down(&self) -> bool
    {
        self.down
    }

    pub fn get_reload(&self) -> bool
    {
        self.reload
    }

    pub fn get_shift(&self) -> u8
    {
        self.shift
    }

    pub fn get_timer(&self) -> u8
    {
        self.timer
    }

    pub fn get_period(&self) -> u8
    {
        self.period
    }

    pub fn get_change(&self) -> u16
    {
        self.change
    }

    pub fn get_mute(&self) -> bool
    {
        self.mute
    }

    pub fn get_channel(&self) -> bool
    {
        self.channel
    }

    pub fn get_target(&self) -> u16
    {
        self.target
    }

    pub fn changed_by_tick(&self) -> bool
    {
        self.changed_by_tick
    }

    pub fn set_enabled(&mut self, enabled: bool)
    {
        self.enabled = enabled;
    }

    pub fn set_down(&mut self, down: bool)
    {
        self.down = down;
    }

    pub fn set_reload(&mut self, reload: bool)
    {
        self.reload = reload;
    }

    pub fn set_shift(&mut self, shift: u8)
    {
        self.shift = shift;
    }

    pub fn set_timer(&mut self, timer: u8)
    {
        self.timer = timer;
    }

    pub fn set_period(&mut self, period: u8)
    {
        self.period = period;
    }

    pub fn set_change(&mut self, change: u16)
    {
        self.change = change;
    }

    pub fn set_mute(&mut self, mute: bool)
    {
        self.mute = mute;
    }

    pub fn set_channel(&mut self, channel: bool)
    {
        self.channel = channel;
    }

    pub fn set_target(&mut self, target: u16)
    {
        self.target = target;
    }

    pub fn set_changed_by_tick(&mut self, changed_by_tick: bool)
    {
        self.changed_by_tick = changed_by_tick;
    }

    pub fn track(&mut self)
    {
        if self.enabled
        {
            self.change = self.target >> self.shift;
            self.mute = self.target < 8 || self.target > 0x7FF;
        }
    }
}

impl Default for Sweeper
{
    fn default() -> Self
    {
        Self::new()
    }
}

impl Clockable for Sweeper
{
    fn clock_tick(&mut self) -> bool
    {
        if self.timer == 0 && self.enabled && self.shift > 0 && !self.mute
            && self.target >= 8 && self.change < 0x07FF
        {
            if self.down
            {
                self.target -= self.change - self.channel as u16;
            }
            else
            {
                self.target += self.change;
            }
            self.changed_by_tick = true;
        }

        {
            if self.timer == 0 || self.reload
            {
                self.timer = self.period;
                self.reload = false;
            }
            else
            {
                self.timer -= 1;
            }

            self.mute = self.target < 8 || self.target > 0x7FF;
        }

        true
    }
}