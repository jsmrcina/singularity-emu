use crate::traits::{ReadWrite, Clockable, Resettable};

struct SoundChannel
{
    enable: bool,
    sample: f64
}

pub struct Apu2a03
{
    pulse_1: SoundChannel
}

impl Apu2a03
{
    pub fn new() -> Self
    {
        Apu2a03
        {
            pulse_1: SoundChannel { enable: false, sample: 0.0 }
        }
    }

    pub fn get_output_sample(&mut self) -> f64
    {
        return self.pulse_1.sample;
    }
}

impl ReadWrite for Apu2a03
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        match address
        {
            0x4000 =>
            {
                true
            },
            0x4001 =>
            {
                true
            },
            0x4002 =>
            {
                true
            },
            0x4003 =>
            {
                true
            },
            0x4004 =>
            {
                true
            },
            0x4005 =>
            {
                true
            },
            0x4006 =>
            {
                true
            },
            0x4007 =>
            {
                true
            },
            0x4008 =>
            {
                true
            },
            0x4009 =>
            {
                true
            },
            0x400A =>
            {
                true
            },
            0x400B =>
            {
                true
            },
            0x400C =>
            {
                true
            },
            0x400D =>
            {
                true
            },
            0x400E =>
            {
                true
            },
            0x400F =>
            {
                true
            },
            0x4010 =>
            {
                true
            },
            0x4011 =>
            {
                true
            },
            0x4012 =>
            {
                true
            },
            0x4013 =>
            {
                true
            },
            0x4015 =>
            {
                true
            },
            0x4017 =>
            {
                true
            },
            _ =>
            {
                panic!("Invalid address written to inside APU: {}", address)
            }
        }
    }

    fn cpu_read(&mut self, _: u16, _: &mut u8) -> bool
    {
        false
    }

    fn ppu_write(&mut self, address: u16, _: u8) -> bool
    {
        panic!("PPU cannot write to APU: {}", address)
    }

    fn ppu_read(&self, address: u16, _: &mut u8) -> bool
    {
        panic!("PPU cannot read from APU: {}", address)
    }
}

impl Clockable for Apu2a03
{
    fn clock_tick(&mut self) -> bool
    {
        return false;
    }
}

impl Resettable for Apu2a03
{
    fn reset(&mut self)
    {
    }
}