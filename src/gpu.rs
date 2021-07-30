use crate::bus;

pub struct GPU {
    clock_cycles: u16,
    current_line: u8,
    mode: u8,
}

impl GPU {
    const MAX_LINE: u8 = 143; // 144 lines in total
    const LINE_VBLANK_END: u8 = 153;

    const CONTROL_REGISTER: u16 = 0xFF40;
    const STATUS_REGISTER: u16 = 0xFF41;
    const SCROLL_Y: u16 = 0xFF42;
    const SCROLL_X: u16 = 0xFF43;
    const Y_COORDINATE: u16 = 0xFF44;
    const Y_COMPARE: u16 = 0xFF45;

    const OAM_ACCESS_SCANLINE_CLOCKS: u16 = 80;
    const VRAM_ACCESS_SCANLINE_CLOCKS: u16 = 172;
    const HORIZONTAL_BLANK_CLOCKS: u16 = 204;
    const VERTICAL_BLANCK_LINE_CLOCKS: u16 = 456; // single line of vlank ; 10 lines total

    pub fn new_gpu() -> GPU {
        GPU {clock_cycles: 0, current_line: 0, mode: 2}
    }

    pub fn get_y_coord(&self) -> u8 {
        self.current_line
    }

    pub fn tick(&mut self, bus: &mut bus::Bus) {
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
                }
            },
            _ => panic!("Unknown GPU mode, aborting")
        }

        // update io ports
        bus.set_byte(GPU::Y_COORDINATE, self.current_line);
    }    
}