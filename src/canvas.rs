use std::ops::Deref;

use crate::color::Color;

use minifb::Window;

pub struct Canvas {
    buffer: Vec<u32>,
    x_size: usize,
    y_size: usize,
    current_draw_color: Color,
}

#[derive(Debug)]
pub enum CanvasFail {
    IndexOutOfBounds,
    WindowUpdate,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buffer: vec![0; width * height],
            x_size: width,
            y_size: height,
            current_draw_color: Color::WHITE,
        }
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.current_draw_color = color;
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) -> Result<(), CanvasFail> {
        if x > self.x_size || y > self.y_size {
            return Err(CanvasFail::IndexOutOfBounds);
        }

        self.buffer[y * self.y_size + x] = self.current_draw_color.as_u32();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<u32, CanvasFail> {
        if x > self.x_size || y > self.y_size {
            return Err(CanvasFail::IndexOutOfBounds);
        }

        Ok(self.buffer[y * self.y_size + x])
    }

    pub fn fill_with_color(&mut self) {
        for i in 0..self.x_size {
            for j in 0..self.y_size {
                self.set_pixel(i, j)
                    .expect("Shouldn't panic, check loop ranges");
            }
        }
    }

    // TODO: remove from there
    pub fn update_window(&self, window: &mut Window) -> Result<(), CanvasFail> {
        match window.update_with_buffer(&self.buffer[..], self.x_size, self.y_size) {
            Ok(_) => Ok(()),
            Err(_) => Err(CanvasFail::WindowUpdate),
        }
    }
}

impl<'a> Deref for Canvas {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}
