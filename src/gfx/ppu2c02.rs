use std::{cell::RefCell, rc::Rc};

use ggez::{graphics::{self, ImageFormat, Sampler}, Context, GameResult};

use crate::{traits::{ReadWrite, Clockable}, cartridge::cart::Cart};

use ggez::glam::*;

use ggez::graphics::Drawable;

use rand::Rng;

pub struct Ppu2c02
{
    cartridge: Option<Rc<RefCell<Cart>>>,
    nametables: [[u8; 1024]; 2],
    palettes: [u8; 32],
    frame_complete: bool,
    scan_line: i32,
    cycle: i32,
    renderer: Ppu2c02Renderer
}

impl Ppu2c02
{
    pub fn new(ctx: &Context) -> Self
    {
        let s = Ppu2c02
        {
            cartridge: None,
            nametables: [[0; 1024]; 2],
            palettes: [0; 32],
            frame_complete: false,
            scan_line: 0,
            cycle: 0,
            renderer: Ppu2c02Renderer::new(ctx)
        };

        return s;
    }

    pub fn connect_cartridge(&mut self, cartridge: Rc<RefCell<Cart>>)
    {
        self.cartridge = Some(cartridge);
    }

    pub fn frame_complete(&self) -> bool
    {
        return self.frame_complete;
    }

    pub fn set_frame_complete(&mut self, frame_complete: bool)
    {
        self.frame_complete = frame_complete;
    }

    // TODO: Move to renderer
    pub fn render(&mut self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas)
    {
        self.renderer.render(ctx, canvas);
    }

}

impl ReadWrite for Ppu2c02
{
    fn cpu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mirror_address = address & 0x7;

