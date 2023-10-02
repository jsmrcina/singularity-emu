use bus::main_bus::MainBus;
use memory::ram::Ram;
use cpu::cpu6502::CPU6502;
use traits::ReadWrite;
use std::cell::RefCell;
use std::rc::Rc;

use ggez::event;
use ggez::graphics::{self, Color};
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
    pos_x: f32,
    bus: MainBus,
    ram: Ram,
    cpu: CPU6502<'a>
}

impl<'a> MainState<'a>
{
    fn new() -> GameResult<MainState<'a>>
    {
        let mut s = MainState
        {
            pos_x: 0.0,
            bus: MainBus::new(),
            ram: Ram::new(),
            cpu: CPU6502::new()
        };

        s.cpu.set_bus(Some(Rc::new(RefCell::new(s.bus))));
        s.bus.add_system((0, 0xFFFF), "RAM".to_string(), &mut s.ram);

        Ok(s)
    }

    fn draw_ram(&mut self, x: i32, y: i32, mut nAddr: u16, nRows: i32, nCols: i32, canvas: &mut ggez::graphics::Canvas)
    {
        let nRamX: f32 = x as f32;
        let mut nRamY: f32 = y as f32;

        for _ in 0..nRows
        {
            let mut sOffset: String = format!("${:04x}:", nAddr);
            for _ in 0..nCols
            {
                sOffset = sOffset + &format!(" {:02x}", self.bus.read(nAddr));
                nAddr = nAddr + 1;
            }
            let text = Text::new(sOffset);
            canvas.draw(&text, Vec2::new(nRamX, nRamY));
            nRamY = nRamY + 10.0;
        }
    }
}

impl<'a> event::EventHandler<ggez::GameError> for MainState<'a>
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult
    {

        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult
    {

        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.0, 0.0, 1.0, 1.0]),
        );

        MainState::<'a>::draw_ram(self, 2, 2, 0x0000, 16, 16, &mut canvas);
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
