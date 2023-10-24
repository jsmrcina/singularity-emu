use std::{cell::RefCell, rc::Rc};

use ggez::{graphics::{self, ImageFormat, Sampler}, Context};

use crate::{traits::{ReadWrite, Clockable}, cartridge::cart::Cart, cartridge::cart::MirrorMode};

use ggez::glam::*;

use ggez::graphics::Drawable;

use bitfield::bitfield;

bitfield! {
    struct StatusRegister(u8);
    impl Debug;
    // Define individual bits/sets of bits as boolean flags
    get_field, set_field: 7, 0;
    unused, set_unused: 0, 4;
    sprite_overflow, set_sprite_overflow: 5;
    sprite_zero_hit, set_sprite_zero_hit: 6;
    vertical_blank, set_vertical_blank: 7;
}

bitfield! {
    struct MaskRegister(u8);
    impl Debug;
    // Define individual bits/sets of bits as boolean flags
    get_field, set_field: 7, 0;
    grayscale, set_grayscale : 0;
    render_background_left, set_render_background_left : 1;
    render_sprites_left, set_render_sprites_left : 2;
    render_background, set_render_background : 3;
    render_sprites, set_render_sprites : 4;
    enhance_red, set_enhance_red : 5;
    enhance_green, set_enhance_green : 6;
    enhance_blue, set_enhance_blue : 7;
}

bitfield! {
    struct CtrlRegister(u8);
    impl Debug;
    // Define individual bits/sets of bits as boolean flags
    get_field, set_field: 7, 0;
    nametable_x, set_nametable_x : 0;
    nametable_y, set_nametable_y : 1;
    increment_mode, set_increment_mode : 2;
    pattern_sprite, set_pattern_sprite : 3;
    pattern_background, set_pattern_background : 4;
    sprite_size, set_sprite_size : 5;
    slave_mode, set_slave_mode : 6; // unused
    enable_nmi, set_enable_nmi : 7;
}

// Uses the loopy register from NesDEV
bitfield! {
    struct LoopyRegister(u16);
    impl Debug;
    get_field, set_field: 15, 0;
    coarse_x, set_coarse_x: 4, 0;
    coarse_y, set_coarse_y: 9, 5;
    name_table_x, set_name_table_x: 10;
    name_table_y, set_name_table_y: 11;
    fine_y, set_fine_y: 14, 12;
    unused, set_unused: 15;
}

// TODO: Is there a more RUST-y way to do this?
#[derive(Default, Copy, Clone)]
struct ObjectAttributeEntry
{
    y: u8,
    id: u8,
    attribute: u8,
    x: u8
}

struct BgNextTileInfo
{
    id: u8,
    attrib: u8,
    lsb: u8,
    msb: u8
}

struct BgShifterInfo
{
    pattern_lo: u16,
    pattern_hi: u16,
    attrib_lo: u16,
    attrib_hi: u16
}

struct FgShifterInfo
{
    pattern_lo: [u8; 8],
    pattern_hi: [u8; 8]
}

pub struct Ppu2c02
{
    cartridge: Option<Rc<RefCell<Cart>>>,
    patterns: Box<[[u8; 4096]; 2]>,
    nametables: Box<[[u8; 1024]; 2]>,
    palettes: [u8; 32],
    oam: [ObjectAttributeEntry; 64],
    sprite_scanline: [ObjectAttributeEntry; 8],
    sprite_count: u8,
    frame_complete: bool,
    scan_line: i32,
    cycle: i32,
    renderer: Ppu2c02Renderer,
    status: StatusRegister,
    mask: MaskRegister,
    ctrl: CtrlRegister,
    address_latch: bool,
    vram_addr: LoopyRegister,
    tram_addr: LoopyRegister,
    fine_x: u8,
    ppu_data_buffer: u8,
    nmi: bool,
    bg_next_info: BgNextTileInfo,
    bg_shifter_info: BgShifterInfo,
    oam_addr: u8,
    fg_shifter_info: FgShifterInfo,
    sprite_zero_hit_possible: bool,
    sprite_zero_being_rendered: bool
}

