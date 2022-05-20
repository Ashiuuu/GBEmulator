pub struct Register {
    pub low: u8,
    pub high: u8,
}

#[derive(Clone, Copy)]
pub enum RegisterPart {
    Low,
    High,
    Both,
}

impl Register {
    pub fn new() -> Self {
        Register { low: 0, high: 0 }
    }

    pub fn get_part(&self, part: RegisterPart) -> u8 {
        match part {
            RegisterPart::Low => self.low,
            RegisterPart::High => self.high,
            RegisterPart::Both => {
                panic!("Got RegisterPart::Both in Register.get_part, which shouldn't happen")
            }
        }
    }

    pub fn set_part(&mut self, part: RegisterPart, data: u8) {
        match part {
            RegisterPart::Low => self.low = data,
            RegisterPart::High => self.high = data,
            RegisterPart::Both => {
                panic!("Got RegisterPart::Both in Register.set_part, which shouldn't happen")
            }
        }
    }

    pub fn get_combined(&self) -> u16 {
        ((self.high as u16) << 8) + (self.low as u16)
    }

    pub fn set_word(&mut self, data: u16) {
        self.low = (data & 0xFF) as u8;
        self.high = ((data & 0xFF00) >> 8) as u8;
    }

    pub fn _print(&self) -> String {
        format!(
            "{:#04x}:{:#04x}     {:#06x}",
            self.high,
            self.low,
            self.get_combined()
        )
    }
}

pub struct AFRegister {
    pub a: u8,
    pub flags: FlagRegister,
}

impl AFRegister {
    pub fn new() -> Self {
        AFRegister {
            a: 0,
            flags: FlagRegister::new(),
        }
    }

    fn set_low(&mut self, data: u8) {
        self.flags.from_byte(data);
    }

    fn set_high(&mut self, data: u8) {
        self.a = data;
    }

    pub fn set_word(&mut self, data: u16) {
        self.set_low((data & 0xFF) as u8);
        self.set_high(((data & 0xFF00) >> 8) as u8);
    }

    pub fn get_part(&self, part: RegisterPart) -> u8 {
        match part {
            RegisterPart::Low => self.flags.to_byte(),
            RegisterPart::High => self.a,
            RegisterPart::Both => {
                panic!("Got RegisterPart::Both in AFRegister.set_part, which shouldn't happen")
            }
        }
    }

    pub fn set_part(&mut self, part: RegisterPart, data: u8) {
        match part {
            RegisterPart::Low => self.flags.from_byte(data),
            RegisterPart::High => self.a = data,
            RegisterPart::Both => {
                panic!("Got RegisterPart::Both in AFRegister.set_part, which shouldn't happen")
            }
        }
    }

    pub fn get_combined(&self) -> u16 {
        ((self.a as u16) << 8) + (self.flags.to_byte() as u16)
    }
}

pub struct FlagRegister {
    zero_flag: bool,
    carry_flag: bool,
    n_flag: bool,
    h_flag: bool,
}

impl FlagRegister {
    pub fn new() -> Self {
        FlagRegister {
            zero_flag: false,
            carry_flag: false,
            n_flag: false,
            h_flag: false,
        }
    }

    pub fn get(&self, c: char) -> bool {
        match c {
            'z' => self.zero_flag,
            'n' => self.n_flag,
            'h' => self.h_flag,
            'c' => self.carry_flag,
            _ => {
                panic!("Invalid flag character : {}", c);
            }
        }
    }

    pub fn set(&mut self, c: char) {
        match c {
            'z' => self.zero_flag = true,
            'n' => self.n_flag = true,
            'h' => self.h_flag = true,
            'c' => self.carry_flag = true,
            _ => panic!("Invalid flag character : {}", c),
        };
    }

    pub fn clear(&mut self, c: char) {
        match c {
            'z' => self.zero_flag = false,
            'n' => self.n_flag = false,
            'h' => self.h_flag = false,
            'c' => self.carry_flag = false,
            _ => panic!("Invalid flag character : {}", c),
        };
    }

    pub fn update(&mut self, c: char, val: bool) {
        if val == true {
            self.set(c);
        } else {
            self.clear(c);
        }
    }

    pub fn from_byte(&mut self, data: u8) {
        let masked = (data & 0b11110000) >> 4;
        self.zero_flag = masked & 0b1000 != 0;
        self.n_flag = masked & 0b100 != 0;
        self.h_flag = masked & 0b10 != 0;
        self.carry_flag = masked & 0b1 != 0;
    }

    pub fn to_byte(&self) -> u8 {
        let mut result = 0;

        if self.zero_flag == true {
            result |= 0b10000000;
        };
        if self.n_flag == true {
            result |= 0b1000000;
        }
        if self.h_flag == true {
            result |= 0b100000;
        }
        if self.carry_flag == true {
            result |= 0b10000;
        }

        result
    }
}
