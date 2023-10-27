use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::{traits::Clockable, MainState};

use super::oscillator::Oscillator;

pub struct SoundEngine
{
    sample_rate: u32,
    channels: usize, 
    oscillators: Vec<Oscillator>,
    audio_time_per_system_sample: f64,
    audio_time_per_nes_clock: f64,
    audio_time: f64,
    osc_sample: [(f64, f64); 4],
    emulator_tick_callback: fn() -> bool
}

impl SoundEngine
{
    pub fn new(emulator_tick_callback: fn() -> bool) -> Self
    {
        SoundEngine {
            sample_rate: 0,
            channels: 0,
            oscillators: Vec::new(),
            audio_time_per_system_sample: 0.0,
            audio_time_per_nes_clock: 0.0,
            audio_time: 0.0,
            osc_sample: [(0.0, 0.0); 4],
            emulator_tick_callback
        }
    }

    pub fn sound_out(engine: Arc<Mutex<SoundEngine>>, data: &mut [f32])
    {
        {
            let channels = engine.lock().unwrap().channels;
            for frame in data.chunks_mut(channels)
            {
                // Generate the next sample
                {
                    let inner = engine.lock().unwrap();
        
                    // This is set elsewhere
                    let callback = inner.emulator_tick_callback;
                    drop(inner);
        
                    let mut sample_ready = false;
                    while !sample_ready
                    {
                        // Call into MainState and keep clocking; this takes the engine lock
                        // so we need to drop it before continuing
                        sample_ready = callback();
                    }
                }

                let mut inner = engine.lock().unwrap();
                let mut dsp_samples: [f64; 4] = [0.0; 4];

                (0..inner.oscillators.len()).for_each(|i: usize|
                {
                    let osc_sample = inner.osc_sample[i];
                    let osc: &mut Oscillator = &mut inner.oscillators[i];

                    osc.set_duty_cycle(osc_sample.0);
                    osc.set_freq(osc_sample.1);
                    dsp_samples[i] = osc.get_mono();
                });

                let final_mix = ((1.0 * dsp_samples[0]) - 0.8) * 0.1 +
                                ((1.0 * dsp_samples[1]) - 0.8) * 0.1;
                                //((2.0 * (noise_output - 0.5))) * 0.1;

                for sample_slot in frame.iter_mut()
                {
                    *sample_slot = final_mix as f32;
                }
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
            inner.audio_time_per_system_sample = 1.0 / supported_config.sample_rate().0 as f64;
            inner.audio_time_per_nes_clock = 1.0 / 5369318.0; // PPU Clock Frequency, based on NTSC NES core frequency
            inner.sample_rate = supported_config.sample_rate().0;
            inner.channels = supported_config.channels() as usize;


            // Create NES oscillators
            let mut pulse_1 = Oscillator::new(); 
            pulse_1.set_sample_rate(inner.sample_rate as f64);
            inner.oscillators.push(pulse_1);

            let mut pulse_2 = Oscillator::new(); 
            pulse_2.set_sample_rate(inner.sample_rate as f64);
            inner.oscillators.push(pulse_2);
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

impl Clockable for SoundEngine
{
    fn clock_tick(&mut self) -> bool
    {
        self.audio_time += self.audio_time_per_nes_clock;
        if self.audio_time >= self.audio_time_per_system_sample
        {
            self.audio_time -= self.audio_time_per_system_sample;
            self.osc_sample = MainState::get_instance().apu.as_ref().unwrap().lock().unwrap().get_osc_data();
            return true;
        }

        false
    }
}