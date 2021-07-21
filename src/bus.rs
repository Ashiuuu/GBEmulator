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

struct WorkingRam {
    data: Box<[u8]>,
    base: u16,
}

impl WorkingRam {
    fn from_size(size: usize, b: u16) -> WorkingRam {
        WorkingRam {
            data: vec![0; size].into_boxed_slice(),
            base: b,
        }
    }

    pub fn get_byte(&self, address: u16) -> u8 {
        let real_address = address - self.base;
        self.data[real_address as usize]
    }

    fn set_byte(&mut self, address: u16, data: u8) {
        let real_address = address - self.base;
        self.data[real_address as usize] = data;
    }
}

pub struct Bus {
    rom: ROM,
    vram: WorkingRam,
    external_ram: WorkingRam,
    wram1: WorkingRam,
    wram2: WorkingRam,
    oam: WorkingRam,
    io: WorkingRam,
    high_ram: WorkingRam,
    interrupt_enable_register: u8,
}

impl Bus {
    pub fn new_bus(filename: &String) -> Bus {
        Bus {
            rom: ROM::from_file(filename),
            vram: WorkingRam::from_size(8192, 0x8000),
            external_ram: WorkingRam::from_size(8192, 0xA000),
            wram1: WorkingRam::from_size(4096, 0xC000),
            wram2: WorkingRam::from_size(4096, 0xD000),
            oam: WorkingRam::from_size(159, 0xFE00),
            io: WorkingRam::from_size(127, 0xFF00),
            high_ram: WorkingRam::from_size(126, 0xFF80),
            interrupt_enable_register: 0,
        }
    }

    pub fn fetch_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.rom.get_byte(address),
            0x4000..=0x7FFF => 0, // ROM bank 1..N in cartridge
            0x8000..=0x9FFF => self.vram.get_byte(address),
            0xA000..=0xBFFF => self.external_ram.get_byte(address),
            0xC000..=0xCFFF => self.wram1.get_byte(address),
            0xD000..=0xDFFF => self.wram2.get_byte(address),
            0xE000..=0xFDFF => self.wram1.get_byte(address),
            0xFE00..=0xFE9F => self.oam.get_byte(address),
            0xFEA0..=0xFEFF => panic!("Address {:#x} is not usable !", address),
            0xFF00..=0xFF7F => self.io.get_byte(address),
            0xFF80..=0xFFFE => self.high_ram.get_byte(address),
            0xFFFF => self.interrupt_enable_register,
        }
    }

    pub fn set_byte(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x3FFF => panic!("Cannot set byte in ROM !"),
            0x4000..=0x7FFF => panic!("Cannot set byte in ROM ! (2)"), // ROM bank 1..N in cartridge
            0x8000..=0x9FFF => self.vram.set_byte(address, data),
            0xA000..=0xBFFF => self.external_ram.set_byte(address, data),
            0xC000..=0xCFFF => self.wram1.set_byte(address, data),
            0xD000..=0xDFFF => self.wram2.set_byte(address, data),
            0xE000..=0xFDFF => self.wram1.set_byte(address, data),
            0xFE00..=0xFE9F => self.oam.set_byte(address, data),
            0xFEA0..=0xFEFF => panic!("Address {:#x} is not usable !", address),
            0xFF00..=0xFF7F => self.io.set_byte(address, data),
            0xFF80..=0xFFFE => self.high_ram.set_byte(address, data),
            0xFFFF => self.interrupt_enable_register = data,
        }
    }

    pub fn set_word(&mut self, address: u16, data: u16) {
        self.set_byte(address, (data & 0xFF) as u8);
        self.set_byte(address + 1, ((data & 0xFF00) >> 8) as u8);
    }
}
