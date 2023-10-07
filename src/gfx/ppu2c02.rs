use std::{cell::RefCell, rc::Rc};

use ggez::{graphics::{self, ImageFormat}, Context, GameResult};

use crate::{traits::{ReadWrite, Clockable}, cartridge::cart::Cart};

use ggez::glam::*;

use ggez::graphics::Drawable;

pub struct Ppu2c02
{
    cartridge: Option<Rc<RefCell<Cart>>>,
    nametables: [[u8; 1024]; 2],
    palettes: [u8; 32],
    frame_complete: bool,
    scan_line: i32,
    cycle: i32,
    debug: Ppu2c02Debug
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
            debug: Ppu2c02Debug::new(ctx)
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

    pub fn render(&mut self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas)
    {
        let image = graphics::Image::from_pixels(ctx, self.debug.screen_pixels.as_slice(), ImageFormat::Rgba8UnormSrgb, 256, 240);
        // let image = graphics::Image::from_color(ctx, 32, 32, Some(graphics::Color::WHITE));
        image.draw(canvas, Vec2::new(0.0, 0.0));
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
        let index: usize = (self.cycle - 1 * self.scan_line * 4) as usize;
        self.debug.screen_pixels[index] = 255;
        self.debug.screen_pixels[index + 1] = 255;
        self.debug.screen_pixels[index + 2] = 255;
        self.debug.screen_pixels[index + 4] = 255;

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

pub struct Ppu2c02Debug
{
    pal_colors: Box<[graphics::Color; 0x40]>,
    screen_pixels: Box<[u8; 256 * 240 * 4]>,
}

impl Ppu2c02Debug
{
    pub fn new(ctx: &Context) -> Self
    {
        let mut ret = Ppu2c02Debug
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
            

            screen_pixels: Box::new([0u8; 256 * 240 * 4])
            
            //name_table: [graphics::Canvas::from_image(ctx, graphics::Image::new_canvas_image(ctx, ImageFormat::Rgba8Unorm, 256, 240, 1), None),
            //            graphics::Canvas::from_image(ctx, graphics::Image::new_canvas_image(ctx, ImageFormat::Rgba8Unorm, 256, 240, 1), None)],

            //pattern_table: [graphics::Canvas::from_image(ctx, graphics::Image::new_canvas_image(ctx, ImageFormat::Rgba8Unorm, 128, 128, 1), None),
            //                graphics::Canvas::from_image(ctx, graphics::Image::new_canvas_image(ctx, ImageFormat::Rgba8Unorm, 128, 128, 1), None)]
        };

        let mut i = 3;
        while i < 256 * 240 * 4
        {
            ret.screen_pixels[i] = 255;
            i += 4;
        }

        return ret;

    }
}