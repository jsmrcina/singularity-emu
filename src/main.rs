use bus::main_bus::MainBus;
use cartridge::cart::Cart;
use cpu::cpu6502::Cpu6502;
use gfx::ppu2c02::Ppu2c02;
use sound::apu2a03::Apu2a03;
use input::controller::NesKey;
use sound::sound_engine::SoundEngine;
use traits::{ReadWrite, Resettable};
use crate::cpu::cpu6502::Flags6502;
use crate::traits::Clockable;
use std::sync::Once;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::ops::Bound;
use std::rc::Rc;
use std::collections::BTreeMap;

// Game engine
use ggez::event;
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::graphics::Text;

pub mod traits;
pub mod bus;
pub mod memory;
pub mod cpu;
pub mod mapper;
pub mod gfx;
pub mod cartridge;
pub mod input;
pub mod sound;

struct MainState
{
    bus: Rc<RefCell<MainBus>>,
    cpu: Option<Rc<RefCell<Cpu6502>>>,
    ppu: Option<Rc<RefCell<Ppu2c02>>>,
    apu: Option<Rc<RefCell<Apu2a03>>>,
    map_asm: BTreeMap<u16, String>,
    emulation_run: bool,
    residual_time: f32,
    sound_engine: Option<Arc<Mutex<SoundEngine>>>,
    sound_thread: Option<cpal::Stream>
}

static mut INSTANCE: Option<MainState> = None;
static INIT: Once = Once::new();


impl MainState
{
    fn initialize(&mut self)
    {
        self.sound_engine = Some(Arc::new(Mutex::new(SoundEngine::new(MainState::emulator_tick))));

        // Link the CPU to the BUS
        // TODO: Better to move this?
        // TODO: Revert back to using the trait once CPU is debugged
        let bus_trait_object = Rc::clone(&self.bus);// as Rc<RefCell<dyn ReadWrite>>;
        self.bus.borrow_mut().get_cpu().borrow_mut().set_bus(Some(bus_trait_object));

        let cart = Cart::new("data\\super mario.nes".to_string());
        match cart
        {
            Ok(x) =>
            {
                let cart_wrapper = Rc::new(RefCell::new(x));
                self.bus.borrow_mut().insert_cartridge(cart_wrapper);
            },
            _ =>
            {
                panic!("Failed to load cartridge");
            }
        }

        // Dissemble code into our main state so we can render it\
        let cpu = self.bus.borrow_mut().get_cpu();
        self.map_asm = cpu.borrow_mut().disassemble(0x0000, 0xFFFF, false);

        // Reset the CPU
        self.reset();
    }

    fn get_instance() -> &'static mut MainState
    {
        // Required to store the raw mutable pointer
        unsafe
        {
            INIT.call_once(|| {
                INSTANCE = Some(MainState
                {
                    bus: Rc::new(RefCell::new(MainBus::new())),
                    cpu: None,
                    ppu: None,
                    apu: None,
                    map_asm: BTreeMap::new(),
                    emulation_run: false,
                    residual_time: 0.0,
                    sound_engine: None,
                    sound_thread: None
                });

                (*INSTANCE.as_mut().unwrap()).initialize();
            });
            INSTANCE.as_mut().unwrap()
        }
    }

    const OFFSET_X: f32 = 16.0;
    const OFFSET_Y: f32 = 14.0;

    fn draw_cpu_ram(&mut self, x: i32, y: i32, mut n_addr: u16, n_rows: i32, n_cols: i32, canvas: &mut ggez::graphics::Canvas)
    {
        let n_cpu_ram_x: f32 = x as f32;
        let mut n_cpu_ram_y: f32 = y as f32;

        for _ in 0..n_rows
        {
            let mut s_offset: String = format!("${:04x}:", n_addr);
            for _ in 0..n_cols
            {
                let mut data: u8 = 0;
                self.bus.borrow_mut().cpu_read(n_addr, &mut data);
                s_offset = s_offset + &format!(" {:02x}", data);
                n_addr += 1;
            }
            let text = Text::new(s_offset);
            canvas.draw(&text, Vec2::new(n_cpu_ram_x, n_cpu_ram_y));
            n_cpu_ram_y += MainState::OFFSET_Y;
        }
    }

    fn draw_cpu(&mut self, x: f32, y: f32, canvas: &mut ggez::graphics::Canvas)
    {
        let cpu = self.bus.borrow_mut().get_cpu();

        canvas.draw(&Text::new("Status"), Vec2::new(x, y));
        let mut num_offset: f32 = 0.0;

        let color = if cpu.borrow().get_flag(Flags6502::N) == Flags6502::N as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("N"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::V) == Flags6502::V as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("V"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::U) == Flags6502::U as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("-"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::B) == Flags6502::B as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("B"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::D) == Flags6502::D as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("D"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::I) == Flags6502::I as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("I"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::Z) == Flags6502::Z as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("Z"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if cpu.borrow().get_flag(Flags6502::C) == Flags6502::C as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("C"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        
        num_offset = 1.0;

        canvas.draw(&Text::new(format!("PC: ${:04x}", cpu.borrow().get_pc())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("A: ${:02x} [{:02}]", cpu.borrow().get_a(), cpu.borrow().get_a())), 
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("X: ${:02x} [{:02}]", cpu.borrow().get_x(), cpu.borrow().get_x())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Y: ${:02x} [{:02}]", cpu.borrow().get_y(), cpu.borrow().get_y())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Stack P: ${:04x}", cpu.borrow().get_stkp())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));

        // TODO: Implement disassembly

    }

