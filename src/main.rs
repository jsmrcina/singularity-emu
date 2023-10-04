use bus::main_bus::MainBus;
use memory::ram::Ram;
use cpu::cpu6502::{CPU6502, Flags6502};
use traits::ReadWrite;
use std::cell::RefCell;
use std::rc::Rc;

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

struct MainState<'a>
{
    bus: Rc<RefCell<MainBus>>,
    ram: Rc<RefCell<Ram>>,
    cpu: Rc<RefCell<CPU6502<'a>>>
}

impl<'a> MainState<'a>
{
    fn new() -> GameResult<MainState<'a>>
    {
        let s = MainState
        {
            bus: Rc::new(RefCell::new(MainBus::new())),
            ram: Rc::new(RefCell::new(Ram::new())),
            cpu: Rc::new(RefCell::new(CPU6502::new()))
        };

        let ram_trait_object = Rc::clone(&s.ram) as Rc<RefCell<dyn ReadWrite>>;
        s.bus.borrow_mut().add_system((0, 0xFFFF), "RAM".to_string(), Some(ram_trait_object));

        let bus_trait_object = Rc::clone(&s.bus) as Rc<RefCell<dyn ReadWrite>>;
        s.cpu.borrow_mut().set_bus(Some(bus_trait_object));

        // Load some code into the emulator
        let code_str = String::from("A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA");
        let bytes: Vec<u8> = code_str
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).expect("Failed to parse hex string"))
            .collect();

        let mut offset: usize = 0;
        for b in bytes
        {
            s.ram.borrow_mut().buffer[offset] = b;
            offset += 1;
        }

        // Set the reset vector
        s.ram.borrow_mut().buffer[0xFFFC] = 0x00;
        s.ram.borrow_mut().buffer[0xFFFC] = 0x80;

        s.cpu.borrow_mut().reset();

        Ok(s)
    }

    const OFFSET_X: f32 = 16.0;
    const OFFSET_Y: f32 = 14.0;

    fn draw_ram(&mut self, x: i32, y: i32, mut n_addr: u16, n_rows: i32, n_cols: i32, canvas: &mut ggez::graphics::Canvas)
    {
        let n_ram_x: f32 = x as f32;
        let mut n_ram_y: f32 = y as f32;

        for _ in 0..n_rows
        {
            let mut s_offset: String = format!("${:04x}:", n_addr);
            for _ in 0..n_cols
            {
                s_offset = s_offset + &format!(" {:02x}", self.bus.borrow().read(n_addr));
                n_addr = n_addr + 1;
            }
            let text = Text::new(s_offset);
            canvas.draw(&text, Vec2::new(n_ram_x, n_ram_y));
            n_ram_y = n_ram_y + MainState::OFFSET_Y;
        }
    }

    fn draw_cpu(&mut self, x: f32, y: f32, canvas: &mut ggez::graphics::Canvas)
    {
        canvas.draw(&Text::new("Status"), Vec2::new(x, y));
        let mut num_offset: f32 = 0.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("N"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("V"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("-"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("B"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("D"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("I"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("Z"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        num_offset += 1.0;

        let color = if self.cpu.borrow().get_flag(Flags6502::N) == 1 { graphics::Color::GREEN } else { graphics::Color::RED };
        canvas.draw(&Text::new("C"), graphics::DrawParam::new().color(color).dest(Vec2::new(x + 64.0 + (MainState::OFFSET_X * num_offset), y)));
        
        num_offset = 1.0;

        canvas.draw(&Text::new(format!("PC: ${:04x}", self.cpu.borrow().get_pc())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("A: ${:02x} [{:02}]", self.cpu.borrow().get_a(), self.cpu.borrow().get_a())), 
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("X: ${:02x} [{:02}]", self.cpu.borrow().get_x(), self.cpu.borrow().get_x())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Y: ${:02x} [{:02}]", self.cpu.borrow().get_y(), self.cpu.borrow().get_y())),
            Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));
        num_offset += 1.0;

        canvas.draw(&Text::new(format!("Stack P: ${:04x}", self.cpu.borrow().get_stkp())), Vec2::new(x, y + (MainState::OFFSET_Y * num_offset)));

        // TODO: Implement disassembly

    }
}

impl<'a> event::EventHandler<ggez::GameError> for MainState<'a>
{
    fn update(&mut self, ctx: &mut Context) -> GameResult
    {
        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::Space)
        {
            while !self.cpu.borrow().complete()
            {
                self.cpu.borrow_mut().clock_tick();
            }
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::R)
        {
            self.cpu.borrow_mut().reset();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::I)
        {
            self.cpu.borrow_mut().irq();
        }

        if ctx.keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::N)
        {
            self.cpu.borrow_mut().nmi();
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
        );

        MainState::<'a>::draw_ram(self, 2, 2, 0x0000, 16, 16, &mut canvas);
        MainState::<'a>::draw_cpu(self, 475.0, 2.0, &mut canvas);
        
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
