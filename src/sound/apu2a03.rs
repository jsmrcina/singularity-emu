use crate::traits::{ReadWrite, Clockable, Resettable};

use super::sequencer::Sequencer;

pub struct Apu2a03
{
    pulse_1: Sequencer,
    pulse_2: Sequencer,
    frame_clock_counter: u32,
    clock_counter: u32
}

impl Apu2a03
{
    pub fn new() -> Self
    {
        Apu2a03
        {
            pulse_1: Sequencer::new(),
            pulse_2: Sequencer::new(),
            frame_clock_counter: 0,
            clock_counter: 0
        }
    }

    // pub fn get_output_sample(&mut self) -> f64
    // {
        
    // }

    pub fn get_osc_data(&mut self) -> [(f64, f64); 4]
    {
        [(self.pulse_1.get_duty_cycle(), self.pulse_1.get_frequency()),
            (self.pulse_2.get_duty_cycle(), self.pulse_2.get_frequency()),
            (0.0, 0.0),
            (0.0, 0.0)]
    }
}

impl Default for Apu2a03 {
    fn default() -> Self {
        Self::new()
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
                match (data & 0xC0) >> 6
                {
                    0x00 =>
                    {
                        self.pulse_1.set_sequence(0b00000001);
                        self.pulse_1.set_duty_cycle(0.125);
                    },
                    0x01 =>
                    {
                        self.pulse_1.set_sequence(0b00000011);
                        self.pulse_1.set_duty_cycle(0.250);
                    },
                    0x02 =>
                    {
                        self.pulse_1.set_sequence(0b00001111);
                        self.pulse_1.set_duty_cycle(0.500);
                    },
                    0x03 =>
                    {
                        self.pulse_1.set_sequence(0b11111100);
                        self.pulse_1.set_duty_cycle(0.750);
                    },
                    _ => panic!("Impossible")
                }
                true
            },
            0x4001 =>
            {
                true
            },
            0x4002 =>
            {
                self.pulse_1.set_reload(self.pulse_1.get_reload() & 0xFF00 | data as u16);
                true
            },
            0x4003 =>
            {
                self.pulse_1.set_reload(((data as u16 & 0x07) << 8) | self.pulse_1.get_reload() & 0x00FF);
                self.pulse_1.set_timer(self.pulse_1.get_reload());
                true
            },
            0x4004 =>
            {
                match (data & 0xC0) >> 6
                {
                    0x00 =>
                    {
                        self.pulse_2.set_sequence(0b00000001);
                        self.pulse_2.set_duty_cycle(0.125);
                    },
                    0x01 =>
                    {
                        self.pulse_2.set_sequence(0b00000011);
                        self.pulse_2.set_duty_cycle(0.250);
                    },
                    0x02 =>
                    {
                        self.pulse_2.set_sequence(0b00001111);
                        self.pulse_2.set_duty_cycle(0.500);
                    },
                    0x03 =>
                    {
                        self.pulse_2.set_sequence(0b11111100);
                        self.pulse_2.set_duty_cycle(0.750);
                    },
                    _ => panic!("Impossible")
                }
                true
            },
            0x4005 =>
            {
                true
            },
            0x4006 =>
            {
                self.pulse_2.set_reload(self.pulse_2.get_reload() & 0xFF00 | data as u16);
                true
            },
            0x4007 =>
            {
                self.pulse_2.set_reload(((data as u16 & 0x07) << 8) | self.pulse_2.get_reload() & 0x00FF);
                self.pulse_2.set_timer(self.pulse_2.get_reload());
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
                self.pulse_1.set_enable(data & 0x01 == 0x01);
                self.pulse_2.set_enable(data & 0x02 == 0x02);
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
        let mut quarter_frame_clock: bool = false;
        let mut half_frame_clock: bool = false;

        if self.clock_counter % 6 == 0
        {
            self.frame_clock_counter += 1;

            // 4-Step sequence mode
            if self.frame_clock_counter == 3729
            {
                quarter_frame_clock = true;
            }

            if self.frame_clock_counter == 7457
            {
                quarter_frame_clock = true;
                half_frame_clock = true;
            }

            if self.frame_clock_counter == 11186
            {
                quarter_frame_clock = true;
            }

            if self.frame_clock_counter == 14916
            {
                quarter_frame_clock = true;
                half_frame_clock = true;
                self.frame_clock_counter = 0;
            }

            // Quarter frame beats adjust volume envelope
            if quarter_frame_clock
            {
                // TODO
            }

            // Half frame beats adjust the note length and frequency sweepers
            if half_frame_clock
            {
                // TODO
            }

            self.pulse_1.set_frequency(1789773.0 / (16.0 * (self.pulse_1.get_reload() as f64 + 1.0)));
            self.pulse_2.set_frequency(1789773.0 / (16.0 * (self.pulse_2.get_reload() as f64 + 1.0)));
        }

        self.clock_counter += 1;

        false
    }
}

impl Resettable for Apu2a03
{
    fn reset(&mut self)
    {
    }
}