        match mirror_address
        {
            // Control
            0x0000 => return true,
            // Mask
            0x0001 => return true,
            // Status
            0x0002 => return true,
            // OAM Status
            0x0003 => return true,
            // OAM Data
            0x0004 => return true,
            // Scroll
            0x0005 => return true,
            // PPU Address
            0x0006 => return true,
            // PPU Data
            0x0007 => return true,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn cpu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mirror_address = address & 0x7;

        match mirror_address
        {
            // Control
            0x0000 => return true,
            // Mask
            0x0001 => return true,
            // Status
            0x0002 => return true,
            // OAM Status
            0x0003 => return true,
            // OAM Data
            0x0004 => return true,
            // Scroll
            0x0005 => return true,
            // PPU Address
            0x0006 => return true,
            // PPU Data
            0x0007 => return true,
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow_mut().ppu_write(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        return handled;
    }

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow().ppu_read(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        return handled;
    }
}

impl Clockable for Ppu2c02
{
    fn clock_tick(&mut self)
    {
        // Set the pixel to white
        let mut rng = rand::thread_rng();

        // println!("{:?} {:?}", self.scan_line, self.cycle);
        let num = rng.gen_range(1..=2);
        if num == 1
        {
            self.renderer.set_screen_pixel_to_color(self.renderer.pal_colors[0x30], self.scan_line, self.cycle - 1);
        }
        else
        {
            self.renderer.set_screen_pixel_to_color(self.renderer.pal_colors[0x3f], self.scan_line, self.cycle - 1);
        }

        self.cycle += 1;
        if self.cycle >= 341
        {
            self.cycle = 0;
            self.scan_line += 1;
            if self.scan_line >= 261
            {
                self.scan_line = -1;
                self.frame_complete = true;
            }
        }
    }
}

const PIXEL_DEPTH: usize = 4;
const SCREEN_ROWS: usize = 256;
const SCREEN_COLS: usize = 240;
const NAME_ROWS: usize = 256;
const NAME_COLS: usize = 240;
const PATTERN_ROWS: usize = 128;
const PATTERN_COLS: usize = 128;

pub struct Ppu2c02Renderer
{
    pal_colors: Box<[graphics::Color; 0x40]>,
    screen_pixels: Box<[u8; SCREEN_ROWS * SCREEN_COLS * PIXEL_DEPTH]>,
    name_table: [Box<[u8; NAME_ROWS * NAME_COLS * PIXEL_DEPTH]>; 2],
    pattern_table: [Box<[u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH]>; 2],
}

impl Ppu2c02Renderer
{
    pub fn new(ctx: &Context) -> Self
    {
        let mut ret = Ppu2c02Renderer
        {
            // Colors taken from NESDev wiki:
            // https://www.nesdev.org/wiki/PPU_palettes
            pal_colors: Box::new([
                graphics::Color::from_rgb(84, 84, 84),
                graphics::Color::from_rgb(0, 30, 116),
                graphics::Color::from_rgb(8, 16, 144),
                graphics::Color::from_rgb(48, 0, 136),
                graphics::Color::from_rgb(68, 0, 100),
                graphics::Color::from_rgb(92, 0, 48),
                graphics::Color::from_rgb(84, 4, 0),
                graphics::Color::from_rgb(60, 24, 0),
                graphics::Color::from_rgb(32, 42, 0),
                graphics::Color::from_rgb(8, 58, 0),
                graphics::Color::from_rgb(0, 64, 0),
                graphics::Color::from_rgb(0, 60, 0),
                graphics::Color::from_rgb(0, 50, 60),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
            
                graphics::Color::from_rgb(152, 150, 152),
                graphics::Color::from_rgb(8, 76, 196),
                graphics::Color::from_rgb(48, 50, 236),
                graphics::Color::from_rgb(92, 30, 228),
                graphics::Color::from_rgb(136, 20, 176),
                graphics::Color::from_rgb(160, 20, 100),
                graphics::Color::from_rgb(152, 34, 32),
                graphics::Color::from_rgb(120, 60, 0),
                graphics::Color::from_rgb(84, 90, 0),
                graphics::Color::from_rgb(40, 114, 0),
                graphics::Color::from_rgb(8, 124, 0),
                graphics::Color::from_rgb(0, 118, 40),
                graphics::Color::from_rgb(0, 102, 120),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
            
                graphics::Color::from_rgb(236, 238, 236),
                graphics::Color::from_rgb(76, 154, 236),
                graphics::Color::from_rgb(120, 124, 236),
                graphics::Color::from_rgb(176, 98, 236),
                graphics::Color::from_rgb(228, 84, 236),
                graphics::Color::from_rgb(236, 88, 180),
                graphics::Color::from_rgb(236, 106, 100),
                graphics::Color::from_rgb(212, 136, 32),
                graphics::Color::from_rgb(160, 170, 0),
                graphics::Color::from_rgb(116, 196, 0),
                graphics::Color::from_rgb(76, 208, 32),
                graphics::Color::from_rgb(56, 204, 108),
                graphics::Color::from_rgb(56, 180, 204),
                graphics::Color::from_rgb(60, 60, 60),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
            
                graphics::Color::from_rgb(236, 238, 236),
                graphics::Color::from_rgb(168, 204, 236),
                graphics::Color::from_rgb(188, 188, 236),
                graphics::Color::from_rgb(212, 178, 236),
                graphics::Color::from_rgb(236, 174, 236),
                graphics::Color::from_rgb(236, 174, 212),
                graphics::Color::from_rgb(236, 180, 176),
                graphics::Color::from_rgb(228, 196, 144),
                graphics::Color::from_rgb(204, 210, 120),
                graphics::Color::from_rgb(180, 222, 120),
                graphics::Color::from_rgb(168, 226, 144),
                graphics::Color::from_rgb(152, 226, 180),
                graphics::Color::from_rgb(160, 214, 228),
                graphics::Color::from_rgb(160, 162, 160),
                graphics::Color::from_rgb(0, 0, 0),
                graphics::Color::from_rgb(0, 0, 0),
            ]),
            

            screen_pixels: Box::new([0u8; SCREEN_ROWS * SCREEN_COLS * PIXEL_DEPTH]),
            name_table: [Box::new([0u8; NAME_ROWS * NAME_COLS * PIXEL_DEPTH]),  Box::new([0u8; NAME_ROWS * NAME_COLS * PIXEL_DEPTH])],
            pattern_table: [Box::new([0u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH]),  Box::new([0u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH])]
        };

        return ret;

    }

    pub fn render(&mut self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas)
    {
        let image = graphics::Image::from_pixels(ctx, self.screen_pixels.as_slice(), ImageFormat::Rgba8UnormSrgb, SCREEN_ROWS as u32, SCREEN_COLS as u32);

        let draw_params = graphics::DrawParam::new()
            .dest(Vec2::new(0.0, 0.0))
            .scale(Vec2::new(2.0, 2.0));
        
        canvas.set_sampler(Sampler::from(graphics::FilterMode::Nearest));
        image.draw(canvas, draw_params);
    }

    fn set_screen_pixel_to_color(&mut self, color: graphics::Color, row: i32, col: i32)
    {
        if row < 0 || row >= SCREEN_ROWS as i32 || col < 0 || col >= SCREEN_COLS as i32
        {
            // Do nothing
            return;
        }

        self.screen_pixels[(row as usize * SCREEN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH) + 0] = color.to_rgba().0;
        self.screen_pixels[(row as usize * SCREEN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH) + 1] = color.to_rgba().1;
        self.screen_pixels[(row as usize * SCREEN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH) + 2] = color.to_rgba().2;
        self.screen_pixels[(row as usize * SCREEN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH) + 3] = 255; // No alpha blending, always opaque
    }


}