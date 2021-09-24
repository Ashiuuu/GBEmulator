use crate::bus;

pub struct GPU {
    clock_cycles: u16,
    current_line: u8,
    mode: u8,
    stopped: bool,
}

impl GPU {
    const SCREEN_WIDTH: u8 = 160;
    const MAX_LINE: u8 = 143; // 144 lines in total
    const LINE_VBLANK_END: u8 = 153;

    const TILESET_1: u16 = 0x8000;
    const TILESET_2: u16 = 0x9000; // from -127 to 128, 0x9000 is pattern 0 but tileset starts at 0x8800
    const BG_MAP_1: u16 = 0x9800;
    //const BG_MAP_2: u16 = 0x9C00;
    const OAM: u16 = 0xFE00;
    const BG_PALETTE: u16 = 0xFF47;
    const CONTROL_REGISTER: u16 = 0xFF40;
    const STATUS_REGISTER: u16 = 0xFF41;
    const SCROLL_Y: u16 = 0xFF42;
    const SCROLL_X: u16 = 0xFF43;
    const Y_COORDINATE: u16 = 0xFF44;
    //const Y_COMPARE: u16 = 0xFF45;
    const DMA_TRANSFER_REGISTER: u16 = 0xFF46;

    const OAM_ACCESS_SCANLINE_CLOCKS: u16 = 80;
    const VRAM_ACCESS_SCANLINE_CLOCKS: u16 = 172;
    const HORIZONTAL_BLANK_CLOCKS: u16 = 204;
    const VERTICAL_BLANCK_LINE_CLOCKS: u16 = 456; // single line of vlank ; 10 lines total

    pub fn new_gpu() -> GPU {
        GPU {clock_cycles: 0, current_line: 0, mode: 2, stopped: false,}
    }

    pub fn tick(&mut self, bus: &mut bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let control_reg = bus.fetch_byte(GPU::CONTROL_REGISTER);
        let display_enable = control_reg & 0b10000000;
        if display_enable == 0 {
            if self.stopped == false {
                canvas.set_draw_color(sdl2::pixels::Color::WHITE);
                canvas.clear();
                canvas.present();

                self.current_line = 0;
                self.clock_cycles = 0;
                self.mode = 0;
                self.stopped = true;
            } 
            return;
        }

        // check if DMA transfer was started
        let dma = bus.fetch_byte(GPU::DMA_TRANSFER_REGISTER) as u16;
        if dma != 0 {
            bus.set_byte(GPU::DMA_TRANSFER_REGISTER, 0);
            for i in 0..=0x9F {
                let content = bus.fetch_byte(((dma << 8) + i) as u16);
                bus.set_byte(0xFE00 + i, content);
            }
        }

        self.clock_cycles += 1;
        match self.mode {
            0 => {
                if self.clock_cycles == GPU::HORIZONTAL_BLANK_CLOCKS { // hblank ends
                    self.clock_cycles = 0;
                    self.current_line += 1;
                    if self.current_line == GPU::MAX_LINE { // beginning hblank of last line => vblank
                        self.mode = 1;
                        let requested = bus.fetch_byte(0xFF0F);
                        bus.set_byte(0xFF0F, requested | 1);
                        // eventually render canvas here
                        self.render_canvas(canvas);
                    } else {
                        self.mode = 2; // hblank over, start scanning again
                    }
                }
            },
            1 => {
                if self.clock_cycles == GPU::VERTICAL_BLANCK_LINE_CLOCKS { // current vblank line ends
                    self.clock_cycles = 0;
                    self.current_line += 1;
                    if self.current_line > GPU::LINE_VBLANK_END { // ending vblank, resume scanning
                        self.mode = 2;
                        self.current_line = 0;
                        // lock oam
                    }
                }
            },
            2 => {
                if self.clock_cycles == GPU::OAM_ACCESS_SCANLINE_CLOCKS { // first part of scanning
                    self.clock_cycles = 0;
                    // unlock oam
                    // lock vram
                    self.mode = 3;
                }
            },
            3 => {
                if self.clock_cycles == GPU::VRAM_ACCESS_SCANLINE_CLOCKS { // horizontal scanning ends
                    self.clock_cycles = 0;
                    self.mode = 0;
                    // write scanline to canvas
                    self.write_scanline(bus, canvas);
                }
            },
            _ => panic!("Unknown GPU mode, aborting")
        }

        // update io ports
        bus.set_byte(GPU::Y_COORDINATE, self.current_line);
        bus.set_byte(GPU::STATUS_REGISTER, self.mode);
    }

    fn choose_color_from_palette(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color_nb: u8) {
        let palette = bus.fetch_byte(GPU::BG_PALETTE);
        let shade = match color_nb {
            3 => (palette & 0b11000000) >> 6,
            2 => (palette & 0b110000) >> 4,
            1 => (palette & 0b1100) >> 2,
            0 => palette & 0b11,
            _ => panic!("Palette color_nb {} not valid !", color_nb),
        };

        match shade {
            3 => canvas.set_draw_color(sdl2::pixels::Color::BLACK),
            2 => canvas.set_draw_color(sdl2::pixels::Color::from((96, 96, 96))),
            1 => canvas.set_draw_color(sdl2::pixels::Color::from((192, 192, 192))),
            0 => canvas.set_draw_color(sdl2::pixels::Color::from((255, 255, 255, 0))),
            _ => panic!("Shade {} not valid!", shade),
        };
    }

