use crate::bus;

pub struct GPU {
    x_size: u32,
    y_size: u32,
    clock_cycles: u16,
    current_line: u8,
    mode: u8,
}

impl GPU {
    const MAX_LINE: u8 = 143; // 144 lines in total
    const LINE_VBLANK_END: u8 = 153;

    const VRAM_TILE_DATA_BEGIN: u16 = 0x8000;
    const VRAM_TILE_DATA_END: u16 = 0x97FF;
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
        GPU {x_size: x_s, y_size: y_s, clock_cycles: 0, current_line: 0, mode: 2}
    }

    pub fn get_y_coord(&self) -> u8 {
        self.current_line
    }

    pub fn tick(&mut self, bus: &mut bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
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
    }

    fn write_scanline(&self, bus: &bus::Bus, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        for line in 0..19 {

            // printing one line of sprite data
            for sprite in 0..20 { // 20 sprites per line
                let raw_sprite: Vec<u8> = (0..16).map(|i| bus.fetch_byte((line * 20 + sprite) * 16 + GPU::VRAM_TILE_DATA_BEGIN + i)).collect();
                for pair in (0..16).step_by(2) { // each sprite is 2 byte long
                    for j in 0..8 { // each bit in a byte is a pixel
                        let shift = 0b10000000 >> j; // scanning from left to right
                        let raw_1: u8 = (raw_sprite[pair] & shift) >> (7 - j);
                        let raw_2: u8 = (raw_sprite[pair + 1] & shift) >> (7 - j);
                        let shade: u8 = raw_1 + (raw_2 << 1);
                        match shade {
                            3 => canvas.set_draw_color(sdl2::pixels::Color::BLACK),
                            2 => canvas.set_draw_color(sdl2::pixels::Color::from((96, 96, 96))),
                            1 => canvas.set_draw_color(sdl2::pixels::Color::from((192, 192, 192))),
                            _ => canvas.set_draw_color(sdl2::pixels::Color::WHITE),
                        };
                        if shade != 0 {
                            let pos_x: i32 = (sprite * 8 + j) as i32;
                            let pos_y: i32 = (line * 8 + ((pair as u16) / 2)) as i32;
                            canvas.draw_point(sdl2::rect::Point::new(pos_x, pos_y)).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn render_canvas(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.present();
        canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        canvas.clear();
    }
}