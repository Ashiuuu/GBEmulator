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
