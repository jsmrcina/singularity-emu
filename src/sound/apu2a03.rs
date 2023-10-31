use crate::traits::{ReadWrite, Clockable, Resettable};

use super::{sequencer::Sequencer, envelope::Envelope, oscillator::Oscillator, sound_length_counter::{SoundLengthCounter, self}, sweeper::Sweeper};

pub struct Apu2a03
{
    pulse_1_sample: f64,
    pulse_1_halt: bool,
    pulse_1_seq: Sequencer,
    pulse_1_osc: Oscillator,
    pulse_1_env: Envelope,
    pulse_1_lc: SoundLengthCounter,
    pulse_1_sweep: Sweeper,
    pulse_1_freq: f64,
    pulse_1_sp: f64,

    pulse_2_sample: f64,
    pulse_2_halt: bool,
    pulse_2_seq: Sequencer,
    pulse_2_osc: Oscillator,
    pulse_2_env: Envelope,
    pulse_2_lc: SoundLengthCounter,
    pulse_2_sweep: Sweeper,
    pulse_2_freq: f64,
    pulse_2_sp: f64,

    noise_sample: f64,
    noise_halt: bool,
    noise_seq: Sequencer,
    noise_env: Envelope,
    noise_lc: SoundLengthCounter,

    frame_clock_counter: u32,
    clock_counter: u32
}

impl Apu2a03
{
    pub fn new() -> Self
    {
        let mut s = Apu2a03
        {
            pulse_1_sample: 0.0,
            pulse_1_halt: false,
            pulse_1_seq: Sequencer::new(),
            pulse_1_osc: Oscillator::new(),
            pulse_1_env: Envelope::new(),
            pulse_1_lc: SoundLengthCounter::new(),
            pulse_1_sweep: Sweeper::new(),
            pulse_1_freq: 0.0,
            pulse_1_sp: 0.0,

            pulse_2_sample: 0.0,
            pulse_2_halt: false,
            pulse_2_seq: Sequencer::new(),
            pulse_2_osc: Oscillator::new(),
            pulse_2_env: Envelope::new(),
            pulse_2_lc: SoundLengthCounter::new(),
            pulse_2_sweep: Sweeper::new(),
            pulse_2_freq: 0.0,
            pulse_2_sp: 0.0,

            noise_sample: 0.0,
            noise_halt: false,
            noise_seq: Sequencer::new(),
            noise_env: Envelope::new(),
            noise_lc: SoundLengthCounter::new(),

            frame_clock_counter: 0,
            clock_counter: 0
        };

        // Initialize noise sequence
        s.noise_seq.set_sequence(0x1);
        s
    }

    pub fn get_final_mix(&mut self) -> f64
    {
        (self.pulse_1_sample - 0.8) * 0.3 +
			(self.pulse_2_sample - 0.8) * 0.3 +
			2.0 * (self.noise_sample - 0.5) * 0.3
    }

    pub fn get_debug_info(&self) -> (f64, f64, f64, f64, f64)
    {
        (self.pulse_1_freq, self.pulse_1_sp, self.pulse_2_freq, self.pulse_2_sp, self.noise_sample)
    }

    pub fn set_oscillator_sample_rate(&mut self, osc_sample_rate: f64)
    {
        self.pulse_1_osc.set_oscillator_sample_rate(osc_sample_rate);
        self.pulse_2_osc.set_oscillator_sample_rate(osc_sample_rate);
    }

    fn rotate_sequence(s: &mut u32, _: bool)
    {
        *s = ((*s & 0x0001) << 7) | ((*s & 0x00FE) >> 1);
    }

