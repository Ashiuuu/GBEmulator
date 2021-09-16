use crate::bus;

pub struct GPU {
    x_size: u32,
    y_size: u32,
    clock_cycles: u16,
    current_line: u8,
    mode: u8,
    stopped: bool,
}

impl GPU {
    const MAX_LINE: u8 = 143; // 144 lines in total
    const LINE_VBLANK_END: u8 = 153;

    const TILESET_1: u16 = 0x8000;
    const TILESET_2: u16 = 0x9000; // from -127 to 128, 0x9000 is pattern 0 but tileset starts at 0x8800
    const BG_MAP_1: u16 = 0x9800;
    const BG_MAP_2: u16 = 0x9C00;
    const BG_PALETTE: u16 = 0xFF47;
    const CONTROL_REGISTER: u16 = 0xFF40;
    const STATUS_REGISTER: u16 = 0xFF41;
    const SCROLL_Y: u16 = 0xFF42;
    const SCROLL_X: u16 = 0xFF43;
    const Y_COORDINATE: u16 = 0xFF44;
    const Y_COMPARE: u16 = 0xFF45;
    const DMA_TRANSFER_REGISTER: u16 = 0xFF46;

    const OAM_ACCESS_SCANLINE_CLOCKS: u16 = 80;
    const VRAM_ACCESS_SCANLINE_CLOCKS: u16 = 172;
    const HORIZONTAL_BLANK_CLOCKS: u16 = 204;
    const VERTICAL_BLANCK_LINE_CLOCKS: u16 = 456; // single line of vlank ; 10 lines total

    pub fn new_gpu(x_s: u32, y_s: u32) -> GPU {
        GPU {x_size: x_s, y_size: y_s, clock_cycles: 0, current_line: 0, mode: 2, stopped: false,}
    }

    pub fn get_y_coord(&self) -> u8 {
        self.current_line
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
            0 => canvas.set_draw_color(sdl2::pixels::Color::WHITE),
            _ => panic!("Shade {} not valid!", shade),
        };
    }

    fn render_background_line(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let tileset = bus.fetch_byte(GPU::CONTROL_REGISTER) & 0b10000;
        let scroll_x = bus.fetch_byte(GPU::SCROLL_X);
        let scroll_y = bus.fetch_byte(GPU::SCROLL_Y);

        let nb_line = (self.current_line.wrapping_add(scroll_y) / 8) % 32; // ith line of sprites
        let sprites: Vec<u8> = (0..32).map(|i| bus.fetch_byte(GPU::BG_MAP_1 + ((nb_line as u16) * 32) + i)).collect(); // sprite line

        for i in 0..self.x_size {
            let nb_sprite = sprites[(((i as u8) + scroll_x) / 8) as usize];
            let address_offset = 16 * (nb_sprite as u16);
            let relative_address_offset = 16 * (nb_sprite as i16);
            let sprite:Vec<u8> = if tileset == 0 {
                (0..16).map(|k| bus.fetch_byte(((GPU::TILESET_2 as i16) + relative_address_offset + k) as u16)).collect()
            } else {
                (0..16).map(|k| bus.fetch_byte(GPU::TILESET_1 + address_offset + k)).collect()
            };
            let pos_x_in_sprite = ((i as u8) + scroll_x) % 8;
            let pos_y_in_sprite = self.current_line.wrapping_add(scroll_y) % 8;

            // drawing the sprite
            let shift = 0b10000000 >> pos_x_in_sprite;
            let raw_1 = (sprite[(pos_y_in_sprite * 2) as usize] & shift) >> (7 - pos_x_in_sprite);
            let raw_2 = (sprite[(pos_y_in_sprite * 2 + 1) as usize] & shift) >> (7 - pos_x_in_sprite);
            let shade = raw_1 + (raw_2 << 1);

            self.choose_color_from_palette(bus, canvas, shade);
            canvas.draw_point(sdl2::rect::Point::new(i as i32, self.current_line as i32)).unwrap();
        }
    }

    fn write_scanline(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let control_reg = bus.fetch_byte(GPU::CONTROL_REGISTER);
        let bg_display_flag = control_reg & 1;
        if bg_display_flag != 0 {
            self.render_background_line(bus, canvas);
        }
    }

    fn render_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.present();
        canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        canvas.clear();
    }
}