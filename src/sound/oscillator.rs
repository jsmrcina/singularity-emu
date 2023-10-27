use fundsp::{hacker::*, prelude::PulseWave};

type OscillatorType = An<Pipe<f64, Stack<f64, Var<f64>, Var<f64>>, PulseWave<f64>>>;

pub struct Oscillator
{
    frequency: Shared<f64>,
    duty_cycle: Shared<f64>,
    dsp_oscillator: Option<OscillatorType>,
}

impl Oscillator
{
    pub fn new() -> Self
    {
        let mut s = Oscillator
        {
            frequency: shared(0.0),
            duty_cycle: shared(0.0),
            dsp_oscillator: None
        };

        s.dsp_oscillator = Some((var(&s.frequency) | var(&s.duty_cycle)) >> pulse());
        s
    }

    pub fn get_freq(&mut self) -> f64
    {
        self.frequency.value()
    }

    pub fn get_duty_cycle(&mut self) -> f64
    {
        self.duty_cycle.value()
    }

    pub fn set_freq(&mut self, value: f64)
    {
        self.frequency.set_value(value);
    }

    pub fn set_duty_cycle(&mut self, value: f64)
    {
        self.duty_cycle.set_value(value);
    }

    pub fn get_mono(&mut self) -> f64
    {
        return self.dsp_oscillator.as_mut().unwrap().get_mono();
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64)
    {
        self.dsp_oscillator.as_mut().unwrap().set_sample_rate(sample_rate);
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Self::new()
    }
}