    fn render_background_line(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let tileset = bus.fetch_byte(GPU::CONTROL_REGISTER) & 0b10000;
        let scroll_x = bus.fetch_byte(GPU::SCROLL_X);
        let scroll_y = bus.fetch_byte(GPU::SCROLL_Y);

        let nb_line = (self.current_line.wrapping_add(scroll_y) / 8) % 32; // ith line of sprites

        for i in 0..GPU::SCREEN_WIDTH {
            let index = (((i as u8) + scroll_x) / 8) as u16;
            let nb_sprite = bus.fetch_byte(GPU::BG_MAP_1 + ((nb_line as u16) * 32) + index);
            let pos_x_in_sprite = ((i as u8) + scroll_x) % 8;
            let pos_y_in_sprite = self.current_line.wrapping_add(scroll_y) % 8;
            
            // drawing the sprite
            let shift = 0b10000000 >> pos_x_in_sprite;
            let shade: u8 = if tileset == 0 {
                let relative_address_offset = 16 * (nb_sprite as i16);
                let raw_1 = bus.fetch_byte(((GPU::TILESET_2 as i16) + relative_address_offset + ((pos_y_in_sprite as i16) * 2)) as u16);
                let raw_1 = (raw_1 & shift) >> (7 - pos_x_in_sprite);
                let raw_2 = bus.fetch_byte(((GPU::TILESET_2 as i16) + relative_address_offset + ((pos_y_in_sprite as i16) * 2 + 1)) as u16);
                let raw_2 = (raw_2 & shift) >> (7 - pos_x_in_sprite);
                raw_1 + (raw_2 << 1)
            } else {
                let address_offset = 16 * (nb_sprite as u16);
                let raw_1 = bus.fetch_byte(GPU::TILESET_1 + address_offset + ((pos_y_in_sprite as u16) * 2));
                let raw_1 = (raw_1 & shift) >> (7 - pos_x_in_sprite);
                let raw_2 = bus.fetch_byte(GPU::TILESET_1 + address_offset + ((pos_y_in_sprite as u16) * 2 + 1));
                let raw_2 = (raw_2 & shift) >> (7 - pos_x_in_sprite);
                raw_1 + (raw_2 << 1)
            };

            self.choose_color_from_palette(bus, canvas, shade);
            canvas.draw_point(sdl2::rect::Point::new(i as i32, self.current_line as i32)).unwrap();
        }
    }

    fn check_8x8_sprite_rendering(&self, x: i32, y: i32) -> bool {
        ((self.current_line as u16 as i32) >= y && (self.current_line as u16 as i32) <= y + 7)
        && (x > 0 && x < 168)
    }

    fn render_sprite_line(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let sprite_size = GPU::CONTROL_REGISTER & 0b100;
        for i in 0..=40 {
            let base_address = GPU::OAM + i * 4;
            let y_pos = (bus.fetch_byte(base_address) as u16 as i32) - 16;
            let x_pos = (bus.fetch_byte(base_address + 1) as u16 as i32) - 8;
            let tile_nb = bus.fetch_byte(base_address + 2);
            let flags = bus.fetch_byte(base_address + 3);

            if sprite_size == 0 { // 8x8 sprites
                if self.check_8x8_sprite_rendering(x_pos, y_pos) == false {
                    continue; // do not render sprite
                }
            } else { // 8x16 sprites
                // TODO
                panic!("8x16 sprites unimplemented yet");
            }
            // sprite is valid to render
            let mut sprite: Vec<u8> = (0..16).map(|k| bus.fetch_byte(GPU::TILESET_1 + 16 * (tile_nb as u16) + k)).collect();
            if flags & 0b1000000 == 1 { // Y flip
                sprite.reverse();
            }
            // get Y line to render
            let row_index = ((self.current_line as u16 as i32) - y_pos) as usize;
            let row_1 = sprite[2 * row_index];
            let row_2 = sprite[2 * row_index + 1];

            let x_flip = flags & 0b100000;

            for i in 0..8 {
                if x_flip == 1 {
                    let shade_1 = (row_1 & (0b10000000 >> (7 - i))) >> (i);
                    let shade_2 = (row_2 & (0b10000000 >> (7 - i))) >> (i);
                    let shade = shade_1 + (shade_2 << 1);
                    self.choose_color_from_palette(bus, canvas, shade);
                    canvas.draw_point(sdl2::rect::Point::new(x_pos + i, self.current_line as i32)).unwrap();
                } else {
                    let shade_1 = (row_1 & (0b10000000 >> i)) >> (7 - i);
                    let shade_2 = (row_2 & (0b10000000 >> i)) >> (7 - i);
                    let shade = shade_1 + (shade_2 << 1);
                    self.choose_color_from_palette(bus, canvas, shade);
                    canvas.draw_point(sdl2::rect::Point::new(x_pos + i, self.current_line as i32)).unwrap();
                }
            }
        }
    }

    fn write_scanline(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let control_reg = bus.fetch_byte(GPU::CONTROL_REGISTER);
        let bg_display_flag = control_reg & 1;
        let sprite_display_flag = control_reg & 0b10;
        if bg_display_flag != 0 {
            self.render_background_line(bus, canvas);
        }
        if sprite_display_flag != 0 {
            self.render_sprite_line(bus, canvas);
        }
    }

    fn render_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.present();
        canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        canvas.clear();
    }
}