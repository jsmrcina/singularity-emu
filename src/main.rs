use bus::main_bus::MainBus;
use crate::cpu::cpu6502::Flags6502;

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
    map_asm: BTreeMap<u16, String>
}

impl MainState
{
    fn new() -> GameResult<MainState>
    {
        let mut s = MainState
        {
            bus: Rc::new(RefCell::new(MainBus::new())),
            map_asm: BTreeMap::new()
        };

        // Link the CPU to the BUS
        // TODO: Better to move this?
        let bus_trait_object = Rc::clone(&s.bus) as Rc<RefCell<dyn ReadWrite>>;
        s.bus.borrow_mut().get_cpu().borrow_mut().set_bus(Some(bus_trait_object));

        // Load some code into the emulator
        let code_str = String::from("A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA");
        let bytes: Vec<u8> = code_str
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).expect("Failed to parse hex string"))
            .collect();

        let mut offset: usize = 0x8000;
        for b in bytes
        {
            s.bus.borrow_mut().cpu_write(offset as u16, b);
            offset += 1;
        }

        // Set the reset vector
        s.bus.borrow_mut().cpu_write(0xFFFC, 0x00);
        s.bus.borrow_mut().cpu_write(0xFFFD, 0x80);

        // Dissemble code into our main state so we can render it
        s.map_asm = s.bus.borrow_mut().get_cpu().borrow().disassemble(0x0000, 0xFFFF);

        // Reset the CPU
        s.bus.borrow_mut().reset();

        Ok(s)
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
                s_offset = s_offset + &format!(" {:02x}", self.bus.borrow().cpu_read(n_addr));
                n_addr = n_addr + 1;
            }
            let text = Text::new(s_offset);
            canvas.draw(&text, Vec2::new(n_cpu_ram_x, n_cpu_ram_y));
            n_cpu_ram_y = n_cpu_ram_y + MainState::OFFSET_Y;
        }
    }

    fn draw_cpu(&mut self, x: f32, y: f32, canvas: &mut ggez::graphics::Canvas)
    {
        canvas.draw(&Text::new("Status"), Vec2::new(x, y));
        let mut num_offset: f32 = 0.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::N) == Flags6502::N as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("N"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::V) == Flags6502::V as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("V"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::U) == Flags6502::U as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("-"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::B) == Flags6502::B as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("B"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::D) == Flags6502::D as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("D"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::I) == Flags6502::I as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("I"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::Z) == Flags6502::Z as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("Z"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.bus.borrow_mut().get_cpu().borrow().get_flag(Flags6502::C) == Flags6502::C as u8 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("C"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        
        num_offset = 1.0;

        canvas.draw(&Text::new(format!("PC: ${:04x}", self.bus.borrow_mut().get_cpu().borrow().get_pc())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("A: ${:02x} [{:02}]", self.bus.borrow_mut().get_cpu().borrow().get_a(), self.bus.borrow_mut().get_cpu().borrow().get_a())), 
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("X: ${:02x} [{:02}]", self.bus.borrow_mut().get_cpu().borrow().get_x(), self.bus.borrow_mut().get_cpu().borrow().get_x())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Y: ${:02x} [{:02}]", self.bus.borrow_mut().get_cpu().borrow().get_y(), self.bus.borrow_mut().get_cpu().borrow().get_y())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Stack P: ${:04x}", self.bus.borrow_mut().get_cpu().borrow().get_stkp())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));

        // TODO: Implement disassembly

    }

    fn draw_code(&mut self, x: f32, y: f32, n_lines: i32, canvas: &mut ggez::graphics::Canvas)
    {
        let mut before_keys: Vec<_> = self.map_asm.range((Bound::Unbounded, Bound::Excluded(self.bus.borrow_mut().get_cpu().borrow().get_pc())))
                                                .rev()
                                                .take((n_lines / 2) as usize)
                                                .collect();

        before_keys.reverse();

        let after_keys: Vec<_> = self.map_asm.range((Bound::Excluded(self.bus.borrow_mut().get_cpu().borrow().get_pc()), Bound::Unbounded))
            .take((n_lines / 2) as usize)
            .collect();

        let mut num_offset: i32 = 0;
        for (_, value) in before_keys
        {
            canvas.draw(&Text::new(value), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset as f32)));
            num_offset += 1;
        }

        let inst = self.map_asm.get(&self.bus.borrow_mut().get_cpu().borrow().get_pc());
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


}

impl event::EventHandler<ggez::GameError> for MainState
{
    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::Space)
        {
            println!("clock!");
            
            loop
            {
                self.bus.borrow_mut().clock_tick();
                if self.bus.borrow_mut().get_cpu().borrow().complete()
                {
                    break;
                }
            }
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::R)
        {
            self.bus.borrow_mut().reset();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::I)
        {
            self.bus.borrow_mut().get_cpu().borrow_mut().irq();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::N)
        {
            self.bus.borrow_mut().get_cpu().borrow_mut().nmi();
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
        );

        MainState::draw_cpu_ram(self, 2, 2, 0x0000, 16, 16, &mut canvas);
        MainState::draw_cpu_ram(self, 2, 250, 0x8000, 16, 16, &mut canvas);
        MainState::draw_cpu(self, 475.0, 2.0, &mut canvas);
        MainState::draw_code(self, 475.0, 100.0, 27, &mut canvas);
        
        canvas.draw(&Text::new("SPACE = Step Instruction    R = RESET    I = IRQ    N = NMI"),
            Vec2::new(10.0, 500.0));

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult
{
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state);
}
