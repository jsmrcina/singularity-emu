use std::sync::{Arc, Mutex};

use fundsp::{hacker::*, prelude::PulseWave};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

type OscillatorType = An<Pipe<f64, Stack<f64, Var<f64>, Var<f64>>, PulseWave<f64>>>;

pub struct SoundEngine
{
    sample_rate: f32,
    channels: usize, 
    oscillators: Vec<OscillatorType>,
    frequency: Vec<Shared<f64>>,
    duty_cycle: Vec<Shared<f64>>,
    harmonics: i32
}

impl SoundEngine
{
    pub fn new() -> Self
    {
        SoundEngine {
            sample_rate: 0.0,
            channels: 0,
            oscillators: Vec::new(),
            frequency: vec!(shared(440.0)),
            duty_cycle: vec!(shared(0.5)),
            harmonics: 3
        }
    }

    pub fn vary_freq(&mut self)
    {
        // let orig_fund_freq = self.get_fundamental_freq();
        // self.set_freq(orig_fund_freq + 1.0);
        // if self.get_fundamental_freq() >= 880.0
        // {
        //     self.set_freq(440.0);
        // }

        // let orig_duty_cycle = self.get_duty_cycle();
        // self.set_duty_cycle(orig_duty_cycle + 0.1);
        // if self.get_duty_cycle() >= 0.9
        // {
        //     self.set_duty_cycle(0.5);
        // }
    }

    pub fn get_fundamental_freq(&mut self) -> f64
    {
        self.frequency[0].value()
    }

    pub fn get_duty_cycle(&mut self) -> f64
    {
        self.duty_cycle[0].value()
    }

    pub fn set_freq(&mut self, value: f64)
    {
        self.frequency[0].set_value(value);

        for i in 1..self.harmonics
        {
            self.frequency[i as usize].set_value(value * (i as f64 + 1.0));
        }
    }

    pub fn set_duty_cycle(&mut self, value: f64)
    {
        for i in 0..self.harmonics
        {
            self.duty_cycle[i as usize].set_value(value);
        }
    }

    pub fn sound_out(engine: Arc<Mutex<SoundEngine>>, data: &mut [f32])
    {
        let mut inner = engine.lock().unwrap();

        for frame in data.chunks_mut(inner.channels)
        {
            let mut sample = 0.0;
            for osc in &mut inner.oscillators
            {
               sample += osc.get_mono();
            }
            sample /= inner.oscillators.len() as f64;

            for sample_slot in frame.iter_mut()
            {
                *sample_slot = sample as f32;
            }
        }
    }

    pub fn initialize(engine: Arc<Mutex<SoundEngine>>) -> cpal::Stream
    {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to get default output device");
        let supported_config = device.default_output_config().expect("Failed to get default output config");

        let cloned_engine = engine.clone();
        let mut inner = cloned_engine.lock().unwrap();
        {
            inner.sample_rate = supported_config.sample_rate().0 as f32;
            inner.channels = supported_config.channels() as usize;

            println!("Device is: {}", device.name().unwrap());
            println!("sr: {}, ch: {}", inner.sample_rate, inner.channels);
            println!("sample format: {}", supported_config.sample_format());


            for i in 0..inner.harmonics
            {
                if i != 0
                {
                    // Add the appropriate harmonic frequencies
                    let new_freq = shared(inner.frequency[0].value() * (i as f64 + 1.0));
                    inner.frequency.push(new_freq);

                    let new_dc = shared(inner.duty_cycle[0].value());
                    inner.duty_cycle.push(new_dc);
                }   

                let mut osc = (var(&inner.frequency[i as usize]) | var(&inner.duty_cycle[i as usize])) >> pulse();
                osc.set_sample_rate(inner.sample_rate as f64);
                inner.oscillators.push(osc);
            }
        }
        drop(inner);

        let config: cpal::StreamConfig = supported_config.into();
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
            {
                let engine_ref = cloned_engine.clone();
                SoundEngine::sound_out(engine_ref, data);
            },
            move |err| {
                eprintln!("An error occurred on stream: {}", err);
            }, None).unwrap();

        stream.play().unwrap();
        stream
    }
}

impl Default for SoundEngine {
    fn default() -> Self {
        SoundEngine::new()
    }
}