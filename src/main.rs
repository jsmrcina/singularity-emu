use bus::main_bus::MainBus;
use cartridge::cart::Cart;
use crate::cpu::cpu6502::Flags6502;
use crate::traits::Clockable;

use traits::ReadWrite;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::ops::Bound;

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

struct MainState
{
    bus: Rc<RefCell<MainBus>>,
    map_asm: BTreeMap<u16, String>,
    emulation_run: bool,
    residual_time: f32,
    selected_palette: u8
}

impl MainState
{
    fn new() -> GameResult<MainState>
    {
        let mut s = MainState
        {
            bus: Rc::new(RefCell::new(MainBus::new())),
            map_asm: BTreeMap::new(),
            emulation_run: false,
            residual_time: 0.0,
            selected_palette: 0
        };

        // Link the CPU to the BUS
        // TODO: Better to move this?
        let bus_trait_object = Rc::clone(&s.bus) as Rc<RefCell<dyn ReadWrite>>;
        s.bus.borrow_mut().get_cpu().borrow_mut().set_bus(Some(bus_trait_object));

        let cart = Cart::new("data\\nestest.nes".to_string());
        match cart
        {
            Ok(x) =>
            {
                let cart_wrapper = Rc::new(RefCell::new(x));
                s.bus.borrow_mut().insert_cartridge(cart_wrapper);
            },
            _ =>
            {
                panic!("Failed to load cartridge");
            }
        }

        // Dissemble code into our main state so we can render it\
        let cpu = s.bus.borrow_mut().get_cpu();
        s.map_asm = cpu.borrow_mut().disassemble(0x0000, 0xFFFF);

        // Reset the CPU
        s.reset();

        Ok(s)
    }

    const OFFSET_X: f32 = 16.0;
    const OFFSET_Y: f32 = 14.0;

    // fn draw_cpu_ram(&mut self, x: i32, y: i32, mut n_addr: u16, n_rows: i32, n_cols: i32, canvas: &mut ggez::graphics::Canvas)
    // {
    //     let n_cpu_ram_x: f32 = x as f32;
    //     let mut n_cpu_ram_y: f32 = y as f32;

    //     for _ in 0..n_rows
    //     {
    //         let mut s_offset: String = format!("${:04x}:", n_addr);
    //         for _ in 0..n_cols
    //         {
    //             let mut data: u8 = 0;
    //             self.bus.borrow_mut().cpu_read(n_addr, &mut data);
    //             s_offset = s_offset + &format!(" {:02x}", data);
    //             n_addr = n_addr + 1;
    //         }
    //         let text = Text::new(s_offset);
    //         canvas.draw(&text, Vec2::new(n_cpu_ram_x, n_cpu_ram_y));
    //         n_cpu_ram_y = n_cpu_ram_y + MainState::OFFSET_Y;
    //     }
    // }

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

    fn reset(&mut self)
    {
        // Reset the main bus
        self.bus.borrow_mut().reset();

        // Reset the CPU
        let cpu = self.bus.borrow_mut().get_cpu();
        cpu.borrow_mut().reset();
    }

}

impl Clockable for MainState
{
    fn clock_tick(&mut self)
    {
        let cpu = self.bus.borrow_mut().get_cpu();
        let ppu = self.bus.borrow_mut().get_ppu();
        let clock_counter = self.bus.borrow().get_clock_counter();

        ppu.borrow_mut().clock_tick();
        if clock_counter % 3 == 0
        {
            cpu.borrow_mut().clock_tick();
        }

        if ppu.borrow().get_nmi()
        {
            ppu.borrow_mut().set_nmi(false);
            cpu.borrow_mut().nmi();
        }

        self.bus.borrow_mut().increment_clock_counter();
    }
}

impl event::EventHandler<ggez::GameError> for MainState
{
    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        let cpu = self.bus.borrow_mut().get_cpu();

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
                    if self.bus.borrow_mut().get_ppu().borrow().frame_complete()
                    {
                        break;
                    }
                }

                self.bus.borrow_mut().get_ppu().borrow_mut().set_frame_complete(false);
            }
        }
        else
        {
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

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::P)
        {
            self.selected_palette += 1;
            self.selected_palette %= 7;
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
        );

        // MainState::draw_cpu_ram(self, 2, 2, 0x0000, 16, 16, &mut canvas);
        // MainState::draw_cpu_ram(self, 2, 250, 0x8000, 16, 16, &mut canvas);
        MainState::draw_cpu(self, 525.0, 2.0, &mut canvas);
        MainState::draw_code(self, 525.0, 100.0, 27, &mut canvas);

        let ppu = self.bus.borrow_mut().get_ppu();

        // TODO: Stopped here, looks like the nametables aren't correct under the covers.
        let mut x = 0;
        let mut y = 0;

        while y < 30
        {
            while x < 32
            {
                let output = format!("{:02x}", ppu.borrow_mut().get_name_table()[0][y * 32 + x]);
                canvas.draw(&Text::new(output), Vec2::new((x as f32) * 16.0, (y as f32) * 16.0));
                x += 1;
            }

            x = 0;
            y += 1;
        }

        // ppu.borrow_mut().render(ctx, &mut canvas, self.selected_palette);
        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult
{
    let (ctx, event_loop) = ggez::ContextBuilder::new("singularity-emu", "jsmrcina")
        .window_setup(ggez::conf::WindowSetup::default().title("Singularity Emu"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(1280.0, 1024.0))
        .build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state);
}