    fn noise_seq_update(s: &mut u32, mode: bool)
    {
        if !mode
        {
            *s = (((*s & 0x0001) ^ ((*s & 0x0002) >> 1)) << 14) | ((*s & 0x7FFF) >> 1);
        }
        else
        {
            *s = (((*s & 0x0001) ^ ((*s & 0x0007) >> 1)) << 14) | ((*s & 0x7FFF) >> 1);
        }

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
                        self.pulse_1_seq.set_sequence(0b00000001);
                        self.pulse_1_osc.set_duty_cycle(0.125);
                    },
                    0x01 =>
                    {
                        self.pulse_1_seq.set_sequence(0b00000011);
                        self.pulse_1_osc.set_duty_cycle(0.250);
                    },
                    0x02 =>
                    {
                        self.pulse_1_seq.set_sequence(0b00001111);
                        self.pulse_1_osc.set_duty_cycle(0.500);
                    },
                    0x03 =>
                    {
                        self.pulse_1_seq.set_sequence(0b11111100);
                        self.pulse_1_osc.set_duty_cycle(0.750);
                    },
                    _ => panic!("Impossible")
                }

                self.pulse_1_halt = data & 0x20 == 0x20;
                self.pulse_1_env.set_volume(data as u16 & 0x0F);
                self.pulse_1_env.set_disable(data as u16 & 0x10 == 0x10);

                true
            },
            0x4001 =>
            {
                self.pulse_1_sweep.set_enabled(data & 0x80 == 0x80);
                self.pulse_1_sweep.set_period((data & 0x70) >> 4);
                self.pulse_1_sweep.set_down(data & 0x08 == 0x08);
                self.pulse_1_sweep.set_shift(data & 0x07);
                self.pulse_1_sweep.set_reload(true);
                true
            },
            0x4002 =>
            {
                self.pulse_1_seq.set_reload(self.pulse_1_seq.get_reload() & 0xFF00 | data as u16);
                true
            },
            0x4003 =>
            {
                self.pulse_1_seq.set_reload(((data as u16 & 0x07) << 8) | self.pulse_1_seq.get_reload() & 0x00FF);
                self.pulse_1_seq.set_timer(self.pulse_1_seq.get_reload());
                self.pulse_1_lc.set_counter(sound_length_counter::LENGTH_TABLE[((data & 0xF8) >> 3) as usize]);
                self.pulse_1_env.set_start(true);
                true
            },
            0x4004 =>
            {
                match (data & 0xC0) >> 6
                {
                    0x00 =>
                    {
                        self.pulse_2_seq.set_sequence(0b00000001);
                        self.pulse_2_osc.set_duty_cycle(0.125);
                    },
                    0x01 =>
                    {
                        self.pulse_2_seq.set_sequence(0b00000011);
                        self.pulse_2_osc.set_duty_cycle(0.250);
                    },
                    0x02 =>
                    {
                        self.pulse_2_seq.set_sequence(0b00001111);
                        self.pulse_2_osc.set_duty_cycle(0.500);
                    },
                    0x03 =>
                    {
                        self.pulse_2_seq.set_sequence(0b11111100);
                        self.pulse_2_osc.set_duty_cycle(0.750);
                    },
                    _ => panic!("Impossible")
                }

                self.pulse_2_halt = data & 0x20 == 0x20;
                self.pulse_2_env.set_volume(data as u16 & 0x0F);
                self.pulse_2_env.set_disable(data as u16 & 0x10 == 0x10);
                true
            },
            0x4005 =>
            {
                self.pulse_2_sweep.set_enabled(data & 0x80 == 0x80);
                self.pulse_2_sweep.set_period((data & 0x70) >> 4);
                self.pulse_2_sweep.set_down(data & 0x08 == 0x08);
                self.pulse_2_sweep.set_shift(data & 0x07);
                self.pulse_2_sweep.set_reload(true);
                true
            },
            0x4006 =>
            {
                self.pulse_2_seq.set_reload(self.pulse_2_seq.get_reload() & 0xFF00 | data as u16);
                true
            },
            0x4007 =>
            {
                self.pulse_2_seq.set_reload(((data as u16 & 0x07) << 8) | self.pulse_2_seq.get_reload() & 0x00FF);
                self.pulse_2_seq.set_timer(self.pulse_2_seq.get_reload());
                self.pulse_2_lc.set_counter(sound_length_counter::LENGTH_TABLE[((data & 0xF8) >> 3) as usize]);
                self.pulse_2_env.set_start(true);
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
                self.noise_env.set_volume((data & 0x0F) as u16);
                self.noise_env.set_disable(data & 0x10 == 0x10);
                self.noise_halt = data & 0x20 == 0x20;
                true
            },
            0x400D =>
            {
                true
            },
            0x400E =>
            {
                self.noise_seq.set_mode(data & 0xF0 == 0xF0);

                match data & 0x0F
                {
                    0x00 => self.noise_seq.set_reload(0),
                    0x01 => self.noise_seq.set_reload(4),
                    0x02 => self.noise_seq.set_reload(8),
                    0x03 => self.noise_seq.set_reload(16),
                    0x04 => self.noise_seq.set_reload(32),
                    0x05 => self.noise_seq.set_reload(64),
                    0x06 => self.noise_seq.set_reload(96),
                    0x07 => self.noise_seq.set_reload(128),
                    0x08 => self.noise_seq.set_reload(160),
                    0x09 => self.noise_seq.set_reload(202),
                    0x0A => self.noise_seq.set_reload(254),
                    0x0B => self.noise_seq.set_reload(380),
                    0x0C => self.noise_seq.set_reload(508),
                    0x0D => self.noise_seq.set_reload(1016),
                    0x0E => self.noise_seq.set_reload(2034),
                    0x0F => self.noise_seq.set_reload(4068),
                    _ => panic!("Invalid reload value for noise channel")
                }
                true
            },
            0x400F =>
            {
                self.pulse_1_env.set_start(true);
                self.pulse_2_env.set_start(true);
                self.noise_env.set_start(true);
                self.noise_lc.set_counter(sound_length_counter::LENGTH_TABLE[((data & 0xF8) >> 3) as usize]);
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
                self.pulse_1_seq.set_enable(data & 0x01 == 0x01);
                self.pulse_2_seq.set_enable(data & 0x02 == 0x02);
                self.noise_seq.set_enable(data & 0x04 == 0x04);
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

    fn cpu_read(&mut self, _: u16, data: &mut u8) -> bool
    {
        *data = 0x00;
        true
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
                self.pulse_1_env.set_looped(self.pulse_1_halt);
                self.pulse_1_env.clock_tick();

                self.pulse_2_env.set_looped(self.pulse_2_halt);
                self.pulse_2_env.clock_tick();

                self.noise_env.set_looped(self.noise_halt);
                self.noise_env.clock_tick();
            }

            // Half frame beats adjust the note length and frequency sweepers
            if half_frame_clock
            {
                self.pulse_1_lc.set_enable(self.pulse_1_seq.get_enable());
                self.pulse_1_lc.set_halt(self.pulse_1_halt);
                self.pulse_1_lc.clock_tick();

                self.pulse_2_lc.set_enable(self.pulse_1_seq.get_enable());
                self.pulse_2_lc.set_halt(self.pulse_1_halt);
                self.pulse_2_lc.clock_tick();

                self.noise_lc.set_enable(self.noise_seq.get_enable());
                self.noise_lc.set_halt(self.noise_halt);
                self.noise_lc.clock_tick();

                self.pulse_1_sweep.set_target(self.pulse_1_seq.get_reload());
                self.pulse_1_sweep.set_channel(false);
                self.pulse_1_sweep.clock_tick();
                self.pulse_1_seq.set_reload(self.pulse_1_sweep.get_target());

                self.pulse_2_sweep.set_target(self.pulse_2_seq.get_reload());
                self.pulse_2_sweep.set_channel(true);
                self.pulse_2_sweep.clock_tick();
                self.pulse_2_seq.set_reload(self.pulse_2_sweep.get_target());
            }

            // Pulse 1
            {
                self.pulse_1_seq.set_callback(Apu2a03::rotate_sequence);
                self.pulse_1_seq.clock_tick();
                
                self.pulse_1_sp = self.pulse_1_seq.get_reload() as f64 + 1.0;
                self.pulse_1_freq = 1789773.0 / (16.0 * self.pulse_1_sp);
                self.pulse_1_osc.set_base_frequency(self.pulse_1_freq);
                self.pulse_1_osc.set_amplitude((self.pulse_1_env.get_output() as f64 - 1.0) / 16.0);
                let new_pulse_1_sample = self.pulse_1_osc.get_output();

                if self.pulse_1_lc.get_counter() > 0 && self.pulse_1_seq.get_timer() >= 8 && !self.pulse_1_sweep.get_mute() && self.pulse_1_env.get_output() > 2
                {
                    self.pulse_1_sample += (new_pulse_1_sample - self.pulse_1_sample) * 0.5;
                }
                else
                {
                    self.pulse_1_sample = 0.0;
                }

                if !self.pulse_1_seq.get_enable()
                {
                    self.pulse_1_sample = 0.0;
                }
            }

            // Pulse 2
            {
                self.pulse_2_seq.set_callback(Apu2a03::rotate_sequence);
                self.pulse_2_seq.clock_tick();

                self.pulse_2_sp = self.pulse_2_seq.get_reload() as f64 + 1.0;
                self.pulse_2_freq = 1789773.0 / (16.0 * self.pulse_2_sp);
                self.pulse_2_osc.set_base_frequency(self.pulse_2_freq);
                self.pulse_2_osc.set_amplitude((self.pulse_2_env.get_output() as f64 - 1.0) / 16.0);
                let new_pulse_2_sample = self.pulse_2_osc.get_output();

                if self.pulse_2_lc.get_counter() > 0 && self.pulse_2_seq.get_timer() >= 8 && !self.pulse_2_sweep.get_mute() && self.pulse_2_env.get_output() > 2
                {
                    self.pulse_2_sample += (new_pulse_2_sample - self.pulse_2_sample) * 0.5;
                }
                else
                {
                    self.pulse_2_sample = 0.0;
                }

                if !self.pulse_2_seq.get_enable()
                {
                    self.pulse_2_sample = 0.0;
                }
            }
            
            // Noise
            {
                self.noise_seq.set_callback(Apu2a03::noise_seq_update);
                self.noise_seq.clock_tick();

                if self.noise_lc.get_counter() > 0
                {
                    self.noise_sample = self.noise_seq.get_output() as f64 * (self.noise_env.get_output().saturating_sub(1)) as f64 / 16.0;
                }

                if !self.noise_seq.get_enable()
                {
                    self.noise_sample = 0.0;
                }
            }
        }

        // Frequency sweepers change at high frequency
        self.pulse_1_sweep.set_target(self.pulse_1_seq.get_reload());
        self.pulse_1_sweep.track();
        self.pulse_1_seq.set_reload(self.pulse_1_sweep.get_target());

        self.pulse_2_sweep.set_target(self.pulse_2_seq.get_reload());
        self.pulse_2_sweep.track();
        self.pulse_2_seq.set_reload(self.pulse_2_sweep.get_target());

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