use bus::main_bus::MainBus;
use memory::ram::Ram;
use cpu::cpu6502::CPU6502;
use traits::ReadWrite;

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use ggez::glam::*;

pub mod traits;
pub mod bus;
pub mod memory;
pub mod cpu;

struct MainState<'a>
{
    pos_x: f32,
    bus: MainBus<'a>,
    ram: Ram,
    cpu: CPU6502<'a>
}

impl<'a> MainState<'a>
{
    fn new() -> GameResult<MainState<'a>>
    {
        let s = MainState
        {
            pos_x: 0.0,
            bus: MainBus::new(),
            ram: Ram::new(),
            cpu: CPU6502::new()
        };
        Ok(s)
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
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        canvas.draw(&circle, Vec2::new(self.pos_x, 380.0));

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