    fn draw_code(&mut self, x: f32, y: f32, n_lines: i32, canvas: &mut ggez::graphics::Canvas)
    {
        let cpu = self.bus.borrow_mut().get_cpu();

        let mut before_keys: Vec<_> = self.map_asm.range((Bound::Unbounded, Bound::Excluded(cpu.borrow().get_pc())))
                                                .rev()
                                                .take((n_lines / 2) as usize)
                                                .collect();

        before_keys.reverse();

        let after_keys: Vec<_> = self.map_asm.range((Bound::Excluded(cpu.borrow().get_pc()), Bound::Unbounded))
            .take((n_lines / 2) as usize)
            .collect();

        let mut num_offset: i32 = 0;
        for (_, value) in before_keys
        {
            canvas.draw(&Text::new(value), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset as f32)));
            num_offset += 1;
        }

        let inst = self.map_asm.get(&cpu.borrow().get_pc());
        match inst
        {
            
            Some(s) => canvas.draw(&Text::new(s),
                graphics::DrawParam::new().color(graphics::Color::CYAN).dest(Vec2::new(x, y + (MainState::OFFSET_Y * num_offset as f32)))),
            None => ()
        }

        num_offset += 1;

        for (_, value) in after_keys
        {
            canvas.draw(&Text::new(value), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset as f32)));
            num_offset += 1;
        }
    }

    fn draw_oam(&mut self, x: f32, y: f32, n_lines: i32, canvas: &mut ggez::graphics::Canvas)
    {
        for i in 0..(n_lines as u8)
        {
            let ppu = self.bus.borrow_mut().get_ppu();
            let byte_0 = ppu.borrow_mut().get_oam_memory_at_addr(i * 4);
            let byte_1 = ppu.borrow_mut().get_oam_memory_at_addr(i * 4 + 1);
            let byte_2 = ppu.borrow_mut().get_oam_memory_at_addr(i * 4 + 2);
            let byte_3 = ppu.borrow_mut().get_oam_memory_at_addr(i * 4 + 3);

            let s: String = format!("{:02x}: ({}, {}) ID: {:02x} AT: {:02x}",
                i,
                byte_3,
                byte_0,
                byte_1,
                byte_2);
            
            canvas.draw(&Text::new(s), Vec2::new(x, y + (i as f32 * MainState::OFFSET_Y)));
        }
    }

    fn process_controller_input(&mut self, ctx: &mut Context)
    {
        let mut bus = self.bus.borrow_mut();
        bus.get_controller(0).borrow_mut().clear_live_state();
        
        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::X)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::A);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::Z)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::B);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::A)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::START);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::S)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::SELECT);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::Up)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::UP);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::Down)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::DOWN);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::Left)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::LEFT);
        }

        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::Right)
        {
            bus.get_controller(0).borrow_mut().set_live_state_bit(NesKey::RIGHT);
        }
    }

    // For debugging purposes
    pub fn emulator_update_without_audio(&mut self, ctx: &mut Context) -> GameResult
    {
        if self.emulation_run
        {
            if self.residual_time > 0.0
            {
                // Sleeping
                self.residual_time -= ctx.time.delta().as_secs_f32();
            }
            else
            {
                // Rendering a frame
                self.residual_time += (1.0 / 60.0) - ctx.time.delta().as_secs_f32();
                loop
                {
                    self.clock_tick();
                    if self.ppu.as_ref().unwrap().borrow().frame_complete()
                    {
                        break;
                    }
                }

                self.ppu.as_ref().unwrap().borrow_mut().set_frame_complete(false);
            }
        }
        else
        {
            let cpu = self.bus.borrow_mut().get_cpu();

            // Stepping mode
            if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::C)
            {   
                loop
                {
                    self.clock_tick();
                    if cpu.borrow().complete()
                    {
                        break;
                    }
                }

                // Since the CPU runs slower, clear out any leftover instructions
                loop
                {
                    self.clock_tick();
                    if !cpu.borrow().complete()
                    {
                        break;
                    }
                }
            }

            if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::F)
            {   
                loop
                {
                    self.clock_tick();
                    if self.bus.borrow_mut().get_ppu().borrow().frame_complete()
                    {
                        break;
                    }
                }

                // Since the CPU runs slower, clear out any leftover instructions
                loop
                {
                    self.clock_tick();
                    if !cpu.borrow().complete()
                    {
                        break;
                    }
                }

                self.bus.borrow_mut().get_ppu().borrow_mut().set_frame_complete(false);
            }
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::R)
        {
            self.reset();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::Space)
        {
            self.emulation_run = !self.emulation_run;
        }

        self.process_controller_input(ctx);

        // TODO: For testing, remove eventually
        self.sound_engine.as_mut().unwrap().lock().unwrap().vary_freq();
        
        Ok(())
    }

    pub fn emulator_update_with_audio(&mut self, ctx: &mut Context) -> GameResult
    {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::R)
        {
            self.reset();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::Space)
        {
            self.emulation_run = !self.emulation_run;
        }

        self.process_controller_input(ctx);

        // TODO: For testing, remove eventually
        // self.sound_engine.lock().unwrap().vary_freq();
        
        Ok(())
    }

    pub fn emulator_tick()
    {
        MainState::get_instance().clock_tick();
    }

}