impl Ppu2c02
{
    pub fn new() -> Self
    {
        let s = Ppu2c02
        {
            cartridge: None,
            patterns: Box::new([[0u8; 4096]; 2]),
            nametables: Box::new([[0u8; 1024]; 2]),
            palettes: [0u8; 32],
            oam: [ObjectAttributeEntry { y: 0, id: 0, attribute: 0, x: 0 }; 64],
            sprite_scanline: [ObjectAttributeEntry { y: 0, id: 0, attribute: 0, x: 0 }; 8],
            sprite_count: 0,
            frame_complete: false,
            scan_line: 0,
            cycle: 0,
            renderer: Ppu2c02Renderer::new(),
            status: StatusRegister(0),
            mask: MaskRegister(0),
            ctrl: CtrlRegister(0),
            address_latch: false,
            vram_addr: LoopyRegister(0),
            tram_addr: LoopyRegister(0),
            fine_x: 0,
            ppu_data_buffer: 0x00,
            nmi: false,
            bg_next_info: BgNextTileInfo { id: 0x00, attrib: 0x00, lsb: 0x00, msb: 0x00 },
            bg_shifter_info: BgShifterInfo { pattern_lo: 0x0000, pattern_hi: 0x0000, attrib_lo: 0x0000, attrib_hi: 0x0000 },
            oam_addr: 0,
            fg_shifter_info: FgShifterInfo { pattern_lo: [0; 8] , pattern_hi: [0; 8] },
            sprite_zero_hit_possible: false,
            sprite_zero_being_rendered: false
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

    pub fn get_nmi(&self) -> bool
    {
        return self.nmi;
    }

    pub fn set_nmi(&mut self, nmi: bool)
    {
        self.nmi = nmi
    }

    pub fn get_cycle(&self) -> i32
    {
        return self.cycle;
    }

    pub fn get_scan_line(&self) -> i32
    {
        return self.scan_line;
    }

    pub fn set_oam_memory_at_addr(&mut self, addr: u8, data: u8)
    {
        unsafe
        {
            let oam_as_u8_ptr: &mut [u8] = std::slice::from_raw_parts_mut(
                self.oam.as_mut_ptr() as *mut u8,
                self.oam.len() * std::mem::size_of::<ObjectAttributeEntry>()
            );

            oam_as_u8_ptr[addr as usize] = data;
        }
    }

    pub fn get_oam_memory_at_addr(&mut self, addr: u8) -> u8
    {
        unsafe
        {
            let oam_as_u8_ptr: &mut [u8] = std::slice::from_raw_parts_mut(
                self.oam.as_mut_ptr() as *mut u8,
                self.oam.len() * std::mem::size_of::<ObjectAttributeEntry>()
            );

            return oam_as_u8_ptr[addr as usize];
        }
    }

    fn clear_scanline_memory(&mut self)
    {
        unsafe
        {
            let sprite_scanline_as_ptr: &mut [u8] = std::slice::from_raw_parts_mut(
                self.sprite_scanline.as_mut_ptr() as *mut u8,
                self.sprite_scanline.len() * std::mem::size_of::<ObjectAttributeEntry>()
            );

            for i in 0..sprite_scanline_as_ptr.len()
            {
                sprite_scanline_as_ptr[i] = 0xFF;
            }
        }

        self.sprite_count = 0;
    }

    pub fn render(&mut self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas, palette_id: u8)
    {
        self.prepare_pattern_table(0, palette_id);
        self.prepare_pattern_table(1, palette_id);
        self.renderer.render(ctx, canvas);
    }

    pub fn get_color_from_palette_ram(&mut self, palette_id: u8, pixel: u8) -> graphics::Color
    {
        const PALETTE_MEMORY_START: u16 = 0x3f00;
        let palette_address = PALETTE_MEMORY_START + (palette_id << 2) as u16 + (pixel as u16);

        // Read the color ID from the palette
        let mut palette_data: u8 = 0; 
        self.ppu_read(palette_address, &mut palette_data);
        
        // We've stored all the colors into our pal_colors array so we can just directly index it.
        // Avoid an overrun
        return self.renderer.pal_colors[(palette_data & 0x3F) as usize];
    }

    // pattern_index is either 0 or 1 depending on whether we're reading from the left or right side of the pattern table
    pub fn prepare_pattern_table(&mut self, pattern_index: u16, palette_id: u8)
    {        
        const TILE_SIZE_IN_BYTES: u16 = 16;
        const TABLE_ROW_SIZE_IN_BYTES: u16 = TILE_SIZE_IN_BYTES * 16;

        const TILE_TOTAL_BYTES: u16 = 8;
        const TILE_ROW_BITS: u16 = 8;
        const TILE_COL_BITS: u16 = 8;

        const PATTERN_TABLE_HALF_SIZE: u16 = 0x1000; // 4kb

        // Iterate through 16x16 tiles (each of size 16 bytes) of one half of the pattern memory
        for t_y in 0..TILE_SIZE_IN_BYTES
        {
            for t_x in 0..TILE_SIZE_IN_BYTES
            {
                let offset: u16 = t_y * TABLE_ROW_SIZE_IN_BYTES + t_x * TILE_SIZE_IN_BYTES;

                // A single tile is 8 bits across and 8 cols across
                for row in 0..TILE_ROW_BITS
                {
                    let mut tile_lsb: u8 = 0;
                    self.ppu_read(pattern_index * PATTERN_TABLE_HALF_SIZE + offset + row + 0, &mut tile_lsb);
                    let mut tile_msb: u8 = 0;
                    self.ppu_read(pattern_index * PATTERN_TABLE_HALF_SIZE + offset + row + TILE_TOTAL_BYTES, &mut tile_msb);

                    for col in 0..TILE_COL_BITS
                    {
                        // Each pixel color is determined by the sum of the lowest two bits
                        // This will always give a value between [0, 3]
                        let pixel: u8 = (tile_lsb & 0x01) + (tile_msb & 0x01);

                        // Then we shift through the bits of that byte
                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        let color = self.get_color_from_palette_ram(palette_id, pixel);
                        self.renderer.set_pixel_to_color(Surface::Pattern, pattern_index as usize, color,
                                (t_y * 8 + row) as i32,
                                (t_x * 8 + (7 - col)) as i32); // TODO: Look at this again, (7 - col) is done because we are drawing right to left
                    }
                }
            }
        }
    }

    // Utility closure for manipulating the loopy register on specific scan_line and cycle changes
    pub fn increment_scroll_x(&mut self)
    {
        if self.mask.render_background() || self.mask.render_sprites()
        {
            if self.vram_addr.coarse_x() == 31
            {
                // Wrap coarse X
                self.vram_addr.set_coarse_x(0);

                // Flip the nametable bit
                self.vram_addr.set_name_table_x(!self.vram_addr.name_table_x());
            }
            else
            {
                self.vram_addr.set_coarse_x(self.vram_addr.coarse_x() + 1);
            }
        }
    }

    pub fn increment_scroll_y(&mut self)
    {
        if self.mask.render_background() || self.mask.render_sprites()
        {
            if self.vram_addr.fine_y() < 7
            {
                self.vram_addr.set_fine_y(self.vram_addr.fine_y() + 1);
            }
            else
            {
                // We've crossed a tile boundary
                self.vram_addr.set_fine_y(0);

                // Check to see if we crossed into a new nametable (note that since the bottom
                // of the nametable is used for pallete (attribute) info, we only have 29 rows
                if self.vram_addr.coarse_y() == 29
                {
                    self.vram_addr.set_coarse_y(0);
                    self.vram_addr.set_name_table_y(!self.vram_addr.name_table_y());
                }
                else if self.vram_addr.coarse_y() == 31
                {
                    self.vram_addr.set_coarse_y(0);
                }
                else
                {
                    self.vram_addr.set_coarse_y(self.vram_addr.coarse_y() + 1);
                }
            }
        }
    }

    pub fn transfer_address_x(&mut self)
    {
        if self.mask.render_background() || self.mask.render_sprites()
        {
            self.vram_addr.set_name_table_x(self.tram_addr.name_table_x());
            self.vram_addr.set_coarse_x(self.tram_addr.coarse_x());
        }
    }

    pub fn transfer_address_y(&mut self)
    {
        if self.mask.render_background() || self.mask.render_sprites()
        {
            self.vram_addr.set_fine_y(self.tram_addr.fine_y());
            self.vram_addr.set_name_table_y(self.tram_addr.name_table_y());
            self.vram_addr.set_coarse_y(self.tram_addr.coarse_y());
        }
    }

    pub fn load_background_shifters(&mut self)
    {
        self.bg_shifter_info.pattern_lo = (self.bg_shifter_info.pattern_lo & 0xFF00) | (self.bg_next_info.lsb as u16);
        self.bg_shifter_info.pattern_hi = (self.bg_shifter_info.pattern_hi & 0xFF00) | (self.bg_next_info.msb as u16);

        if self.bg_next_info.attrib & 0b01 == 0b01
        {
            self.bg_shifter_info.attrib_lo = (self.bg_shifter_info.attrib_lo & 0xFF00) | 0xFF;
        }
        else
        {
            self.bg_shifter_info.attrib_lo = (self.bg_shifter_info.attrib_lo & 0xFF00) | 0x00;
        }

        if self.bg_next_info.attrib & 0b10 == 0b10
        {
            self.bg_shifter_info.attrib_hi = (self.bg_shifter_info.attrib_hi & 0xFF00) | 0xFF;
        }
        else
        {
            self.bg_shifter_info.attrib_hi = (self.bg_shifter_info.attrib_hi & 0xFF00) | 0x00;
        }   
    }

    pub fn update_shifters(&mut self)
    {
        if self.mask.render_background()
        {
            self.bg_shifter_info.pattern_lo <<= 1;
            self.bg_shifter_info.pattern_hi <<= 1;
            self.bg_shifter_info.attrib_lo <<= 1;
            self.bg_shifter_info.attrib_hi <<= 1;
        }

        if self.mask.render_sprites() && self.cycle >= 1 && self.cycle < 258
        {
            for i in 0..self.sprite_count
            {
                if self.sprite_scanline[i as usize].x > 0
                {
                    self.sprite_scanline[i as usize].x -= 1;
                }
                else
                {
                    self.fg_shifter_info.pattern_lo[i as usize] <<= 1;
                    self.fg_shifter_info.pattern_hi[i as usize] <<= 1;
                }
            }
        }
    }

    pub fn get_name_table(&self) -> Box<[[u8; 1024]; 2]>
    {
        return self.nametables.clone();
    }

    pub fn reset(&mut self)
    {
        self.ctrl.set_field(0);
        self.mask.set_field(0);
        self.status.set_field(self.status.get_field() & 0x80); // Keep top bit set
        self.address_latch = false;
        self.ppu_data_buffer = 0;
        self.frame_complete = false;
        self.scan_line = 0;
        self.cycle = 0;
        self.fine_x = 0;
        self.bg_next_info = BgNextTileInfo { attrib: 0, id: 0, lsb: 0, msb: 0 };
        self.bg_shifter_info = BgShifterInfo { attrib_hi: 0, attrib_lo: 0, pattern_hi: 0, pattern_lo: 0 };
        self.vram_addr.set_field(0);
        self.tram_addr.set_field(0);
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
            0x0000 =>
            {
                self.ctrl.set_field(data);
                self.tram_addr.set_name_table_x(self.ctrl.nametable_x());
                self.tram_addr.set_name_table_y(self.ctrl.nametable_y());
                return true;
            }
            // Mask
            0x0001 =>
            {
                self.mask.set_field(data);
                return true;
            }
            // Status
            0x0002 => return true,
            // OAM Address
            0x0003 =>
            {
                self.oam_addr = data;
                return true;
            }
            // OAM Data
            0x0004 =>
            {
                self.set_oam_memory_at_addr(self.oam_addr, data);
                return true;
            }
            // Scroll
            0x0005 =>
            {
                if !self.address_latch
                {
                    self.fine_x = data & 0x07;
                    self.tram_addr.set_coarse_x((data >> 3) as u16);
                    self.address_latch = true;
                }
                else
                {
                    self.tram_addr.set_fine_y((data & 0x07) as u16);
                    self.tram_addr.set_coarse_y((data >> 3) as u16);
                    self.address_latch = false;
                }

                return true;
            },
            // PPU Address
            0x0006 =>
            {
                if !self.address_latch
                {
                    self.tram_addr.set_field((self.tram_addr.get_field() & 0x00FF) | (((data as u16) & 0x3F) << 8));
                    self.address_latch = true;
                }
                else
                {
                    self.tram_addr.set_field((self.tram_addr.get_field() & 0xFF00) | (data as u16));
                    self.vram_addr.set_field(self.tram_addr.get_field());
                    self.address_latch = false;
                }
                return true;
            }
            // PPU Data
            0x0007 =>
            {
                self.ppu_write(self.vram_addr.get_field(), data);
                if self.ctrl.increment_mode()
                {
                    self.vram_addr.set_field(self.vram_addr.get_field() + 32); // Increment a whole row
                }
                else
                {
                    self.vram_addr.set_field(self.vram_addr.get_field() + 1);
                }
                return true;
            }
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn cpu_read(&mut self, address: u16, data: &mut u8) -> bool
    {
        let mirror_address = address & 0x7;

        match mirror_address
        {
            // Control
            0x0000 =>
            {
                *data = 0;
                 return true
            },
            // Mask
            0x0001 =>
            {
                *data = 0;
                 return true
            },
            // Status
            0x0002 =>
            {
                // The top bits are all that is returned, the bottom bits get noise from the
                // last read (probably not strictly necessary)
                *data = (self.status.get_field() & 0xE0) | (self.ppu_data_buffer & 0x1F);
                
                self.status.set_vertical_blank(false);
                self.address_latch = false;

                return true;
            }
            // OAM Status
            0x0003 => return true,
            // OAM Data
            0x0004 =>
            {
                *data = self.get_oam_memory_at_addr(self.oam_addr);
                return true;
            }
            // Scroll
            0x0005 => return true,
            // PPU Address
            0x0006 => return true,
            // PPU Data
            0x0007 =>
            {
                // Reading most memory has a 1-read delay
                *data = self.ppu_data_buffer;
                let mut temp_ppu_data_buffer: u8 = 0;
                self.ppu_read(self.vram_addr.get_field(), &mut temp_ppu_data_buffer);
                self.ppu_data_buffer = temp_ppu_data_buffer;

                // Palette memory has no delay, so special case it here
                // TODO: Make constants from the memory ranges
                if self.vram_addr.get_field() >= 0x3F00
                {
                    *data = self.ppu_data_buffer;
                }

                if self.ctrl.increment_mode()
                {
                    self.vram_addr.set_field(self.vram_addr.get_field() + 32); // Increment a whole row
                }
                else
                {
                    self.vram_addr.set_field(self.vram_addr.get_field() + 1);
                }

                return true;
            }
            _ => panic!("Non addressable memory in PPU accessed during CPU read")
        }
    }

    fn ppu_write(&mut self, address: u16, data: u8) -> bool
    {
        let mut mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow_mut().ppu_write(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        if !handled
        {
            if mut_addr <= 0x1FFF
            {
                // Calculate the pattern half index (0 or 1 depending on whether the MSB is set or not)
                let pattern_half_index = (mut_addr & 0x1000) >> 12;
                let pattern_index = mut_addr & 0x0FFF; 
                self.patterns[pattern_half_index as usize][pattern_index as usize] = data;
            }
            else if mut_addr >= 0x2000 && mut_addr < 0x3EFF
            {
                mut_addr &= 0x0FFF;

                match &self.cartridge
                {
                    Some(x) =>
                    {
                        let mirror_mode = x.borrow().get_mirror_mode();
                        match mirror_mode
                        {
                            MirrorMode::Vertical =>
                            {
                                if mut_addr <= 0x03FF
                                {
                                    self.nametables[0][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0400 && mut_addr <= 0x07FF
                                {
                                    self.nametables[1][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0800 && mut_addr <= 0x0BFF
                                {
                                    self.nametables[0][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0C00 && mut_addr <= 0x0FFF
                                {
                                    self.nametables[1][(mut_addr & 0x03FF) as usize] = data;
                                }
                            },
                            MirrorMode::Horizontal =>
                            {
                                if mut_addr <= 0x03FF
                                {
                                    self.nametables[0][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0400 && mut_addr <= 0x07FF
                                {
                                    self.nametables[0][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0800 && mut_addr <= 0x0BFF
                                {
                                    self.nametables[1][(mut_addr & 0x03FF) as usize] = data;
                                }
                                else if mut_addr >= 0x0C00 && mut_addr <= 0x0FFF
                                {
                                    self.nametables[1][(mut_addr & 0x03FF) as usize] = data;
                                }
                            },
                            _ => panic!("Unimplemented mirror mode accessed")
                        }
                    },
                    None => panic!("No cartridge inserted when querying mirror mode")
                }
            }
            else if mut_addr >= 0x3F00 && mut_addr <= 0x3FFF
            {
                const BOTTOM_5_BITS_MASK: u16 = 0x001F;
                let mut address_masked = mut_addr & BOTTOM_5_BITS_MASK;

                // The 4 palettes are mirrored 
                if address_masked == 0x0010 { address_masked = 0x0000; }
                if address_masked == 0x0014 { address_masked = 0x0004; }
                if address_masked == 0x0018 { address_masked = 0x0008; }
                if address_masked == 0x001C { address_masked = 0x000C; }
                self.palettes[address_masked as usize] = data;
            }
            else
            {
                panic!("Unhandled PPU write at address {:?}", address);
            }
        }

        return handled;
    }

    fn ppu_read(&self, address: u16, data: &mut u8) -> bool
    {
        let mut mut_addr = address & 0x3FFF;
        let handled;

        match &self.cartridge
        {
            Some(x) =>
            {
                handled = x.borrow().ppu_read(mut_addr, data)
            },
            None => panic!("No cartridge inserted, PPU tried to read")
        };

        if !handled
        {
            // The rest of the memory is going to be handled by the various
            // arrays of memory stored in the PPU
            if mut_addr <= 0x1FFF
            {
                // This should be handled by the cartridge

                // Calculate the pattern half index (0 or 1 depending on whether its the first 4k or not)
                let pattern_half_index = (mut_addr & 0x1000) >> 12;
                let pattern_index = mut_addr & 0x0FFF; 
                *data = self.patterns[pattern_half_index as usize][pattern_index as usize];
            }
            else if mut_addr >= 0x2000 && mut_addr < 0x3EFF
            {
                mut_addr &= 0x0FFF;

                match &self.cartridge
                {
                    Some(x) =>
                    {
                        let mirror_mode = x.borrow().get_mirror_mode();
                        match mirror_mode
                        {
                            MirrorMode::Vertical =>
                            {
                                if mut_addr <= 0x03FF
                                {
                                    *data = self.nametables[0][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0400 && mut_addr <= 0x07FF
                                {
                                    *data = self.nametables[1][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0800 && mut_addr <= 0x0BFF
                                {
                                    *data = self.nametables[0][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0C00 && mut_addr <= 0x0FFF
                                {
                                    *data = self.nametables[1][(mut_addr & 0x03FF) as usize];
                                }
                            },
                            MirrorMode::Horizontal =>
                            {
                                if mut_addr <= 0x03FF
                                {
                                    *data = self.nametables[0][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0400 && mut_addr <= 0x07FF
                                {
                                    *data = self.nametables[0][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0800 && mut_addr <= 0x0BFF
                                {
                                    *data = self.nametables[1][(mut_addr & 0x03FF) as usize];
                                }
                                else if mut_addr >= 0x0C00 && mut_addr <= 0x0FFF
                                {
                                    *data = self.nametables[1][(mut_addr & 0x03FF) as usize];
                                }
                            },
                            _ => panic!("Unimplemented mirror mode accessed")

                        }
                    },
                    None => panic!("No cartridge inserted when querying mirror mode")
                }
            }
            else if mut_addr >= 0x3F00 && mut_addr <= 0x3FFF
            {
                const BOTTOM_5_BITS_MASK: u16 = 0x001F;
                let mut address_masked = mut_addr & BOTTOM_5_BITS_MASK;

                // The 4 palettes are mirrored 
                if address_masked == 0x0010 { address_masked = 0x0000; }
                if address_masked == 0x0014 { address_masked = 0x0004; }
                if address_masked == 0x0018 { address_masked = 0x0008; }
                if address_masked == 0x001C { address_masked = 0x000C; }

                if self.mask.grayscale()
                {
                    *data = self.palettes[address_masked as usize] & 0x30;

                }
                else
                {
                    *data = self.palettes[address_masked as usize] & 0x3F;
                }

            }
            else
            {
                panic!("Unhandled PPU read at address {:?}", address);
            }
        }

        return handled;
    }
}

impl Clockable for Ppu2c02
{
    fn clock_tick(&mut self)
    {
        // Based on the table from NesDev which details what clock ticks perform what operations
        // at what scanelines and cycles: https://www.nesdev.org/wiki/PPU_rendering
        if self.scan_line >= -1 && self.scan_line < 240
        {
            if self.scan_line == 0 && self.cycle == 0
            {
                self.cycle = 1;
            }

            if self.scan_line == -1 && self.cycle == 1
            {
                self.status.set_vertical_blank(false);
                self.status.set_sprite_zero_hit(false);
                self.status.set_sprite_overflow(false);

                for i in 0..8
                {
                    self.fg_shifter_info.pattern_lo[i as usize] = 0;
                    self.fg_shifter_info.pattern_hi[i as usize] = 0;
                }
            }

            // Visible cycles
            if (self.cycle >= 2 && self.cycle < 258) ||
                    (self.cycle >= 321 && self.cycle < 338)
            {
                self.update_shifters();

                // TODO: Read more about this shifty stuff
                match (self.cycle - 1) % 8
                {
                    0 =>
                    {
                        self.load_background_shifters();

                        let mut id: u8 = 0;
                        self.ppu_read(0x2000 | (self.vram_addr.get_field() & 0x0FFF), &mut id);
                        self.bg_next_info.id = id;
                    },
                    2 =>
                    {
                        let mut attrib: u8 = 0;
                        let addr: u16 = 0x23C0 | ((self.vram_addr.name_table_y() as u16) << 11)
                            | ((self.vram_addr.name_table_x() as u16) << 10)
                            | (((self.vram_addr.coarse_y() >> 2) << 3)
                            | (self.vram_addr.coarse_x() >> 2));

                        self.ppu_read(addr, &mut attrib);

                        self.bg_next_info.attrib = attrib;

                        if self.vram_addr.coarse_y() & 0x02 > 0
                        {
                            self.bg_next_info.attrib >>= 4;
                        }
                        if self.vram_addr.coarse_x() & 0x02 > 0
                        {
                            self.bg_next_info.attrib >>= 2;
                        }
                        self.bg_next_info.attrib &= 3;
                    },
                    4 =>
                    {
                        let mut lsb: u8 = 0;
                        let addr: u16 = ((self.ctrl.pattern_background() as u16) << 12)
                            | ((self.bg_next_info.id as u16) << 4)
                            | (self.vram_addr.fine_y() + 0);
                        self.ppu_read(addr, &mut lsb);
                        self.bg_next_info.lsb = lsb;
                    }
                    6 =>
                    {
                        let mut msb: u8 = 0;
                        let addr: u16 = ((self.ctrl.pattern_background() as u16) << 12)
                            | ((self.bg_next_info.id as u16) << 4)
                            | (self.vram_addr.fine_y() + 8);
                        self.ppu_read(addr, &mut msb);
                        self.bg_next_info.msb = msb;
                    },
                    7 =>
                    {
                        self.increment_scroll_x();
                    },
                    _ => () // Do nothing for certain cycles
                }
            }

            if self.cycle == 256
            {
                self.increment_scroll_y();
            }

            if self.cycle == 257
            {
                self.load_background_shifters();
                self.transfer_address_x();
            }

            if self.scan_line == -1 && self.cycle >= 280 && self.cycle < 305
            {
                self.transfer_address_y();
            }

            // These are superfluous, but technically in the implementation
            if self.cycle == 338 || self.cycle == 340
            {
                let mut id = 0;
                self.ppu_read(0x2000 | (self.vram_addr.get_field() & 0x0FFF), &mut id);
                self.bg_next_info.id = id;
            }

            // Foreground rendering
            if self.cycle == 257 && self.scan_line >= 0
            {
                self.clear_scanline_memory();

                let mut oam_entry: u8 = 0;
                self.sprite_zero_hit_possible = false;
                while oam_entry < 64 && self.sprite_count < 9
                {
                    let diff: i16 = self.scan_line as i16 - self.oam[oam_entry as usize].y as i16;
                    let mut sprite_size = 8;
                    if self.ctrl.sprite_size()
                    {
                        sprite_size = 16;
                    }

                    if diff >= 0 && diff < sprite_size
                    {
                        if self.sprite_count < 8
                        {
                            // Update whether sprite zero hit is possible
                            if oam_entry == 0
                            {
                                self.sprite_zero_hit_possible = true;
                            }

                            self.sprite_scanline[self.sprite_count as usize] = self.oam[oam_entry as usize];
                            self.sprite_count += 1;
                        }
                    }

                    oam_entry += 1;
                }
                
                if self.sprite_count > 8
                {
                    self.status.set_sprite_overflow(true);
                }
                else
                {
                    self.status.set_sprite_overflow(false);
                }
            }

            if self.cycle == 340
            {
                for i in 0..self.sprite_count
                {
                    let index = i as usize;
                    let mut sprite_pattern_bits_lo: u8 = 0;
                    let mut sprite_pattern_bits_hi: u8 = 0;
                    let sprite_pattern_addr_lo: u16;
                    let sprite_pattern_addr_hi: u16;

                    if !self.ctrl.sprite_size()
                    {
                        // 8x8 - control register determines pattern table
                        if self.sprite_scanline[index].attribute & 0x80 != 0x80
                        {
                            // Not flipped vertically
                            sprite_pattern_addr_lo =
                                ((self.ctrl.pattern_sprite() as u16) << 12) |                               // Which pattern table (0 or 4kb)
                                ((self.sprite_scanline[index].id as u16) << 4) |                            // Which cell, Tile ID * 16 (16 bytes per tile)
                                (self.scan_line as u16 - self.sprite_scanline[index].y as u16);             // Which row in the cell, 0 to 7
                        }
                        else
                        {
                            // Flipped vertically
                            sprite_pattern_addr_lo =
                                ((self.ctrl.pattern_sprite() as u16) << 12) |                               // Which pattern table (0 or 4kb)
                                ((self.sprite_scanline[index].id as u16) << 4) |                            // Which cell, Tile ID * 16 (16 bytes per tile)
                                (7 - (self.scan_line as u16 - self.sprite_scanline[index].y as u16));       // Which row in the cell, 7 to 0
                        }
                    }
                    else
                    {
                        // 8x16 - sprite attribute determines pattern table 
                        if self.sprite_scanline[index].attribute & 0x80 != 0x80
                        {
                            // Not flipped vertically
                            if self.scan_line as u8 - self.sprite_scanline[index].y < 8
                            {
                                // Top half of the sprite
                                sprite_pattern_addr_lo =
                                    (((self.sprite_scanline[index].id & 0x01) as u16) << 12) |              // Which pattern table (0 or 4kb)
                                    ((self.sprite_scanline[index].id & 0xFE) as u16) << 4 |                 // Which cell, Tile ID * 16 (16 bytes per tile)
                                    (self.scan_line as u16 - self.sprite_scanline[index].y as u16) & 0x07;  // Which row in the cell, 0 to 7
                            }
                            else
                            {
                                sprite_pattern_addr_lo =
                                    (((self.sprite_scanline[index].id & 0x01) as u16) << 12) |              // Which pattern table (0 or 4kb)
                                    ((self.sprite_scanline[index].id & 0xFE) as u16 + 1) << 4 |             // Which cell, Tile ID * 16 (16 bytes per tile)
                                    (self.scan_line as u16 - self.sprite_scanline[index].y as u16) & 0x07;  // Which row in the cell, 0 to 7
                            }
                        }
                        else
                        {
                            // Flipped vertically
                            if self.scan_line as u8 - self.sprite_scanline[index].y < 8
                            {
                                // Top half of the sprite
                                sprite_pattern_addr_lo =
                                    (((self.sprite_scanline[index].id & 0x01) as u16) << 12) |              // Which pattern table (0 or 4kb)
                                    ((self.sprite_scanline[index].id & 0xFE + 1) as u16) << 4 |                 // Which cell, Tile ID * 16 (16 bytes per tile)
                                    (7 - (self.scan_line as u16 - self.sprite_scanline[index].y as u16)) & 0x07;  // Which row in the cell, 0 to 7
                            }
                            else
                            {
                                sprite_pattern_addr_lo =
                                    (((self.sprite_scanline[index].id & 0x01) as u16) << 12) |              // Which pattern table (0 or 4kb)
                                    ((self.sprite_scanline[index].id & 0xFE) as u16) << 4 |                 // Which cell, Tile ID * 16 (16 bytes per tile)
                                    (7 - (self.scan_line as u16 - self.sprite_scanline[index].y as u16)) & 0x07;  // Which row in the cell, 0 to 7
                            }
                        }
                    }

                    sprite_pattern_addr_hi = sprite_pattern_addr_lo + 8;
                    self.ppu_read(sprite_pattern_addr_lo, &mut sprite_pattern_bits_lo);
                    self.ppu_read(sprite_pattern_addr_hi, &mut sprite_pattern_bits_hi);

                    if self.sprite_scanline[index].attribute & 0x40 == 0x40
                    {
                        let flip_byte = |b: u8| -> u8 {
                            let mut r: u8 = b;
                            r = (r & 0xF0) >> 4 | (r & 0x0F) << 4;
                            r = (r & 0xCC) >> 4 | (r & 0x33) << 2;
                            r = (r & 0xAA) >> 1 | (r & 0x55) << 1;
                            return r;
                        };

                        sprite_pattern_bits_lo = flip_byte(sprite_pattern_bits_lo);
                        sprite_pattern_bits_hi = flip_byte(sprite_pattern_bits_hi);
                    }

                    self.fg_shifter_info.pattern_lo[index] = sprite_pattern_bits_lo;
                    self.fg_shifter_info.pattern_hi[index] = sprite_pattern_bits_hi;
                }
            }
        }

        if self.scan_line == 240
        {
            // Nothing happens
        }

        if self.scan_line >= 241 && self.scan_line < 261
        {
            if self.scan_line == 241 && self.cycle == 1
            {
                self.status.set_vertical_blank(true);
                if self.ctrl.enable_nmi()
                {
                    self.nmi = true;
                }
            }
        }

        // Put together all the info we currently have for a pixel
        let mut bg_pixel: u8 = 0;
        let mut bg_palette: u8 = 0;

        if self.mask.render_background()
        {
            // Shift by fine X since we might be partway scrolled into a given background
            // sprite
            let bit_mux: u16 = 0x8000 >> self.fine_x;

            // Composite the two pixels infos into two pixel planes
            let mut p0_pixel: u8 = 0;
            if (self.bg_shifter_info.pattern_lo & bit_mux) > 0
            {
                p0_pixel = 1;
            }

            let mut p1_pixel: u8 = 0;
            if (self.bg_shifter_info.pattern_hi & bit_mux) > 0
            {
                p1_pixel = 1;
            }

            bg_pixel = (p1_pixel << 1) | p0_pixel;

            // Do the same for the palette
            let mut p0_palette: u8 = 0;
            if (self.bg_shifter_info.attrib_lo & bit_mux) > 0
            {
                p0_palette = 1;
            }

            let mut p1_palette: u8 = 0;
            if (self.bg_shifter_info.attrib_hi & bit_mux) > 0
            {
                p1_palette = 1;
            }

            bg_palette = (p1_palette << 1) | p0_palette;
        }

        let mut fg_pixel: u8 = 0;
        let mut fg_palette: u8 = 0;
        let mut fg_priority: bool = false;

        if self.mask.render_sprites()
        {
            self.sprite_zero_being_rendered = false;

            for i in 0..self.sprite_count
            {
                let index = i as usize;

                if self.sprite_scanline[index].x == 0
                {
                    let fg_pixel_lo = (self.fg_shifter_info.pattern_lo[index] & 0x80) > 0;
                    let fg_pixel_hi = (self.fg_shifter_info.pattern_hi[index] & 0x80) > 0;
                    fg_pixel = ((fg_pixel_hi as u8) << 1) | fg_pixel_lo as u8;

                    fg_palette = (self.sprite_scanline[index].attribute & 0x03) + 0x04;
                    fg_priority = (self.sprite_scanline[index].attribute & 0x20) == 0;

                    if fg_pixel != 0
                    {
                        if index == 0
                        {
                            self.sprite_zero_being_rendered = true;
                        }

                        // We found a visible sprite that has the highest possible priority
                        break;
                    }
                }
            }
        }

        let pixel: u8;
        let palette: u8;

        if bg_pixel == 0 && fg_pixel == 0
        {
            // Both are transparent, draw 'background' color
            pixel = 0x0;
            palette = 0x0;
        }
        else if bg_pixel == 0 && fg_pixel > 0
        {
            // Foreground wins
            pixel = fg_pixel;
            palette = fg_palette;
        }
        else if bg_pixel > 0 && fg_pixel == 0
        {
            // Background
            pixel = bg_pixel;
            palette = bg_palette;
        }
        else
        {
            // Both are visible, need to consult priority
            if fg_priority
            {
                pixel = fg_pixel;
                palette = fg_palette;
            }
            else
            {
                pixel = bg_pixel;
                palette = bg_palette;
            }

            if self.sprite_zero_hit_possible && self.sprite_zero_being_rendered
            {
                if self.mask.render_background() && self.mask.render_sprites()
                {
                    if !(self.mask.render_background_left() | self.mask.render_sprites_left())
                    {
                        if self.cycle >= 9 && self.cycle < 258
                        {
                            self.status.set_sprite_zero_hit(true);
                        }
                    }
                    else
                    {
                        if self.cycle >= 1 && self.cycle < 258
                        {
                            self.status.set_sprite_zero_hit(true);
                        }                        
                    }
                }
            }
        }

        let color = self.get_color_from_palette_ram(palette, pixel);
        self.renderer.set_pixel_to_color(Surface::Screen, 0, color, self.scan_line, self.cycle - 1);

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
const SCREEN_ROWS: usize = 240;
const SCREEN_COLS: usize = 256;
const PATTERN_ROWS: usize = 128;
const PATTERN_COLS: usize = 128;

enum Surface
{
    Screen,
    Pattern
}

pub struct Ppu2c02Renderer
{
    pal_colors: Box<[graphics::Color; 0x40]>,
    screen_pixels: Box<[u8; SCREEN_ROWS * SCREEN_COLS * PIXEL_DEPTH]>,
    pattern_table: [Box<[u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH]>; 2],
}

impl Ppu2c02Renderer
{
    pub fn new() -> Self
    {
        let ret = Ppu2c02Renderer
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
            pattern_table: [Box::new([0u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH]), Box::new([0u8; PATTERN_ROWS * PATTERN_COLS * PIXEL_DEPTH])]
        };

        return ret;

    }

    pub fn render(&mut self, ctx: &mut Context, canvas: &mut ggez::graphics::Canvas)
    {
        canvas.set_sampler(Sampler::from(graphics::FilterMode::Nearest));

        let screen_image = graphics::Image::from_pixels(ctx, self.screen_pixels.as_slice(), ImageFormat::Rgba8UnormSrgb, SCREEN_COLS as u32, SCREEN_ROWS as u32);
        let pattern0_image = graphics::Image::from_pixels(ctx, self.pattern_table[0].as_slice(),
            ImageFormat::Rgba8UnormSrgb, PATTERN_ROWS as u32, PATTERN_COLS as u32);
        let pattern1_image = graphics::Image::from_pixels(ctx, self.pattern_table[1].as_slice(),
            ImageFormat::Rgba8UnormSrgb, PATTERN_ROWS as u32, PATTERN_COLS as u32);

        let screen_params = graphics::DrawParam::new()
            .dest(Vec2::new(0.0, 0.0))
            .scale(Vec2::new(2.0, 2.0));
        
        screen_image.draw(canvas, screen_params);

        let pattern0_params = graphics::DrawParam::new()
            .dest(Vec2::new(525.0, 500.0))
            .scale(Vec2::new(2.0, 2.0));
        pattern0_image.draw(canvas, pattern0_params);

        let pattern1_params = graphics::DrawParam::new()
            .dest(Vec2::new(800.0, 500.0))
            .scale(Vec2::new(2.0, 2.0));
        pattern1_image.draw(canvas, pattern1_params);
    }

    // Row = scanline
    // Col = cycle
    fn set_pixel_to_color(&mut self, surface_id: Surface, surface_index: usize, color: graphics::Color, row: i32, col: i32)
    {
        match surface_id
        {
            Surface::Screen => 
            {
                if row < 0 || row >= SCREEN_ROWS as i32 || col < 0 || col >= SCREEN_COLS as i32
                {
                    // Do nothing
                    return;
                }

                let start_index = (row as usize * SCREEN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH);
                self.screen_pixels[start_index + 0] = color.to_rgba().0;
                self.screen_pixels[start_index + 1] = color.to_rgba().1;
                self.screen_pixels[start_index + 2] = color.to_rgba().2;
                self.screen_pixels[start_index + 3] = 255; // No alpha blending, always opaque
            },

            Surface::Pattern => 
            {
                if row < 0 || row >= PATTERN_ROWS as i32 || col < 0 || col >= PATTERN_COLS as i32
                {
                    // Do nothing
                    return;
                }

                let start_index = (row as usize * PATTERN_COLS * PIXEL_DEPTH) + (col as usize * PIXEL_DEPTH);
                self.pattern_table[surface_index][start_index + 0] = color.to_rgba().0;
                self.pattern_table[surface_index][start_index + 1] = color.to_rgba().1;
                self.pattern_table[surface_index][start_index + 2] = color.to_rgba().2;
                self.pattern_table[surface_index][start_index + 3] = 255; // No alpha blending, always opaque
            }
        };

        
    }


}