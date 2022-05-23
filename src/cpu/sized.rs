#[derive(Clone, Copy)]
pub enum Sized {
    Zero,
    Byte(u8),
    Word(u16),
}

impl Sized {
    pub fn number_of_bytes(&self) -> u16 {
        match self {
            Sized::Zero => 0,
            Sized::Byte(_) => 1,
            Sized::Word(_) => 2,
        }
    }

    pub fn is_value_zero(&self) -> bool {
        match self {
            Sized::Byte(value) => value == &0,
            Sized::Word(value) => value == &0,
            Sized::Zero => panic!("[is_value_zero] Sized::Zero has no internal value!"),
        }
    }

    pub fn check_value_for_half_carry(&self) -> bool {
        match self {
            Sized::Byte(value) => value & 0b1111 == 0,
            Sized::Word(value) => value & 0b11111111 == 0,
            Sized::Zero => {
                panic!("[check_value_for_half_carry] Sized::Zero has no internal value!")
            }
        }
    }
}

impl From<u8> for Sized {
    fn from(data: u8) -> Self {
        Sized::Byte(data)
    }
}

impl From<Sized> for u8 {
    fn from(data: Sized) -> Self {
        match data {
            Sized::Byte(d) => d,
            _ => panic!("Trying to cast non byte Sized to u8"),
        }
    }
}

impl From<u16> for Sized {
    fn from(data: u16) -> Self {
        Sized::Word(data)
    }
}

impl From<Sized> for u16 {
    fn from(data: Sized) -> Self {
        match data {
            Sized::Word(d) => d,
            _ => panic!("Trying to cast non word Sized to u16"),
        }
    }
}
