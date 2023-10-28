use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait};

use crate::{traits::Clockable, MainState};

pub struct SoundEngine
{
    sample_rate: u32,
    channels: usize,
    audio_time_per_system_sample: f64,
    audio_time_per_nes_clock: f64,
    audio_time: f64,
    final_mix: f64,
    emulator_tick_callback: fn() -> bool
}

impl SoundEngine
{
    pub fn new(emulator_tick_callback: fn() -> bool) -> Self
    {
        SoundEngine {
            sample_rate: 0,
            channels: 0,
            audio_time_per_system_sample: 0.0,
            audio_time_per_nes_clock: 0.0,
            audio_time: 0.0,
            final_mix: 0.0,
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

                let inner = engine.lock().unwrap();
                for sample_slot in frame.iter_mut()
                {
                    *sample_slot = inner.final_mix as f32;
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
        }
        drop(inner);

        let config: cpal::StreamConfig = supported_config.into();
        device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
            {
                let engine_ref = cloned_engine.clone();
                SoundEngine::sound_out(engine_ref, data);
            },
            move |err| {
                eprintln!("An error occurred on stream: {}", err);
            }, None).unwrap()
    }

    pub fn get_sample_rate(&self) -> u32
    {
        self.sample_rate
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
            self.final_mix = MainState::get_instance().apu.as_ref().unwrap().lock().unwrap().get_final_mix();
            return true;
        }

        false
    }
}