impl Clockable for MainState
{
    fn clock_tick(&mut self)
    {
        let cpu = self.cpu.as_ref().unwrap();
        let ppu = self.ppu.as_ref().unwrap();
        let apu = self.apu.as_ref().unwrap();
        let clock_counter = self.bus.borrow().get_clock_counter();

        ppu.borrow_mut().clock_tick();

        apu.borrow_mut().clock_tick();

        if clock_counter % 3 == 0
        {
            if self.bus.borrow().is_dma_transfer_in_progress()
            {
                let dma_info_ptr = self.bus.borrow_mut().get_dma_info();
                let mut dma_info = dma_info_ptr.borrow_mut();

                if dma_info.is_sync_needed()
                {
                    // Since DMA transfer can only be initiated on an even clock cycle, we synchronize here
                    if self.bus.borrow_mut().get_clock_counter() % 2 == 1
                    {
                        dma_info.set_sync_needed(false);
                    }
                }
                else
                {
                    // On even cycles, read from the bus (could be CPU, cart, etc.)
                    // On odd cycles, write to the PPU
                    if self.bus.borrow_mut().get_clock_counter() % 2 == 0
                    {
                        let mut data: u8 = 0;
                        self.bus.borrow_mut().cpu_read((dma_info.get_page() as u16) << 8 | (dma_info.get_addr() as u16), &mut data);
                        dma_info.set_data(data);
                    }
                    else
                    {
                        ppu.borrow_mut().set_oam_memory_at_addr(dma_info.get_addr(), dma_info.get_data());
                        let new_addr = dma_info.get_addr().wrapping_add(1);
                        dma_info.set_addr(new_addr);

                        if new_addr == 0x00
                        {
                            dma_info.set_transfer_in_progress(false);
                            dma_info.set_sync_needed(true);
                        }
                    }
                }
            }
            else
            {
                cpu.borrow_mut().clock_tick();
            }
        }

        // Synchronize with audio
        self.sound_engine.as_mut().unwrap().lock().unwrap().clock_tick();

        if ppu.borrow().get_nmi()
        {
            ppu.borrow_mut().set_nmi(false);
            cpu.borrow_mut().nmi();
        }

        self.bus.borrow_mut().increment_clock_counter();
    }
}

impl Resettable for MainState
{
    fn reset(&mut self)
    {
        // Reset the main bus
        self.bus.borrow_mut().reset();

        // Reset the CPU
        self.cpu = Some(self.bus.borrow_mut().get_cpu());
        self.cpu.as_ref().unwrap().borrow_mut().reset();

        // Reset the PPU
        self.ppu = Some(self.bus.borrow_mut().get_ppu());
        self.ppu.as_ref().unwrap().borrow_mut().reset();

        // Reset the APU
        self.apu = Some(self.bus.borrow_mut().get_apu());
        self.apu.as_ref().unwrap().borrow_mut().reset();
    }
}

struct EventHandlingState
{

}

impl EventHandlingState
{
    pub fn new() -> Self
    {
        EventHandlingState {  }
    }
}

impl event::EventHandler<ggez::GameError> for EventHandlingState
{

    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        MainState::get_instance().emulator_update_with_audio(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 0.0, 1.0]),
        );

        // Zero page
        MainState::draw_cpu_ram(MainState::get_instance(), 10, 750, 0x0000, 16, 16, &mut canvas);
        MainState::draw_cpu(MainState::get_instance(), 775.0, 2.0, &mut canvas);
        MainState::draw_code(MainState::get_instance(), 775.0, 100.0, 26, &mut canvas);
        MainState::draw_oam(MainState::get_instance(), 1175.0, 100.0, 26, &mut canvas);

        let ppu = MainState::get_instance().bus.borrow_mut().get_ppu();
        ppu.borrow_mut().render(ctx, &mut canvas, 3.0);
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult
{
    let (ctx, event_loop) = ggez::ContextBuilder::new("singularity-emu", "jsmrcina")
        .window_setup(ggez::conf::WindowSetup::default().title("Singularity Emu"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1440.0, 1080.0))
        .build()?;

    let main_state = MainState::get_instance();
    main_state.sound_thread = Some(SoundEngine::initialize(main_state.sound_engine.as_mut().unwrap().clone()));

    let event_handling_state = EventHandlingState::new();
    event::run(ctx, event_loop, event_handling_state);
}
