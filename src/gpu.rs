use minifb::Window;

use crate::bus::Bus;
use crate::canvas::Canvas;
use crate::color::Color;

enum GPUMode {
    HBlank,
    VBlank,
    SearchingOAM,
    SearchingVRAM,
}

impl GPUMode {
    pub fn as_u8(&self) -> u8 {
        match self {
            GPUMode::HBlank => 0,
            GPUMode::VBlank => 1,
            GPUMode::SearchingOAM => 3,
            GPUMode::SearchingVRAM => 4,
        }
    }
}

enum WindowTileMapArea {
    Area9800,
    Area9C00,
}

enum BGWindowTileDataArea {
    Area9000, // from -127 to 128, 0x9000 is pattern 0 but tileset starts at 0x8800
    Area8000,
}

impl BGWindowTileDataArea {
    pub fn address(&self) -> u16 {
        match self {
            BGWindowTileDataArea::Area8000 => 0x8000,
            BGWindowTileDataArea::Area9000 => 0x9000,
        }
    }
}

enum BGTileMapArea {
    Area9800,
    Area9C00,
}

enum OBJSize {
    Size8x8,
    Sixe8x16,
}

struct ControlRegister {
    display_enabled: bool,
    window_tile_map_area: WindowTileMapArea,
    window_enabled: bool,
    bg_window_tile_data_area: BGWindowTileDataArea,
    bg_tile_map_area: BGTileMapArea,
    obj_size: OBJSize,
    obj_enabled: bool,
    bg_window_enable_priority: bool,
}

impl ControlRegister {
    const ADDRESS: u16 = 0xFF40;

    pub fn fetch(bus: &Bus) -> Self {
        let raw = bus.fetch_byte(ControlRegister::ADDRESS);

        ControlRegister {
            display_enabled: !((raw & 0b10000000) == 0),
            window_tile_map_area: match raw & 0b1000000 {
                0 => WindowTileMapArea::Area9800,
                _ => WindowTileMapArea::Area9C00,
            },
            window_enabled: !((raw & 0b100000) == 0),
            bg_window_tile_data_area: match raw & 0b10000 {
                0 => BGWindowTileDataArea::Area9000,
                _ => BGWindowTileDataArea::Area8000,
            },
            bg_tile_map_area: match raw & 0b1000 {
                0 => BGTileMapArea::Area9800,
                _ => BGTileMapArea::Area9C00,
            },
            obj_size: match raw & 0b100 {
                0 => OBJSize::Size8x8,
                _ => OBJSize::Sixe8x16,
            },
            obj_enabled: !((raw & 0b10) == 0),
            bg_window_enable_priority: !((raw & 0b1) == 0),
        }
    }
}

pub struct GPU {
    clock_cycles: u16,
    current_line: u8,
    mode: GPUMode,
    stopped: bool,
}

impl GPU {
    const SCREEN_WIDTH: u8 = 160;
    const MAX_LINE: u8 = 143; // 144 lines in total
    const LINE_VBLANK_END: u8 = 153;

    const TILESET_1: u16 = 0x8000;
    const BG_MAP_1: u16 = 0x9800;
    //const BG_MAP_2: u16 = 0x9C00;
    const OAM: u16 = 0xFE00;
    const BG_PALETTE: u16 = 0xFF47;
    const STATUS_REGISTER: u16 = 0xFF41;
    const SCROLL_Y: u16 = 0xFF42;
    const SCROLL_X: u16 = 0xFF43;
    const Y_COORDINATE: u16 = 0xFF44;
    //const Y_COMPARE: u16 = 0xFF45;
    const DMA_TRANSFER_REGISTER: u16 = 0xFF46;

    const OAM_ACCESS_SCANLINE_CLOCKS: u16 = 80;
    const VRAM_ACCESS_SCANLINE_CLOCKS: u16 = 172;
    const HORIZONTAL_BLANK_CLOCKS: u16 = 204;
    const VERTICAL_BLANCK_LINE_CLOCKS: u16 = 456; // single line of vblank ; 10 lines total

    pub fn new() -> GPU {
        GPU {
            clock_cycles: 0,
            current_line: 0,
            mode: GPUMode::SearchingOAM,
            stopped: false,
        }
    }

