pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    pub const LIGHT_GRAY: Color = Color {
        red: 192,
        green: 192,
        blue: 192,
    };
    pub const DARK_GRAY: Color = Color {
        red: 96,
        green: 96,
        blue: 96,
    };
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    #[allow(dead_code)]
    pub fn from(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn as_u32(&self) -> u32 {
        (self.blue as u32) + ((self.green as u32) << 8) + ((self.red as u32) << 16)
    }
}
