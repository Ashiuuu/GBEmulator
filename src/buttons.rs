use crate::bus::Bus;

use minifb::{Key, Window};

struct SelectedRows {
    pub first: bool,
    pub second: bool,
}

impl SelectedRows {
    pub fn fetch(bus: &Bus) -> SelectedRows {
        let row = (bus.fetch_byte(Buttons::BUTTON_REGISTER_ADDRESS) & 0b110000) >> 4;
        SelectedRows {
            first: row & 1 == 0,
            second: row & 0b10 == 0,
        }
    }
}

pub struct Buttons {
    row_1: u8,
    row_2: u8,
}

impl Buttons {
    const BUTTON_REGISTER_ADDRESS: u16 = 0xFF00;

    pub fn new() -> Buttons {
        Buttons {
            row_1: 0xF,
            row_2: 0xF,
        }
    }

    fn key_down(&mut self, key: Key) {
        match key {
            Key::Down => self.row_1 &= 0b11110111,
            Key::Up => self.row_1 &= 0b11111011,
            Key::Left => self.row_1 &= 0b11111101,
            Key::Right => self.row_1 &= 0b11111110,
            Key::Enter => self.row_2 &= 0b11110111, // Start Button
            Key::Space => self.row_2 &= 0b11111011, // Select Button
            Key::B => self.row_2 &= 0b11111101,
            Key::A => self.row_2 &= 0b11111110,
            _ => (),
        };
    }

    fn key_up(&mut self, key: Key) {
        match key {
            Key::Down => self.row_1 |= 0b1000,
            Key::Up => self.row_1 |= 0b100,
            Key::Left => self.row_1 |= 0b10,
            Key::Right => self.row_1 |= 0b1,
            Key::Enter => self.row_2 |= 0b1000, // Start Button
            Key::Space => self.row_2 |= 0b100,  // Select Button
            Key::B => self.row_2 |= 0b10,
            Key::A => self.row_2 |= 0b1,
            _ => (),
        };
    }

    pub fn update_keys(&mut self, window: &Window) {
        let keys = vec![
            Key::Down,
            Key::Up,
            Key::Left,
            Key::Right,
            Key::Enter,
            Key::Space,
            Key::B,
            Key::A,
        ];

        for key in keys {
            if window.is_key_down(key) {
                self.key_down(key);
            } else {
                self.key_up(key);
            }
        }
    }

    pub fn update_register(&self, bus: &mut Bus) {
        let rows = SelectedRows::fetch(bus);
        if rows.first {
            // 4 upper bits are set to 1 to keep from reading more values
            bus.set_byte(
                Buttons::BUTTON_REGISTER_ADDRESS,
                (self.row_1 & 0xF) | 0b11110000,
            );
        // update register with direction keys values
        } else if rows.second {
            bus.set_byte(
                Buttons::BUTTON_REGISTER_ADDRESS,
                (self.row_2 & 0xF) | 0b11110000,
            );
        }
    }
}
