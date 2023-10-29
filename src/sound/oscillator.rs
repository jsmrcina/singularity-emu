use fundsp::hacker::*;

// type OscillatorType = fundsp::combinator::An<fundsp::audionode::Binop<f64, fundsp::audionode::FrameAdd<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, f64>,
//     fundsp::audionode::Binop<f64, fundsp::audionode::FrameAdd<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, f64>,
//     fundsp::audionode::Binop<f64, fundsp::audionode::FrameAdd<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, f64>,
//     fundsp::audionode::Binop<f64, fundsp::audionode::FrameAdd<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, f64>,
//     fundsp::audionode::Pipe<f64, fundsp::audionode::Stack<f64, fundsp::hacker::Var<f64>, fundsp::hacker::Var<f64>>, fundsp::prelude::PulseWave<f64>>,
//     fundsp::audionode::Pipe<f64, fundsp::audionode::Stack<f64, fundsp::hacker::Var<f64>, fundsp::hacker::Var<f64>>, fundsp::prelude::PulseWave<f64>>>,
//     fundsp::audionode::Pipe<f64, fundsp::audionode::Stack<f64, fundsp::hacker::Var<f64>, fundsp::hacker::Var<f64>>, fundsp::prelude::PulseWave<f64>>>,
//     fundsp::audionode::Pipe<f64, fundsp::audionode::Stack<f64, fundsp::hacker::Var<f64>, fundsp::hacker::Var<f64>>, fundsp::prelude::PulseWave<f64>>>,
//     fundsp::audionode::Pipe<f64, fundsp::audionode::Stack<f64, fundsp::hacker::Var<f64>, fundsp::hacker::Var<f64>>, fundsp::prelude::PulseWave<f64>>>>;

type OscillatorType = An<Pipe<f64, Stack<f64, Var<f64>, Var<f64>>, prelude::PulseWave<f64>>>;

type OptionalOscillatorType = Option<OscillatorType>;

pub struct Oscillator
{
    frequencies: Vec<Shared<f64>>,
    duty_cycle: Shared<f64>,
    amplitude: Shared<f64>,
    dsp_oscillator: OptionalOscillatorType,
    harmonics: u32
}

impl Oscillator
{
    pub fn new() -> Self
    {
        let mut s = Oscillator
        {
            frequencies: Vec::new(),
            duty_cycle: shared(0.0),
            amplitude: shared(0.0),
            dsp_oscillator: None,
            harmonics: 5
        };

        for _ in 0..s.harmonics
        {
            s.frequencies.push(shared(0.0));
        }

        s.dsp_oscillator = Some(
                (var(&s.frequencies[0]) | var(&s.duty_cycle)) >> pulse()// +
                // ((var(&s.frequencies[1]) | var(&s.duty_cycle)) >> pulse()) +
                // ((var(&s.frequencies[2]) | var(&s.duty_cycle)) >> pulse()) +
                // ((var(&s.frequencies[3]) | var(&s.duty_cycle)) >> pulse()) +
                // ((var(&s.frequencies[4]) | var(&s.duty_cycle)) >> pulse())
            );
        s
    }

    pub fn get_duty_cycle(&mut self) -> f64
    {
        self.duty_cycle.value()
    }

    pub fn set_base_frequency(&mut self, value: f64)
    {
        self.frequencies[0].set_value(value);

        for i in 1..self.harmonics
        {
            self.frequencies[i as usize].set_value(value * i as f64);
        }
    }

    pub fn set_duty_cycle(&mut self, value: f64)
    {
        self.duty_cycle.set_value(value);
    }

    pub fn set_oscillator_sample_rate(&mut self, osc_sample_rate: f64)
    {
        self.dsp_oscillator.as_mut().unwrap().set_sample_rate(osc_sample_rate);
    }

    pub fn get_output(&mut self) -> f64
    {
        self.dsp_oscillator.as_mut().unwrap().get_mono() * self.amplitude.value()
    }

    pub fn get_amplitude(&mut self) -> f64
    {
        self.amplitude.value()
    }

    pub fn set_amplitude(&mut self, amplitude: f64)
    {
        self.amplitude.set_value(amplitude);
    }
}

impl Default for Oscillator {
    fn default() -> Self {
        Self::new()
    }
}