    pub fn tick(&mut self, bus: &mut Bus, canvas: &mut Canvas, window: &mut Window) {
        let control_register = ControlRegister::fetch(bus);
        if control_register.display_enabled {
            if self.stopped == false {
                canvas.set_draw_color(Color::WHITE);
                canvas.fill_with_color();
                canvas
                    .update_window(window)
                    .expect("Couldn't update render window");

                self.current_line = 0;
                self.clock_cycles = 0;
                self.mode = GPUMode::HBlank;
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
            GPUMode::HBlank => {
                if self.clock_cycles == GPU::HORIZONTAL_BLANK_CLOCKS {
                    // hblank ends
                    self.clock_cycles = 0;
                    self.current_line += 1;
                    if self.current_line == GPU::MAX_LINE {
                        // beginning hblank of last line => vblank
                        self.mode = GPUMode::VBlank;
                        let requested = bus.fetch_byte(0xFF0F);
                        bus.set_byte(0xFF0F, requested | 1);
                        // eventually render canvas here
                        self.render_canvas(canvas, window);
                    } else {
                        self.mode = GPUMode::SearchingOAM; // hblank over, start scanning again
                    }
                }
            }
            GPUMode::VBlank => {
                if self.clock_cycles == GPU::VERTICAL_BLANCK_LINE_CLOCKS {
                    // current vblank line ends
                    self.clock_cycles = 0;
                    self.current_line += 1;
                    if self.current_line > GPU::LINE_VBLANK_END {
                        // ending vblank, resume scanning
                        self.mode = GPUMode::SearchingOAM;
                        self.current_line = 0;
                        // lock oam
                    }
                }
            }
            GPUMode::SearchingOAM => {
                if self.clock_cycles == GPU::OAM_ACCESS_SCANLINE_CLOCKS {
                    // first part of scanning
                    self.clock_cycles = 0;
                    // unlock oam
                    // lock vram
                    self.mode = GPUMode::SearchingVRAM;
                }
            }
            GPUMode::SearchingVRAM => {
                if self.clock_cycles == GPU::VRAM_ACCESS_SCANLINE_CLOCKS {
                    // horizontal scanning ends
                    self.clock_cycles = 0;
                    self.mode = GPUMode::HBlank;
                    // write scanline to canvas
                    self.write_scanline(bus, canvas);
                }
            }
        }

        // update io ports
        bus.set_byte(GPU::Y_COORDINATE, self.current_line);
        bus.set_byte(GPU::STATUS_REGISTER, self.mode.as_u8());
    }

    fn choose_color_from_palette(&self, bus: &Bus, canvas: &mut Canvas, color_nb: u8) {
        let palette = bus.fetch_byte(GPU::BG_PALETTE);
        let shade = match color_nb {
            3 => (palette & 0b11000000) >> 6,
            2 => (palette & 0b110000) >> 4,
            1 => (palette & 0b1100) >> 2,
            0 => palette & 0b11,
            _ => panic!("Palette color_nb {} not valid !", color_nb),
        };

        match shade {
            3 => canvas.set_draw_color(Color::BLACK),
            2 => canvas.set_draw_color(Color::DARK_GRAY),
            1 => canvas.set_draw_color(Color::LIGHT_GRAY),
            0 => canvas.set_draw_color(Color::WHITE),
            _ => panic!("Shade {} not valid!", shade),
        }
    }

    fn render_background_line(&self, bus: &Bus, canvas: &mut Canvas) {
        let tileset = ControlRegister::fetch(bus).bg_window_tile_data_area;
        let tileset_address = tileset.address();
        //let tileset = bus.fetch_byte(GPU::CONTROL_REGISTER) & 0b10000;
        let scroll_x = bus.fetch_byte(GPU::SCROLL_X);
        let scroll_y = bus.fetch_byte(GPU::SCROLL_Y);

        let nb_line = (self.current_line.wrapping_add(scroll_y) / 8) % 32; // ith line of sprites

        for i in 0..GPU::SCREEN_WIDTH {
            let index = ((i + scroll_x) / 8) as u16;
            let nb_sprite = bus.fetch_byte(GPU::BG_MAP_1 + ((nb_line as u16) * 32) + index);
            let pos_x_in_sprite = ((i as u8) + scroll_x) % 8;
            let pos_y_in_sprite = self.current_line.wrapping_add(scroll_y) % 8;

            // drawing the sprite
            let shift = 0b10000000 >> pos_x_in_sprite;
            let shade: u8 = match tileset {
                BGWindowTileDataArea::Area9000 => {
                    let relative_address_offset = 16 * (nb_sprite as i16);
                    let raw_1 = bus.fetch_byte(
                        ((tileset_address as i16)
                            + relative_address_offset
                            + ((pos_y_in_sprite as i16) * 2)) as u16,
                    );
                    let raw_1 = (raw_1 & shift) >> (7 - pos_x_in_sprite);
                    let raw_2 = bus.fetch_byte(
                        ((tileset_address as i16)
                            + relative_address_offset
                            + ((pos_y_in_sprite as i16) * 2 + 1)) as u16,
                    );
                    let raw_2 = (raw_2 & shift) >> (7 - pos_x_in_sprite);
                    raw_1 + (raw_2 << 1)
                }
                BGWindowTileDataArea::Area8000 => {
                    let address_offset = 16 * (nb_sprite as u16);
                    let raw_1 = bus.fetch_byte(
                        tileset_address + address_offset + ((pos_y_in_sprite as u16) * 2),
                    );
                    let raw_1 = (raw_1 & shift) >> (7 - pos_x_in_sprite);
                    let raw_2 = bus.fetch_byte(
                        tileset_address + address_offset + ((pos_y_in_sprite as u16) * 2 + 1),
                    );
                    let raw_2 = (raw_2 & shift) >> (7 - pos_x_in_sprite);
                    raw_1 + (raw_2 << 1)
                }
            };

            self.choose_color_from_palette(bus, canvas, shade);
            canvas
                .set_pixel(i as usize, self.current_line as usize)
                .expect("Couldn't set pixel in canvas");
        }
    }

    fn check_8x8_sprite_rendering(&self, x: i32, y: i32) -> bool {
        ((self.current_line as u16 as i32) >= y && (self.current_line as u16 as i32) <= y + 7)
            && (x > 0 && x < 168)
    }

    fn render_sprite_line(&self, bus: &Bus, canvas: &mut Canvas) {
        let sprite_size = ControlRegister::fetch(bus).obj_size;
        for i in 0..=40 {
            let base_address = GPU::OAM + i * 4;
            let y_pos = (bus.fetch_byte(base_address) as u16 as i32) - 16;
            let x_pos = (bus.fetch_byte(base_address + 1) as u16 as i32) - 8;
            let tile_nb = bus.fetch_byte(base_address + 2);
            let flags = bus.fetch_byte(base_address + 3);

            match sprite_size {
                OBJSize::Size8x8 => {
                    if self.check_8x8_sprite_rendering(x_pos, y_pos) == false {
                        continue;
                    }
                }
                OBJSize::Sixe8x16 => panic!("8x16 sprites are not implemented yet!"),
            }
            // sprite is valid to render
            let mut sprite: Vec<u8> = (0..16)
                .map(|k| bus.fetch_byte(GPU::TILESET_1 + 16 * (tile_nb as u16) + k))
                .collect();
            if flags & 0b1000000 == 1 {
                // Y flip
                sprite.reverse();
            }
            // get Y line to render
            let row_index = ((self.current_line as u16 as i32) - y_pos) as usize;
            let row_1 = sprite[2 * row_index];
            let row_2 = sprite[2 * row_index + 1];

            let x_flip = flags & 0b100000;

            for i in 0..8 {
                let (shade_1, shade_2) = if x_flip == 1 {
                    (
                        (row_1 & (0b10000000 >> (7 - i))) >> (i),
                        (row_2 & (0b10000000 >> (7 - i))) >> (i),
                    )
                } else {
                    (
                        (row_1 & (0b10000000 >> i)) >> (7 - i),
                        (row_2 & (0b10000000 >> i)) >> (7 - i),
                    )
                };

                self.choose_color_from_palette(bus, canvas, shade_1 + (shade_2 << 1));
                canvas
                    .set_pixel((x_pos + i) as usize, self.current_line as usize)
                    .expect("Couldn't set pixel in canvas");
            }
        }
    }

    fn write_scanline(&self, bus: &Bus, canvas: &mut Canvas) {
        let control_register = ControlRegister::fetch(bus);
        if control_register.bg_window_enable_priority {
            self.render_background_line(bus, canvas);
        }
        if control_register.obj_enabled {
            self.render_sprite_line(bus, canvas);
        }
    }

    fn render_canvas(&self, canvas: &mut Canvas, window: &mut Window) {
        canvas
            .update_window(window)
            .expect("Couldn't update window");
    }
}
