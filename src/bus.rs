use std::fs;

struct ROM {
    cartridge: Vec<u8>,
}

impl ROM {
    pub fn from_file(filename: &String) -> ROM {
        ROM {
            cartridge: match fs::read(filename) {
                Err(err) => panic!("Could not read content of {} : {}", filename, err),
                Ok(file) => file,
            },
        }
    }

    fn get_byte(&self, i: u16) -> u8 {
        self.cartridge[i as usize]
    }
}

struct VRAM {
    size: usize,
}

impl VRAM {
    fn from_size(s: usize) -> VRAM {
        VRAM { size: s }
    }

    fn get_byte(&self, _i: usize) -> u8 {
        0
    }

    fn set_byte(&self, _data: u8) {}
}

struct WorkingRam {
    data: Box<[u8]>,
}

impl WorkingRam {
    fn from_size(size: usize) -> WorkingRam {
        WorkingRam {
            data: vec![0; size].into_boxed_slice(),
        }
    }

    pub fn get_byte(&self, i: usize) -> u8 {
        self.data[i]
    }

    fn set_byte(&mut self, i: usize, data: u8) {
        self.data[i] = data;
    }
}

pub struct Bus {
    rom: ROM,
    vram: VRAM,
    external_ram: WorkingRam,
    wram1: WorkingRam,
    wram2: WorkingRam,
}

impl Bus {
    pub fn new_bus(filename: &String) -> Bus {
        Bus {
            rom: ROM::from_file(filename),
            vram: VRAM::from_size(8192),
            external_ram: WorkingRam::from_size(8192),
            wram1: WorkingRam::from_size(4096),
            wram2: WorkingRam::from_size(4096),
        }
    }

    pub fn fetch_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.get_byte(address),
            0x4000..=0x7FFF => 0, // ROM bank 1..N in cartridge
            0x8000..=0x9FFF => self.vram.get_byte(0),
            0xA000..=0xBFFF => self.external_ram.get_byte(0),
            0xC000..=0xCFFF => self.wram1.get_byte(0),
            0xD000..=0xDFFF => self.wram2.get_byte(0),
            0xE000..=0xFFFF => {
                eprintln!("Address {} not yet implented", address);
                0
            }
        }
    }

    pub fn set_byte(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3FFF => println!("Cannot set byte in ROM !"),
            0x4000..=0x7FFF => println!("Cannot set byte in ROM !"), // ROM bank 1..N in cartridge
            0x8000..=0x9FFF => self.vram.set_byte(data),
            0xA000..=0xBFFF => self.external_ram.set_byte(0, data),
            0xC000..=0xCFFF => self.wram1.set_byte(0, data),
            0xD000..=0xDFFF => self.wram2.set_byte(0, data),
            0xE000..=0xFFFF => println!("{} not yet implented", address),
        }
    }

    pub fn set_word(&mut self, address: u16, data: u16) {
        self.set_byte(address, (data & 0xFF) as u8);
        self.set_byte(address + 1, ((data & 0xFF00) >> 8) as u8);
    }
}
